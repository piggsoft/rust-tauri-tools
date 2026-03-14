import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { taskApi } from '../api/tasks'
import type { Task, TaskInput, TaskFilter, ViewType } from '../types'

export const useTaskStore = defineStore('task', () => {
  const tasks = ref<Task[]>([])
  const currentView = ref<ViewType>('list')
  const selectedTask = ref<Task | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  const filters = ref<TaskFilter>({
    search: '',
    tags: [],
    status: ['pending'],
    priority: [],
    urgency: [],
    date_start: undefined,
    date_end: undefined,
  })

  const selectedTasks = ref<Set<number>>(new Set())

  // Computed properties
  const pendingTasks = computed(() =>
    tasks.value.filter(t => t.status === 'pending')
  )

  const completedTasks = computed(() =>
    tasks.value.filter(t => t.status === 'completed')
  )

  const archivedTasks = computed(() =>
    tasks.value.filter(t => t.status === 'archived')
  )

  const overdueTasks = computed(() => {
    const now = new Date()
    return tasks.value.filter(t =>
      t.status === 'pending' &&
      t.due_date &&
      new Date(t.due_date) < now
    )
  })

  // Actions
  async function fetchTasks() {
    loading.value = true
    error.value = null
    try {
      tasks.value = await taskApi.listTasks(filters.value)
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to fetch tasks'
    } finally {
      loading.value = false
    }
  }

  async function createTask(task: TaskInput) {
    loading.value = true
    error.value = null
    try {
      const newTask = await taskApi.createTask(task)
      tasks.value.push(newTask)
      return newTask
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to create task'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function updateTask(id: number, task: TaskInput) {
    loading.value = true
    error.value = null
    try {
      const updatedTask = await taskApi.updateTask(id, task)
      const index = tasks.value.findIndex(t => t.id === id)
      if (index !== -1) {
        tasks.value[index] = updatedTask
      }
      if (selectedTask.value?.id === id) {
        selectedTask.value = updatedTask
      }
      return updatedTask
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to update task'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function deleteTask(id: number) {
    loading.value = true
    error.value = null
    try {
      await taskApi.deleteTask(id)
      tasks.value = tasks.value.filter(t => t.id !== id)
      if (selectedTask.value?.id === id) {
        selectedTask.value = null
      }
      selectedTasks.value.delete(id)
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to delete task'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function toggleComplete(id: number) {
    const task = tasks.value.find(t => t.id === id)
    if (!task) return

    const newStatus = task.status === 'pending' ? 'completed' : 'pending'
    await updateTask(id, {
      title: task.title,
      description: task.description,
      priority: task.priority,
      urgency: task.urgency,
      tags: task.tags,
      start_date: task.start_date,
      due_date: task.due_date,
      reminder_time: task.reminder_time,
      estimated_duration: task.estimated_duration,
      repeat_pattern: task.repeat_pattern,
      repeat_until: task.repeat_until,
    })
  }

  async function getSubtasks(taskId: number) {
    loading.value = true
    error.value = null
    try {
      return await taskApi.getSubtasks(taskId)
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to get subtasks'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function createSubtask(taskId: number, title: string, sortOrder = 0) {
    loading.value = true
    error.value = null
    try {
      return await taskApi.createSubtask(taskId, title, sortOrder)
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to create subtask'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function updateSubtask(id: number, titleOrStatus: string, sortOrder?: number) {
    loading.value = true
    error.value = null
    try {
      await taskApi.updateSubtask(id, titleOrStatus, sortOrder)
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to update subtask'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function deleteSubtask(id: number) {
    loading.value = true
    error.value = null
    try {
      await taskApi.deleteSubtask(id)
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to delete subtask'
      throw e
    } finally {
      loading.value = false
    }
  }


  async function batchComplete() {
    if (selectedTasks.value.size === 0) return

    loading.value = true
    error.value = null
    try {
      await taskApi.batchCompleteTasks(Array.from(selectedTasks.value))
      await fetchTasks()
      selectedTasks.value.clear()
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to complete tasks'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function batchDelete() {
    if (selectedTasks.value.size === 0) return

    loading.value = true
    error.value = null
    try {
      await taskApi.batchDeleteTasks(Array.from(selectedTasks.value))
      await fetchTasks()
      selectedTasks.value.clear()
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to delete tasks'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function batchArchive() {
    if (selectedTasks.value.size === 0) return

    loading.value = true
    error.value = null
    try {
      await taskApi.batchArchiveTasks(Array.from(selectedTasks.value))
      await fetchTasks()
      selectedTasks.value.clear()
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to archive tasks'
      throw e
    } finally {
      loading.value = false
    }
  }

  function switchView(view: ViewType) {
    currentView.value = view
  }

  function updateFilters(newFilters: Partial<TaskFilter>) {
    filters.value = { ...filters.value, ...newFilters }
  }

  function toggleTaskSelection(id: number) {
    if (selectedTasks.value.has(id)) {
      selectedTasks.value.delete(id)
    } else {
      selectedTasks.value.add(id)
    }
  }

  function clearSelection() {
    selectedTasks.value.clear()
  }

  function selectTask(task: Task) {
    selectedTask.value = task
  }

  function clearSelectedTask() {
    selectedTask.value = null
  }

  function resetFilters() {
    filters.value = {
      search: '',
      tags: [],
      status: ['pending'],
      priority: [],
      urgency: [],
      date_start: undefined,
      date_end: undefined,
    }
  }

  return {
    tasks,
    currentView,
    selectedTask,
    loading,
    error,
    filters,
    selectedTasks,
    pendingTasks,
    completedTasks,
    archivedTasks,
    overdueTasks,
    fetchTasks,
    createTask,
    updateTask,
    deleteTask,
    toggleComplete,
    getSubtasks,
    createSubtask,
    updateSubtask,
    deleteSubtask,
    batchComplete,
    batchDelete,
    batchArchive,
    switchView,
    updateFilters,
    toggleTaskSelection,
    clearSelection,
    selectTask,
    clearSelectedTask,
    resetFilters,
  }
})
