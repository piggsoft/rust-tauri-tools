# CODEBUDDY.md This file provides guidance to CodeBuddy when working with code in this repository.

## Project Overview
This is a Tauri + Rust + SQLx + SQLite + Vue 3 desktop application with two main modules:
1. **Task Management**: Personal task and subtask tracking with multiple views (list, calendar, quadrant)
2. **Password Management**: Secure password vault with categorization and search capabilities

## Build Commands

### Frontend Development
```bash
# Start development server (recommended)
npm run tauri:dev

# Alternative commands
npm run tauri dev
cd src-tauri && npx tauri dev
```

### Production Build
```bash
# Build application
npm run tauri:build

# Alternative
cd src-tauri && npx tauri build
```

### Frontend Only Commands
```bash
# Vite development server
npm run dev

# Preview production build
npm run preview
```

## Test Commands
Currently, there are no automated tests configured in this project.

### Manual Testing Guidelines
1. Test functionality must be performed within the Tauri application window
2. Browser environment cannot test Tauri's invoke() functions
3. Test all CRUD operations for tasks and passwords
4. Verify data persistence across application restarts
5. Test on target platforms (Windows, macOS, Linux) when possible
6. Test password functionality: copy to clipboard, show/hide passwords, URL navigation

### Adding Tests
When adding tests in the future:
1. Use Vitest for unit testing Vue components
2. Use @tauri-apps/api for testing Tauri commands
3. Place tests in __tests__ or tests directories
4. Follow Vue 3 Composition API testing patterns

## Linting & Formatting
No linting/formatting tools are currently configured.

### Recommended Setup
When adding linting:
1. Install ESLint: `npm install -D eslint @vue/eslint-config-typescript`
2. Install Prettier: `npm install -D prettier eslint-plugin-prettier eslint-config-prettier`
3. Add scripts to package.json:
   ```json
   "lint": "eslint src --ext .vue,.js,.ts",
   "format": "prettier --write src"
   ```

## Architecture

### Full-Stack Tauri Pattern
This desktop application runs Rust and JavaScript in the same process. The Rust backend handles system-level operations (file I/O, database), while Vue.js handles the UI through Tauri's bridge.

**Key Architecture Components:**
- Frontend invokes Rust commands via `@tauri-apps/api/core`'s `invoke()` function
- Rust handlers registered in `src-tauri/src/main.rs` using `#[tauri::command]` attribute
- All command handlers return `Result<ApiResponse<T>, String>` for consistent error handling
- Frontend API layer (`src/api/*.ts`) wraps invoke calls with type-safe TypeScript interfaces
- Pinia stores (`src/stores/*.ts`) manage state and orchestrate API calls

### Backend (Rust) Structure
**Database Layer (`src-tauri/src/db.rs`):**
- Uses `tokio::sync::Mutex` for thread-safe global DB pool
- `get_pool()` returns a shared SqlitePool with lazy initialization
- WAL mode enabled for better concurrency
- Foreign key constraints enforced
- Separate tables: tasks, subtasks, task_history, tags, passwords

**Models (`src-tauri/src/models.rs`):**
- All structs derive `Serialize, Deserialize, sqlx::FromRow` for automatic mapping
- Filter structs implement `Default` for optional filter parameters
- `ApiResponse<T>` wrapper for consistent success/error responses

**Command Handlers (`src-tauri/src/handlers.rs`):**
- Organized into sections: Task CRUD, Subtasks, Password Management, Search/Filter, Backup/Restore
- Each handler: gets pool from `get_pool()`, executes SQL, maps errors to String, wraps in ApiResponse
- Task history automatically recorded for create/update/complete/archive operations
- Password handlers support search by account, category, subcategory, and notes

**Tauri Registration (`src-tauri/src/main.rs`):**
- All commands registered in `tauri::generate_handler![]` macro
- Commands exported via `lib.rs` for proper compilation

### Frontend (Vue.js) Structure
**State Management (`src/stores/*.ts`):**
- Separate Pinia stores for each module: task.ts, password.ts
- Reactive state with computed getters for derived data
- Exported methods for component access
- Loading and error state management

**API Layer (`src/api/*.ts`):**
- Separate API files: tasks.ts, passwords.ts
- Each method wraps `invoke()` with response validation
- Type-safe parameters matching Rust handler signatures
- Automatic error throwing for failed responses

**Component Architecture:**
- `App.vue`: Root component with page routing (home, todo, password)
- Collapsible sidebar navigation with hover-to-expand behavior
- Components: TaskForm, TaskList, TaskItem, PasswordForm, CalendarView, QuadrantView, etc.
- Views: HomeView (feature selection), TaskView, PasswordView

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
- `passwords`: Independent table with category/subcategory filtering
- All timestamp fields stored as ISO 8601 strings (RFC 3339)
- Tags stored as JSON array strings in tasks table

### Key Implementation Details
- **Timezone Safety**: Use `src/utils/dateUtils.ts` for all date operations to avoid timezone issues
- **Error Handling**: Rust handlers return `Result<ApiResponse<T>, String>`; API methods throw JavaScript errors
- **Async Operations**: All DB queries use `.await`; global pool uses `tokio::sync::Mutex`
- **Serialization**: JavaScript objects → Rust structs automatically handled; pass `undefined` for optional empty arrays
- **Tauri Plugin Integration**: `@tauri-apps/plugin-dialog` and `@tauri-apps/plugin-fs` for file operations

## Code Style Guidelines

### TypeScript & Vue 3

#### File Organization
- Components: PascalCase.vue (e.g., TaskItem.vue, PasswordForm.vue)
- Stores: camelCase.ts (e.g., task.ts, password.ts)
- API files: camelCase.ts (e.g., tasks.ts, passwords.ts)
- Types: PascalCase.ts (e.g., Task.ts, Password.ts)
- Views: PascalCase.vue (e.g., TaskView.vue, PasswordView.vue)
- Utilities: camelCase.ts (e.g., dateUtils.ts)

#### Import Order
1. Vue/Pinia imports
2. Relative imports (components, stores, types, api)
3. Third-party library imports
4. Style imports

Example:
```typescript
import { computed } from 'vue'
import { useTaskStore } from '../stores/task'
import type { Task } from '../types'
import { someFunction } from '@/api/tasks'
import './style.css'
```

#### Component Structure
```vue
<script setup lang="ts">
// Imports
// Props definition
// Emits definition
// Store usage
// Computed properties
// Methods
</script>

<template>
<!-- Template content -->
</template>

<style scoped>
/* Component-scoped styles */
</style>
```

#### Naming Conventions
- Components: PascalCase (TaskItem, TaskForm, PasswordForm)
- Variables/functions: camelCase (taskTitle, handleEdit, passwordValue)
- Constants: UPPER_SNAKE_CASE (MAX_ITEMS, API_URL)
- Types/Interfaces: PascalCase (Task, Subtask, Password, PasswordInput)
- Events: kebab-case (edit-task, toggle-complete, copy-password)
- Props: camelCase (taskId, isSelected, showPassword)

#### Props Definition
```typescript
const props = defineProps<{
  task: Task
  readonly?: boolean
}>()
```

#### Emits Definition
```typescript
const emit = defineEmits<{
  edit: [task: Task]
  delete: [id: string]
}>()
```

#### Store Usage
```typescript
// Task store
const taskStore = useTaskStore()
const tasks = taskStore.tasks
taskStore.addTask(newTask)

// Password store
const passwordStore = usePasswordStore()
const passwords = passwordStore.passwords
passwordStore.createPassword(newPassword)
```

#### Computed Properties
```typescript
const isOverdue = computed(() => {
  if (!props.task.due_date || props.task.status === 'completed') return false
  return new Date(props.task.due_date) < new Date()
})
```

#### Methods
```typescript
function handleEdit() {
  emit('edit', props.task)
}

function handleToggleComplete() {
  taskStore.toggleComplete(props.task.id)
}
```

#### Template Guidelines
- Use v-if for conditional rendering
- Use v-for with :key for lists
- Use @ shorthand for event handlers
- Use : shorthand for binding props
- Keep templates readable and maintainable

#### Styling
- Use scoped styles for component-specific CSS
- Follow BEM-like naming for classes
- Use CSS variables for theme colors
- Prefer CSS Flexbox/Grid for layout
- Avoid !important unless absolutely necessary

### Rust Backend (src-tauri/src/)

#### File Organization
- main.rs: Tauri application entry and command registration
- models.rs: Data structures (Task, Password, etc.)
- db.rs: Database connection, initialization, and migrations
- handlers.rs: Tauri command handlers (task and password CRUD)

#### Import Order
1. Standard library imports
2. External crate imports
3. Internal module imports
4. Macro imports

Example:
```rust
use serde::{Deserialize, Serialize};
use sqlx::{Sqlite, Pool, FromRow};
use tauri::command;
```

#### Naming Conventions
- Functions: snake_case (get_tasks, create_task, create_password)
- Types/Structs: PascalCase (Task, Subtask, Password, PasswordInput)
- Constants: UPPER_SNAKE_CASE (MAX_RETRY_ATTEMPTS)
- Variables: snake_case (task_id, db_pool)
- Modules: snake_case (database, handlers)

#### Error Handling
```rust
#[tauri::command]
async fn get_tasks(pool: State<'_, Pool<Sqlite>>) -> Result<Vec<Task>, String> {
    sqlx::query_as::<_, Task>("SELECT * FROM tasks")
        .fetch_all(&pool)
        .await
        .map_err(|e| format!("Database error: {}", e))
}
```

#### Database Operations
- Use sqlx::query_as for type-safe queries
- Handle database errors gracefully
- Use transactions for multiple related operations
- Close connections properly

#### Tauri Commands
```rust
#[tauri::command]
fn create_task(
    task: Task,
    pool: State<'_, Pool<Sqlite>>
) -> Result<Task, String> {
    // Implementation
}
```

## Directory Structure Guidelines

### Frontend (src/)
```
/src
  /api          # API service functions (tasks.ts, passwords.ts)
  /components   # Vue components (TaskItem.vue, PasswordForm.vue, etc.)
  /stores       # Pinia stores (task.ts, password.ts)
  /types        # TypeScript type definitions
  /utils        # Utility functions (dateUtils.ts, etc.)
  /views        # Page-level components (TaskView.vue, PasswordView.vue, HomeView.vue)
  App.vue       # Root component with page routing
  main.ts       # Application entry
```

### Backend (src-tauri/src/)
```
/src-tauri/src
  main.rs       # Tauri entry point
  models.rs     # Data models
  db.rs         # Database setup
  handlers.rs   # Command handlers
  Cargo.toml    # Rust dependencies
```

### Configuration Files
- vite.config.ts    # Vite configuration
- tsconfig.json     # TypeScript configuration
- tauri.conf.json   # Tauri configuration
- package.json      # Node.js dependencies and scripts

## Best Practices

### State Management
1. Use Pinia stores for shared state
2. Keep component state local when possible
3. Normalize data in stores
4. Use getters for derived state
5. Persist state when appropriate (localStorage/plugins)

### API Layer
1. Encapsulate all Tauri invocations in API files
2. Handle loading/error states in components
3. Use consistent response formats
4. Abstract away Tauri specifics from components

### Component Design
1. Keep components small and focused
2. Use composition over inheritance
3. Pass data down via props, events up via emits
4. Use slots for flexible component composition
5. Avoid deep prop drilling

### Performance
1. Use v-show for frequently toggled elements
2. Implement lazy loading for large lists
3. Debounce expensive operations
4. Use computed properties for derived data
5. Minimize reactive properties

### Security
1. Validate all inputs before processing
2. Use parameterized queries to prevent SQL injection
3. Sanitize user-generated content
4. Implement proper error handling without leaking sensitive info
5. Follow principle of least privilege for Tauri permissions

## Troubleshooting

### Common Issues
1. **Tauri invoke not working**: Ensure testing in Tauri window, not browser
2. **Database locking**: Check for unclosed connections in Rust handlers
3. **Type mismatches**: Verify TypeScript definitions match database schema
4. **Performance drops**: Check for unnecessary computations in templates
5. **Styling issues**: Verify CSS specificity and scoped styles
6. **Timezone issues with dates**: Use timezone-safe utilities from `src/utils/dateUtils.ts`

### Debugging
1. Use console.log in Vue components (visible in devtools)
2. Use println! in Rust handlers (visible in terminal)
3. Use Tauri's built-in debugging tools
4. Inspect database directly with SQLite tools
5. Use Vue Devtools for component inspection
