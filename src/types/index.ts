export interface Task {
  id: number
  title: string
  description?: string
  priority: 'high' | 'medium' | 'low'
  urgency: 'high' | 'medium' | 'low'
  status: 'pending' | 'completed' | 'archived'
  tags?: string[]
  start_date?: string
  due_date?: string
  reminder_time?: string
  estimated_duration?: number
  repeat_pattern?: string
  repeat_until?: string
  created_at: string
  updated_at: string
  completed_at?: string
}

export interface TaskInput {
  title: string
  description?: string
  priority: 'high' | 'medium' | 'low'
  urgency: 'high' | 'medium' | 'low'
  tags?: string[]
  start_date?: string
  due_date?: string
  reminder_time?: string
  estimated_duration?: number
  repeat_pattern?: string
  repeat_until?: string
}

export interface Subtask {
  id: number
  task_id: number
  title: string
  status: 'pending' | 'completed'
  sort_order: number
  created_at: string
}

export interface TaskHistory {
  id: number
  task_id: number
  action: string
  changes?: string
  created_at: string
}

export interface TaskFilter {
  search?: string
  tags?: string[]
  status?: string[]
  priority?: string[]
  urgency?: string[]
  date_start?: string
  date_end?: string
}

export interface ApiResponse<T> {
  success: boolean
  data?: T
  error?: string
}

export type ViewType = 'calendar' | 'list' | 'quadrant'

export type Quadrant = 1 | 2 | 3 | 4
