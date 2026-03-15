use sqlx::SqlitePool;
use std::path::PathBuf;

pub async fn get_db_path() -> PathBuf {
    // Try to use AppData directory, fallback to current directory if it fails
    let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("personal-tools");
    path.push("data");
    
    // Try to create directory, fallback to current directory if it fails
    if std::fs::create_dir_all(&path).is_err() {
        eprintln!("Failed to create AppData directory, using current directory");
        path = PathBuf::from(".");
        path.push("data");
        std::fs::create_dir_all(&path).ok();
    }
    
    path.push("todo.db");
    path
}

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    let db_path = get_db_path().await;
    
    // Ensure parent directory exists
    if let Some(parent) = db_path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            eprintln!("Warning: Failed to create database directory: {}", e);
            eprintln!("Database may fail to initialize");
        }
    }
    
    eprintln!("Database path: {}", db_path.display());
    
    // Try multiple connection strategies
    let pool = try_connect(&db_path).await?;
    
    // Enable WAL mode for better concurrency
    sqlx::query("PRAGMA journal_mode = WAL")
        .execute(&pool)
        .await?;

    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&pool)
        .await?;
    
    eprintln!("Database initialized successfully");
    Ok(pool)
}

async fn try_connect(db_path: &std::path::Path) -> Result<SqlitePool, sqlx::Error> {
    // Strategy 1: Use sqlite:// with mode=rwc
    let path_str = db_path.to_string_lossy().replace('\\', "/");
    let database_url = format!("sqlite://{}?mode=rwc", path_str);
    
    eprintln!("Trying connection URL: {}", database_url);
    
    match SqlitePool::connect(&database_url).await {
        Ok(pool) => return Ok(pool),
        Err(e) => eprintln!("Strategy 1 failed: {}", e),
    }
    
    // Strategy 2: Use simple sqlite: format
    let database_url = format!("sqlite:{}", db_path.display());
    eprintln!("Trying connection URL: {}", database_url);
    
    match SqlitePool::connect(&database_url).await {
        Ok(pool) => return Ok(pool),
        Err(e) => eprintln!("Strategy 2 failed: {}", e),
    }
    
    // Strategy 3: Use in-memory database as last resort
    eprintln!("Falling back to in-memory database");
    let database_url = "sqlite::memory:";
    SqlitePool::connect(database_url).await
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Create tasks table
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT,
            priority TEXT NOT NULL CHECK(priority IN ('high', 'medium', 'low')),
            urgency TEXT NOT NULL CHECK(urgency IN ('high', 'medium', 'low')),
            status TEXT NOT NULL DEFAULT 'pending' CHECK(status IN ('pending', 'completed', 'archived')),
            tags TEXT,
            start_date TEXT,
            due_date TEXT,
            reminder_time TEXT,
            estimated_duration INTEGER,
            repeat_pattern TEXT,
            repeat_until TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            completed_at TEXT
        )
    "#).execute(pool).await?;

    // Create subtasks table
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS subtasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending' CHECK(status IN ('pending', 'completed')),
            sort_order INTEGER DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
        )
    "#).execute(pool).await?;

    // Create task_history table
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS task_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task_id INTEGER NOT NULL,
            action TEXT NOT NULL,
            changes TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
        )
    "#).execute(pool).await?;

    // Create tags table
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )
    "#).execute(pool).await?;

    // Create passwords table
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS passwords (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            category TEXT NOT NULL,
            subcategory TEXT,
            account TEXT NOT NULL,
            password TEXT NOT NULL,
            login_url TEXT,
            notes TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )
    "#).execute(pool).await?;

    // Create indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_due_date ON tasks(due_date)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_priority ON tasks(priority)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_urgency ON tasks(urgency)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_subtasks_task_id ON subtasks(task_id)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_task_history_task_id ON task_history(task_id)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_passwords_category ON passwords(category)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_passwords_subcategory ON passwords(subcategory)")
        .execute(pool)
        .await?;

    Ok(())
}

// Global database pool
use tokio::sync::Mutex;
use once_cell::sync::Lazy;

pub static DB_POOL: Lazy<Mutex<Option<SqlitePool>>> = Lazy::new(|| Mutex::new(None));

pub async fn get_pool() -> Result<SqlitePool, String> {
    let mut pool = DB_POOL.lock().await;
    if pool.is_none() {
        let new_pool = init_db().await.map_err(|e| format!("Failed to initialize DB: {}", e))?;
        run_migrations(&new_pool).await.map_err(|e| format!("Failed to run migrations: {}", e))?;
        *pool = Some(new_pool);
    }
    Ok(pool.as_ref().unwrap().clone())
}

pub async fn tags_to_json(tags: Option<&Vec<String>>) -> Option<String> {
    tags.map(|t| serde_json::to_string(t).ok())
        .flatten()
}

pub async fn json_to_tags(json: Option<&str>) -> Option<Vec<String>> {
    json.and_then(|s| serde_json::from_str::<Vec<String>>(s).ok())
}
