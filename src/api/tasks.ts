import { invoke } from '@tauri-apps/api/core'
import type { Task, TaskInput, TaskFilter, ApiResponse, Subtask } from '../types'

export const taskApi = {
  // Task CRUD
  async createTask(task: TaskInput): Promise<Task> {
    const response: ApiResponse<Task> = await invoke('create_task', { task })
    if (!response.success || !response.data) {
      throw new Error(response.error || 'Failed to create task')
    }
    return response.data
  },

  async updateTask(id: number, task: TaskInput): Promise<Task> {
    const response: ApiResponse<Task> = await invoke('update_task', { id, task })
    if (!response.success || !response.data) {
      throw new Error(response.error || 'Failed to update task')
    }
    return response.data
  },

  async deleteTask(id: number): Promise<void> {
    const response: ApiResponse<void> = await invoke('delete_task', { id })
    if (!response.success) {
      throw new Error(response.error || 'Failed to delete task')
    }
  },

  async getTask(id: number): Promise<Task> {
    const response: ApiResponse<Task> = await invoke('get_task', { id })
    if (!response.success || !response.data) {
      throw new Error(response.error || 'Failed to get task')
    }
    return response.data
  },

  async listTasks(filter?: TaskFilter): Promise<Task[]> {
    const response: ApiResponse<Task[]> = await invoke('list_tasks', { filter })
    if (!response.success || !response.data) {
      throw new Error(response.error || 'Failed to list tasks')
    }
    return response.data
  },

  // Subtasks
  async getSubtasks(taskId: number): Promise<Subtask[]> {
    const response: ApiResponse<Subtask[]> = await invoke('get_subtasks', { taskId })
    if (!response.success || !response.data) {
      throw new Error(response.error || 'Failed to get subtasks')
    }
    return response.data
  },

  async createSubtask(taskId: number, title: string, sortOrder = 0): Promise<number> {
    const response = await invoke('create_subtask', {
      taskId,
      subtask: { title, sort_order: sortOrder }
    })
    return response as number
  },

  async updateSubtask(id: number, title: string, sortOrder?: number): Promise<void> {
    await invoke('update_subtask', {
      id,
      subtask: { title, sort_order: sortOrder }
    })
  },

  async deleteSubtask(id: number): Promise<void> {
    await invoke('delete_subtask', { id })
  },

  // Batch operations
  async batchCompleteTasks(ids: number[]): Promise<void> {
    const response: ApiResponse<void> = await invoke('batch_complete_tasks', { ids })
    if (!response.success) {
      throw new Error(response.error || 'Failed to complete tasks')
    }
  },

  async batchDeleteTasks(ids: number[]): Promise<void> {
    const response: ApiResponse<void> = await invoke('batch_delete_tasks', { ids })
    if (!response.success) {
      throw new Error(response.error || 'Failed to delete tasks')
    }
  },

  async batchArchiveTasks(ids: number[]): Promise<void> {
    const response: ApiResponse<void> = await invoke('batch_archive_tasks', { ids })
    if (!response.success) {
      throw new Error(response.error || 'Failed to archive tasks')
    }
  },

  // Search and filter
  async searchTasks(query: string, filters?: TaskFilter): Promise<Task[]> {
    const response: ApiResponse<Task[]> = await invoke('search_tasks', { query, filters })
    if (!response.success || !response.data) {
      throw new Error(response.error || 'Failed to search tasks')
    }
    return response.data
  },

  async getTasksByQuadrant(quadrant: 1 | 2 | 3 | 4): Promise<Task[]> {
    const response: ApiResponse<Task[]> = await invoke('get_tasks_by_quadrant', { quadrant })
    if (!response.success || !response.data) {
      throw new Error(response.error || 'Failed to get tasks by quadrant')
    }
    return response.data
  },

  async getTasksByDateRange(start: string, end: string): Promise<Task[]> {
    const response: ApiResponse<Task[]> = await invoke('get_tasks_by_date_range', { start, end })
    if (!response.success || !response.data) {
      throw new Error(response.error || 'Failed to get tasks by date range')
    }
    return response.data
  },

  // History and archive
  async getTaskHistory(taskId: number): Promise<any[]> {
    const response = await invoke('get_task_history', { taskId })
    if (!response.success || !response.data) {
      throw new Error(response.error || 'Failed to get task history')
    }
    return response.data
  },

  async restoreTask(id: number): Promise<Task> {
    const response: ApiResponse<Task> = await invoke('restore_task', { id })
    if (!response.success || !response.data) {
      throw new Error(response.error || 'Failed to restore task')
    }
    return response.data
  },

  // Data export/import
  async exportTasks(format: 'csv' | 'json'): Promise<string> {
    const response: ApiResponse<string> = await invoke('export_tasks', { format })
    if (!response.success || !response.data) {
      throw new Error(response.error || 'Failed to export tasks')
    }
    return response.data
  },

  async importTasks(data: string): Promise<number> {
    const response: ApiResponse<number> = await invoke('import_tasks', { data })
    if (!response.success || !response.data) {
      throw new Error(response.error || 'Failed to import tasks')
    }
    return response.data
  },

  // Backup and restore
  async backupDatabase(): Promise<string> {
    const response: ApiResponse<string> = await invoke('backup_database')
    if (!response.success || !response.data) {
      throw new Error(response.error || 'Failed to backup database')
    }
    return response.data
  },

  async restoreDatabase(backupPath: string): Promise<string> {
    const response: ApiResponse<string> = await invoke('restore_database', { backupPath })
    if (!response.success || !response.data) {
      throw new Error(response.error || 'Failed to restore database')
    }
    return response.data
  },

  async listBackups(): Promise<string[]> {
    const response: ApiResponse<string[]> = await invoke('list_backups')
    if (!response.success || !response.data) {
      throw new Error(response.error || 'Failed to list backups')
    }
    return response.data
  },

  async deleteBackup(backupPath: string): Promise<void> {
    const response: ApiResponse<void> = await invoke('delete_backup', { backupPath })
    if (!response.success) {
      throw new Error(response.error || 'Failed to delete backup')
    }
  }
}
