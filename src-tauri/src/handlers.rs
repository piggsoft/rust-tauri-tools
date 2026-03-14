use crate::models::*;
use crate::db::{get_pool, tags_to_json, DB_POOL};
use chrono::Utc;
use serde_json::json;

// ==================== Task CRUD ====================

#[tauri::command]
pub async fn create_task(task: TaskInput) -> Result<ApiResponse<Task>, String> {
    let pool = get_pool().await?;
    let now = Utc::now().to_rfc3339();

    let tags_json = tags_to_json(task.tags.as_ref()).await;

    let result = sqlx::query_as::<_, Task>(
        r#"
        INSERT INTO tasks (
            title, description, priority, urgency, status,
            tags, start_date, due_date, reminder_time,
            estimated_duration, repeat_pattern, repeat_until,
            created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING *
        "#
    )
    .bind(&task.title)
    .bind(&task.description)
    .bind(&task.priority)
    .bind(&task.urgency)
    .bind("pending")
    .bind(tags_json)
    .bind(&task.start_date)
    .bind(&task.due_date)
    .bind(&task.reminder_time)
    .bind(task.estimated_duration)
    .bind(&task.repeat_pattern)
    .bind(&task.repeat_until)
    .bind(&now)
    .bind(&now)
    .fetch_one(&pool)
    .await
    .map_err(|e| format!("Failed to create task: {}", e))?;

    // Record history
    record_task_history(&pool, result.id, "created", Some(json!({ "title": task.title }).to_string()))
        .await
        .map_err(|e| format!("Failed to record task history: {}", e))?;

    Ok(ApiResponse::success(result))
}

#[tauri::command]
pub async fn update_task(id: i64, task: TaskInput) -> Result<ApiResponse<Task>, String> {
    let pool = get_pool().await?;
    let now = Utc::now().to_rfc3339();

    let tags_json = tags_to_json(task.tags.as_ref()).await;

    // Get old task for history
    let old_task = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| format!("Failed to get task: {}", e))?;

    let changes = if let Some(old) = &old_task {
        let mut changes = serde_json::Map::new();
        if old.title != task.title {
            changes.insert("title".to_string(), json!({ "old": old.title, "new": task.title }));
        }
        changes.insert("updated_at".to_string(), json!(now));
        Some(serde_json::to_string(&changes).unwrap())
    } else {
        None
    };

    let result = sqlx::query_as::<_, Task>(
        r#"
        UPDATE tasks
        SET title = ?, description = ?, priority = ?, urgency = ?,
            tags = ?, start_date = ?, due_date = ?, reminder_time = ?,
            estimated_duration = ?, repeat_pattern = ?, repeat_until = ?,
            updated_at = ?
        WHERE id = ?
        RETURNING *
        "#
    )
    .bind(&task.title)
    .bind(&task.description)
    .bind(&task.priority)
    .bind(&task.urgency)
    .bind(tags_json)
    .bind(&task.start_date)
    .bind(&task.due_date)
    .bind(&task.reminder_time)
    .bind(task.estimated_duration)
    .bind(&task.repeat_pattern)
    .bind(&task.repeat_until)
    .bind(&now)
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|e| format!("Failed to update task: {}", e))?;

    // Record history
    if let Some(changes) = changes {
        record_task_history(&pool, id, "updated", Some(changes))
            .await
            .map_err(|e| format!("Failed to record task history: {}", e))?;
    }

    Ok(ApiResponse::success(result))
}

#[tauri::command]
pub async fn delete_task(id: i64) -> Result<ApiResponse<()>, String> {
    let pool = get_pool().await?;

    sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| format!("Failed to delete task: {}", e))?;

    Ok(ApiResponse::success(()))
}

#[tauri::command]
pub async fn get_task(id: i64) -> Result<ApiResponse<Task>, String> {
    let pool = get_pool().await?;

    let task = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| format!("Failed to get task: {}", e))?
        .ok_or_else(|| "Task not found".to_string())?;

    Ok(ApiResponse::success(task))
}

#[tauri::command]
pub async fn list_tasks(filter: Option<TaskFilter>) -> Result<ApiResponse<Vec<Task>>, String> {
    let pool = get_pool().await?;

    let mut query = String::from("SELECT * FROM tasks WHERE 1=1");
    let mut params: Vec<String> = Vec::new();

    if let Some(f) = filter {
        if let Some(search) = f.search {
            query.push_str(&format!(" AND (title LIKE '%{}%' OR description LIKE '%{}%')", search, search));
        }
        if let Some(tags) = f.tags {
            if !tags.is_empty() {
                for tag in &tags {
                    query.push_str(&format!(" AND tags LIKE '%{}%'", tag));
                }
            }
        }
        if let Some(statuses) = f.status {
            if !statuses.is_empty() {
                let placeholders = statuses.iter().map(|_| "?").collect::<Vec<_>>().join(",");
                query.push_str(&format!(" AND status IN ({})", placeholders));
                params.extend(statuses);
            }
        }
        if let Some(priorities) = f.priority {
            if !priorities.is_empty() {
                let placeholders = priorities.iter().map(|_| "?").collect::<Vec<_>>().join(",");
                query.push_str(&format!(" AND priority IN ({})", placeholders));
                params.extend(priorities);
            }
        }
        if let Some(urgencies) = f.urgency {
            if !urgencies.is_empty() {
                let placeholders = urgencies.iter().map(|_| "?").collect::<Vec<_>>().join(",");
                query.push_str(&format!(" AND urgency IN ({})", placeholders));
                params.extend(urgencies);
            }
        }
        if let Some(date_start) = f.date_start {
            query.push_str(" AND (due_date >= ? OR start_date >= ?)");
            params.push(date_start.clone());
            params.push(date_start);
        }
        if let Some(date_end) = f.date_end {
            query.push_str(" AND (due_date <= ? OR start_date <= ?)");
            params.push(date_end.clone());
            params.push(date_end);
        }
    }

    query.push_str(" ORDER BY due_date ASC, created_at DESC");

    let mut query_builder = sqlx::query_as::<_, Task>(&query);
    for param in params {
        query_builder = query_builder.bind(param);
    }

    let tasks = query_builder.fetch_all(&pool).await
        .map_err(|e| format!("Failed to list tasks: {}", e))?;

    Ok(ApiResponse::success(tasks))
}

// ==================== Subtasks ====================

#[tauri::command]
pub async fn get_subtasks(task_id: i64) -> Result<ApiResponse<Vec<Subtask>>, String> {
    let pool = get_pool().await?;

    let subtasks = sqlx::query_as::<_, Subtask>(
        r#"
        SELECT * FROM subtasks
        WHERE task_id = ?
        ORDER BY sort_order ASC, created_at ASC
        "#
    )
    .bind(task_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| format!("Failed to get subtasks: {}", e))?;

    Ok(ApiResponse::success(subtasks))
}

#[tauri::command]
pub async fn create_subtask(task_id: i64, subtask: SubtaskInput) -> Result<ApiResponse<Subtask>, String> {
    let pool = get_pool().await?;
    let now = Utc::now().to_rfc3339();

    let result = sqlx::query_as::<_, Subtask>(
        r#"
        INSERT INTO subtasks (task_id, title, status, sort_order, created_at)
        VALUES (?, ?, ?, ?, ?)
        RETURNING *
        "#
    )
    .bind(task_id)
    .bind(&subtask.title)
    .bind("pending")
    .bind(subtask.sort_order.unwrap_or(0))
    .bind(&now)
    .fetch_one(&pool)
    .await
    .map_err(|e| format!("Failed to create subtask: {}", e))?;

    Ok(ApiResponse::success(result))
}

#[tauri::command]
pub async fn update_subtask(id: i64, subtask: SubtaskInput) -> Result<ApiResponse<Subtask>, String> {
    let pool = get_pool().await?;

    let result = sqlx::query_as::<_, Subtask>(
        r#"
        UPDATE subtasks
        SET title = ?, sort_order = ?
        WHERE id = ?
        RETURNING *
        "#
    )
    .bind(&subtask.title)
    .bind(subtask.sort_order.unwrap_or(0))
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|e| format!("Failed to update subtask: {}", e))?;

    Ok(ApiResponse::success(result))
}

#[tauri::command]
pub async fn delete_subtask(id: i64) -> Result<ApiResponse<()>, String> {
    let pool = get_pool().await?;

    sqlx::query("DELETE FROM subtasks WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| format!("Failed to delete subtask: {}", e))?;

    Ok(ApiResponse::success(()))
}

// ==================== Batch Operations ====================

#[tauri::command]
pub async fn batch_complete_tasks(ids: Vec<i64>) -> Result<ApiResponse<()>, String> {
    let pool = get_pool().await?;
    let now = Utc::now().to_rfc3339();

    for id in &ids {
        sqlx::query(
            r#"
            UPDATE tasks
            SET status = 'completed', completed_at = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&now)
        .bind(&now)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| format!("Failed to complete task {}: {}", id, e))?;

        record_task_history(&pool, *id, "completed", None)
            .await
            .map_err(|e| format!("Failed to record task history: {}", e))?;
    }

    Ok(ApiResponse::success(()))
}

#[tauri::command]
pub async fn batch_delete_tasks(ids: Vec<i64>) -> Result<ApiResponse<()>, String> {
    let pool = get_pool().await?;

    for id in &ids {
        sqlx::query("DELETE FROM tasks WHERE id = ?")
            .bind(id)
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to delete task {}: {}", id, e))?;
    }

    Ok(ApiResponse::success(()))
}

#[tauri::command]
pub async fn batch_archive_tasks(ids: Vec<i64>) -> Result<ApiResponse<()>, String> {
    let pool = get_pool().await?;
    let now = Utc::now().to_rfc3339();

    for id in &ids {
        sqlx::query(
            r#"
            UPDATE tasks
            SET status = 'archived', updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&now)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| format!("Failed to archive task {}: {}", id, e))?;

        record_task_history(&pool, *id, "archived", None)
            .await
            .map_err(|e| format!("Failed to record task history: {}", e))?;
    }

    Ok(ApiResponse::success(()))
}

// ==================== Search and Filter ====================

#[tauri::command]
pub async fn search_tasks(query: String, filters: Option<TaskFilter>) -> Result<ApiResponse<Vec<Task>>, String> {
    let mut filter = filters.unwrap_or_default();
    filter.search = Some(query);
    list_tasks(Some(filter)).await
}

#[tauri::command]
pub async fn get_tasks_by_quadrant(quadrant: i32) -> Result<ApiResponse<Vec<Task>>, String> {
    let pool = get_pool().await?;

    let (priority_condition, urgency_condition) = match quadrant {
        1 => ("priority = 'high'", "urgency = 'high'"),      // 第一象限
        2 => ("priority = 'high'", "urgency IN ('medium', 'low')"),  // 第二象限
        3 => ("priority IN ('medium', 'low')", "urgency = 'high'"),  // 第三象限
        4 => ("priority IN ('medium', 'low')", "urgency IN ('medium', 'low')"),  // 第四象限
        _ => return Err("Invalid quadrant number".to_string()),
    };

    let query = format!(
        "SELECT * FROM tasks WHERE {} AND {} AND status = 'pending' ORDER BY due_date ASC",
        priority_condition, urgency_condition
    );

    let tasks = sqlx::query_as::<_, Task>(&query)
        .fetch_all(&pool)
        .await
        .map_err(|e| format!("Failed to get tasks by quadrant: {}", e))?;

    Ok(ApiResponse::success(tasks))
}

#[tauri::command]
pub async fn get_tasks_by_date_range(start: String, end: String) -> Result<ApiResponse<Vec<Task>>, String> {
    let pool = get_pool().await?;

    let tasks = sqlx::query_as::<_, Task>(
        r#"
        SELECT * FROM tasks
        WHERE (due_date >= ? OR start_date >= ?)
          AND (due_date <= ? OR start_date <= ?)
          AND status IN ('pending', 'completed')
        ORDER BY due_date ASC
        "#
    )
    .bind(&start)
    .bind(&start)
    .bind(&end)
    .bind(&end)
    .fetch_all(&pool)
    .await
    .map_err(|e| format!("Failed to get tasks by date range: {}", e))?;

    Ok(ApiResponse::success(tasks))
}

// ==================== History and Archive ====================

#[tauri::command]
pub async fn get_task_history(task_id: i64) -> Result<ApiResponse<Vec<TaskHistory>>, String> {
    let pool = get_pool().await?;

    let history = sqlx::query_as::<_, TaskHistory>(
        "SELECT * FROM task_history WHERE task_id = ? ORDER BY created_at DESC"
    )
    .bind(task_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| format!("Failed to get task history: {}", e))?;

    Ok(ApiResponse::success(history))
}

#[tauri::command]
pub async fn restore_task(id: i64) -> Result<ApiResponse<Task>, String> {
    let pool = get_pool().await?;
    let now = Utc::now().to_rfc3339();

    let result = sqlx::query_as::<_, Task>(
        r#"
        UPDATE tasks
        SET status = 'pending', completed_at = NULL, updated_at = ?
        WHERE id = ?
        RETURNING *
        "#
    )
    .bind(&now)
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|e| format!("Failed to restore task: {}", e))?;

    record_task_history(&pool, id, "restored", None)
        .await
        .map_err(|e| format!("Failed to record task history: {}", e))?;

    Ok(ApiResponse::success(result))
}

// ==================== Data Export ====================

#[tauri::command]
pub async fn export_tasks(format: String) -> Result<ApiResponse<String>, String> {
    let pool = get_pool().await?;

    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
        .fetch_all(&pool)
        .await
        .map_err(|e| format!("Failed to export tasks: {}", e))?;

    let result = match format.as_str() {
        "csv" => {
            let mut csv = "id,title,description,priority,urgency,status,tags,start_date,due_date,reminder_time,estimated_duration,repeat_pattern,repeat_until,created_at,updated_at,completed_at\n".to_string();
            for task in &tasks {
                csv.push_str(&format!(
                    "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                    task.id,
                    task.title.replace(',', ";"),
                    task.description.as_deref().unwrap_or("").replace(',', ";"),
                    task.priority,
                    task.urgency,
                    task.status,
                    task.tags.as_deref().unwrap_or("").replace(',', ";"),
                    task.due_date.as_deref().unwrap_or(""),
                    task.reminder_time.as_deref().unwrap_or(""),
                    task.estimated_duration.unwrap_or(0),
                    task.repeat_pattern.as_deref().unwrap_or(""),
                    task.repeat_until.as_deref().unwrap_or(""),
                    task.created_at,
                    task.updated_at,
                    task.completed_at.as_deref().unwrap_or("")
                ));
            }
            csv
        }
        "json" => {
            serde_json::to_string_pretty(&tasks)
                .map_err(|e| format!("Failed to serialize to JSON: {}", e))?
        }
        _ => return Err("Unsupported format".to_string()),
    };

    Ok(ApiResponse::success(result))
}

#[tauri::command]
pub async fn import_tasks(data: String) -> Result<ApiResponse<usize>, String> {
    let pool = get_pool().await?;

    // Try to parse as JSON first
    if let Ok(tasks) = serde_json::from_str::<Vec<Task>>(&data) {
        let mut count = 0;
        for task in tasks {
            let result = sqlx::query(
                r#"
                INSERT INTO tasks (
                    title, description, priority, urgency, status,
                    tags, start_date, due_date, reminder_time,
                    estimated_duration, repeat_pattern, repeat_until,
                    created_at, updated_at, completed_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#
            )
            .bind(&task.title)
            .bind(&task.description)
            .bind(&task.priority)
            .bind(&task.urgency)
            .bind(&task.status)
            .bind(&task.tags)
            .bind(&task.start_date)
            .bind(&task.due_date)
            .bind(&task.reminder_time)
            .bind(task.estimated_duration)
            .bind(&task.repeat_pattern)
            .bind(&task.repeat_until)
            .bind(&task.created_at)
            .bind(&task.updated_at)
            .bind(&task.completed_at)
            .execute(&pool)
            .await;

            if result.is_ok() {
                count += 1;
            }
        }
        return Ok(ApiResponse::success(count));
    }

    // CSV parsing would go here (simplified for now)
    Err("Failed to parse import data. Expected JSON format.".to_string())
}

// ==================== Helper Functions ====================

async fn record_task_history(
    pool: &sqlx::SqlitePool,
    task_id: i64,
    action: &str,
    changes: Option<String>,
) -> Result<(), sqlx::Error> {
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO task_history (task_id, action, changes, created_at)
        VALUES (?, ?, ?, ?)
        "#
    )
    .bind(task_id)
    .bind(action)
    .bind(changes)
    .bind(&now)
    .execute(pool)
    .await?;

    Ok(())
}

// ==================== Backup and Restore ====================

#[tauri::command]
pub async fn backup_database() -> Result<ApiResponse<String>, String> {
    let pool = get_pool().await?;

    // Close the database to ensure all changes are flushed
    pool.close().await;

    let db_path = crate::db::get_db_path().await;

    // Generate backup filename with timestamp
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let backup_filename = format!("backup_{}.db", timestamp);

    let backup_path = {
        let mut path = dirs::data_local_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        path.push("personal-tools");
        path.push("backups");
        std::fs::create_dir_all(&path).map_err(|e| format!("Failed to create backups directory: {}", e))?;
        path.push(&backup_filename);
        path
    };

    // Copy the database file
    std::fs::copy(&db_path, &backup_path)
        .map_err(|e| format!("Failed to copy database: {}", e))?;

    // Reopen the database
    let new_pool = crate::db::init_db().await
        .map_err(|e| format!("Failed to reopen database: {}", e))?;

    // Update the global pool
    *DB_POOL.lock().await = Some(new_pool);

    Ok(ApiResponse::success(backup_path.to_string_lossy().to_string()))
}

#[tauri::command]
pub async fn restore_database(backup_path: String) -> Result<ApiResponse<String>, String> {
    let pool = get_pool().await?;

    // Close the current database
    pool.close().await;

    let db_path = crate::db::get_db_path().await;

    // Verify backup file exists
    if !std::path::Path::new(&backup_path).exists() {
        return Err("Backup file not found".to_string());
    }

    // Create a backup of current database before restoring
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let pre_restore_backup = {
        let mut path = dirs::data_local_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        path.push("personal-tools");
        path.push("backups");
        path.push(format!("pre_restore_{}.db", timestamp));
        path
    };
    std::fs::copy(&db_path, &pre_restore_backup)
        .map_err(|e| format!("Failed to create pre-restore backup: {}", e))?;

    // Copy the backup file to replace the current database
    std::fs::copy(&backup_path, &db_path)
        .map_err(|e| format!("Failed to restore database: {}", e))?;

    // Reopen the database
    let new_pool = crate::db::init_db().await
        .map_err(|e| format!("Failed to reopen database: {}", e))?;

    // Update the global pool
    *DB_POOL.lock().await = Some(new_pool);

    Ok(ApiResponse::success("Database restored successfully".to_string()))
}

#[tauri::command]
pub async fn list_backups() -> Result<ApiResponse<Vec<String>>, String> {
    let mut backup_dir = dirs::data_local_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    backup_dir.push("personal-tools");
    backup_dir.push("backups");

    let backups: Vec<String> = if backup_dir.exists() {
        std::fs::read_dir(&backup_dir)
            .map_err(|e| format!("Failed to read backups directory: {}", e))?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "db"))
            .map(|entry| entry.path().to_string_lossy().to_string())
            .collect()
    } else {
        Vec::new()
    };

    Ok(ApiResponse::success(backups))
}

#[tauri::command]
pub async fn delete_backup(backup_path: String) -> Result<ApiResponse<()>, String> {
    std::fs::remove_file(&backup_path)
        .map_err(|e| format!("Failed to delete backup: {}", e))?;

    Ok(ApiResponse::success(()))
}
