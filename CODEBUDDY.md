# CODEBUDDY.md This file provides guidance to CodeBuddy when working with code in this repository.

## Project Overview

Personal Tools is a task management desktop application built with Tauri 2 (Rust backend) + Vue 3 (TypeScript frontend). It uses SQLite for data persistence with full CRUD operations, advanced filtering, search, backup/restore, and subtask management.

## Development Commands

### Frontend (Vue.js)
```bash
# Start Vite dev server (port 5173 or next available)
npm run dev

# Build for production
npm run build

# Type checking
npm run build  # Runs vue-tsc -b
```

### Backend (Rust/Tauri)
```bash
# Start Tauri dev mode (runs both frontend and backend) - Recommended
npm run tauri:dev

# Or use full command
npm run tauri dev

# Or run from src-tauri directory
cd src-tauri && npx tauri dev

# Run Rust binary only
cd src-tauri && cargo run

# Build release version
cd src-tauri && cargo build --release

# Clean build artifacts
cd src-tauri && cargo clean
```

### Dependencies
```bash
# Install frontend dependencies (includes Tauri CLI)
npm install

# Add missing Tauri plugins
npm install @tauri-apps/plugin-dialog @tauri-apps/plugin-fs
```

## Architecture

### Full-Stack Tauri Architecture

This is a **desktop application** where Rust and JavaScript run in the same process. The Rust backend handles system-level operations (file I/O, database), while Vue.js handles the UI through Tauri's bridge.

**Key Architectural Pattern:**
- Frontend invokes Rust commands via `@tauri-apps/api/core`'s `invoke()` function
- Rust handlers are registered in `src-tauri/src/main.rs` using `#[tauri::command]` attribute
- All command handlers return `Result<ApiResponse<T>, String>` for consistent error handling
- Frontend API layer (`src/api/tasks.ts`) wraps invoke calls with type-safe TypeScript interfaces

### Backend (Rust) Structure

**Database Layer (`src-tauri/src/db.rs`):**
- Uses `tokio::sync::Mutex` for thread-safe global DB pool
- `get_pool()` returns a shared SqlitePool with lazy initialization
- WAL mode enabled for better concurrency
- Foreign key constraints enforced
- Indexes on status, due_date, priority, urgency for query optimization

**Models (`src-tauri/src/models.rs`):**
- All structs derive `Serialize, Deserialize, sqlx::FromRow` for automatic mapping
- `TaskFilter` implements `Default` for optional filter parameters
- `ApiResponse<T>` wrapper for consistent success/error responses

**Command Handlers (`src-tauri/src/handlers.rs`):**
- Organized into sections: Task CRUD, Subtasks, Search/Filter, Data Export, Backup/Restore
- Each handler: gets pool from `get_pool()`, executes SQL, maps errors to String, wraps in ApiResponse
- Task history automatically recorded for create/update/complete/archive operations
- `record_task_history()` helper requires `.map_err()` to convert `sqlx::Error` to `String`

**Tauri Registration (`src-tauri/src/main.rs`):**
- All commands registered in `tauri::generate_handler![]` macro
- Commands are exported via `lib.rs` for proper compilation

### Frontend (Vue.js) Structure

**State Management (`src/stores/task.ts`):**
- Pinia store with reactive state for tasks, filters, loading
- Fetch methods call API and update state atomically
- Exported subtask methods in return statement for component access

**API Layer (`src/api/tasks.ts`):**
- Single `taskApi` object with all backend methods
- Each method wraps `invoke()` with response validation
- Type-safe parameters matching Rust handler signatures
- Automatic error throwing for failed responses

**Component Architecture:**
- `TaskView.vue`: Main task list with inline editing
- `SearchFilterPanel.vue`: Unified search/filter component used across all views
- `DataManager.vue`: Modal overlay for export/import/backup operations
- `CalendarView.vue` & `QuadrantView.vue`: Alternative visualizations
- All components emit events to parent for state changes

**View Pattern:**
- Views use Pinia store for data fetching
- Components use props/emit for communication
- No direct database access from components

### Data Flow

1. User action → Vue component event handler
2. Component → Pinia store method or direct API call
3. Store/API → `invoke('command_name', { params })`
4. Tauri bridge → Rust handler in `handlers.rs`
5. Rust handler → SQLite query via `sqlx`
6. Response → Wrapped in `ApiResponse<T>` → JSON serialization
7. Frontend receives JSON → Validates response → Updates UI state

### Database Schema Relationships

- `tasks` (parent) ← one-to-many → `subtasks` (child) with CASCADE delete
- `tasks` ← one-to-many → `task_history` with CASCADE delete
- All timestamp fields stored as ISO 8601 strings (RFC 3339)
- Tags stored as JSON array strings in tasks table

## Key Implementation Details

### Error Handling
- Rust: All handlers return `Result<ApiResponse<T>, String>` - errors converted to String via `.map_err()`
- Frontend: API methods throw JavaScript errors on failed responses
- UI: Components show error messages in message banners

### Asynchronous Database Access
- Must use `.await` with `tokio::sync::Mutex` lock operations: `*DB_POOL.lock().await = Some(pool)`
- All SQL queries use `.await` for async execution
- Never mix `std::sync::Mutex` with async code (Send trait issues)

### Tauri Plugin Integration
- `@tauri-apps/plugin-dialog`: File save/open dialogs (`save()`, `open()`)
- `@tauri-apps/plugin-fs`: File system operations (`readTextFile()`, `writeTextFile()`)
- Install missing plugins: `npm install @tauri-apps/plugin-dialog`

### File System Paths
- Database: Platform-specific app data directory + "personal-tools/data/todo.db"
- Backups: Same directory + "backups/" subdirectory
- Use `dirs::data_local_dir()` for cross-platform path resolution

### Compilation Notes
- Icon file required at `src-tauri/icons/icon.ico` for Windows resource compilation
- If missing, copy any valid `.ico` file or disable bundle in `tauri.conf.json`
- `cargo clean` may be needed after changing `Cargo.toml` or dependencies

## Common Issues

**"Failed to resolve import @tauri-apps/plugin-dialog"**: Install the missing package with `npm install @tauri-apps/plugin-dialog`

**"future cannot be sent between threads safely"**: Use `tokio::sync::Mutex` instead of `std::sync::Mutex` for the global DB_POOL

**"icon.ico not found"**: Windows resource compilation requires a valid .ico file. Copy from system (e.g., `C:\Windows\System32\OneDrive.ico`) as temporary solution

**Import statement conflicts**: Import multiple items from same package in one line (e.g., `import { save, open } from '@tauri-apps/plugin-dialog'`)
