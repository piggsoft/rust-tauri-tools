// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod db;
mod handlers;

fn main() {
    // Initialize database pool on startup
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    rt.block_on(async {
        match db::get_pool().await {
            Ok(_) => println!("Database initialized successfully"),
            Err(e) => eprintln!("Failed to initialize database: {}", e),
        }
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            // Task commands
            handlers::create_task,
            handlers::update_task,
            handlers::delete_task,
            handlers::get_task,
            handlers::list_tasks,
            // Subtask commands
            handlers::get_subtasks,
            handlers::create_subtask,
            handlers::update_subtask,
            handlers::delete_subtask,
            // Batch operations
            handlers::batch_complete_tasks,
            handlers::batch_delete_tasks,
            handlers::batch_archive_tasks,
            // Search and filter
            handlers::search_tasks,
            handlers::get_tasks_by_quadrant,
            handlers::get_tasks_by_date_range,
            // History and archive
            handlers::get_task_history,
            handlers::restore_task,
            // Data export/import
            handlers::export_tasks,
            handlers::import_tasks,
            // Backup and restore
            handlers::backup_database,
            handlers::restore_database,
            handlers::list_backups,
            handlers::delete_backup,
            // Password management
            handlers::create_password,
            handlers::get_password,
            handlers::list_passwords,
            handlers::update_password,
            handlers::delete_password,
            handlers::get_password_categories,
            handlers::get_password_subcategories,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
