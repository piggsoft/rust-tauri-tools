<script setup lang="ts">
import { computed } from 'vue'
import { useTaskStore } from '../stores/task'
import type { Task } from '../types'

const props = defineProps<{
  task: Task
}>()

const emit = defineEmits<{
  edit: [task: Task]
}>()

const taskStore = useTaskStore()

const isSelected = computed(() => taskStore.selectedTasks.has(props.task.id))

const priorityLabels = {
  high: '高',
  medium: '中',
  low: '低',
}

const priorityColors = {
  high: '#ff4d4f',
  medium: '#faad14',
  low: '#52c41a',
}

const urgencyLabels = {
  high: '紧急',
  medium: '一般',
  low: '不急',
}

const urgencyColors = {
  high: '#ff4d4f',
  medium: '#faad14',
  low: '#52c41a',
}

const isOverdue = computed(() => {
  if (!props.task.due_date || props.task.status === 'completed') return false
  return new Date(props.task.due_date) < new Date()
})

const dueDateText = computed(() => {
  if (!props.task.due_date) return null
  const date = new Date(props.task.due_date)
  const today = new Date()
  today.setHours(0, 0, 0, 0)

  const taskDate = new Date(date)
  taskDate.setHours(0, 0, 0, 0)

  if (taskDate.getTime() === today.getTime()) {
    return '今天'
  } else if (taskDate.getTime() === today.getTime() + 24 * 60 * 60 * 1000) {
    return '明天'
  } else {
    return `${date.getMonth() + 1}/${date.getDate()}`
  }
})

const dueTimeText = computed(() => {
  if (!props.task.due_date) return null
  const date = new Date(props.task.due_date)
  return `${date.getHours().toString().padStart(2, '0')}:${date.getMinutes().toString().padStart(2, '0')}`
})

function handleCheckboxChange(e: Event) {
  const checked = (e.target as HTMLInputElement).checked
  taskStore.toggleTaskSelection(props.task.id)
}

function handleEdit() {
  emit('edit', props.task)
}

function handleToggleComplete() {
  taskStore.toggleComplete(props.task.id)
}
</script>

<template>
  <div
    class="task-item"
    :class="{
      'is-completed': task.status === 'completed',
      'is-overdue': isOverdue,
      'is-selected': isSelected,
    }"
    @click="handleEdit"
  >
    <div class="task-left">
      <div class="task-checkbox" @click.stop>
        <input
          type="checkbox"
          :checked="task.status === 'completed'"
          @change="handleToggleComplete"
        />
      </div>
      <div class="task-select" @click.stop>
        <input
          type="checkbox"
          :checked="isSelected"
          @change="handleCheckboxChange"
        />
      </div>
      <div class="task-content">
        <h4 class="task-title">{{ task.title }}</h4>
        <p v-if="task.description" class="task-description">{{ task.description }}</p>
        <div v-if="task.tags && task.tags.length > 0" class="task-tags">
          <span v-for="tag in task.tags" :key="tag" class="tag">{{ tag }}</span>
        </div>
      </div>
    </div>
    <div class="task-right">
      <div class="task-meta">
        <div class="meta-item">
          <span class="meta-label">优先级</span>
          <span
            class="meta-value priority"
            :style="{ color: priorityColors[task.priority] }"
          >
            {{ priorityLabels[task.priority] }}
          </span>
        </div>
        <div class="meta-item">
          <span class="meta-label">紧急度</span>
          <span
            class="meta-value urgency"
            :style="{ color: urgencyColors[task.urgency] }"
          >
            {{ urgencyLabels[task.urgency] }}
          </span>
        </div>
        <div v-if="dueDateText" class="meta-item">
          <span class="meta-label">截止</span>
          <span class="meta-value due-date" :class="{ overdue: isOverdue }">
            {{ dueDateText }}
            <span v-if="dueTimeText">{{ dueTimeText }}</span>
          </span>
        </div>
        <div v-if="task.estimated_duration" class="meta-item">
          <span class="meta-label">耗时</span>
          <span class="meta-value">{{ task.estimated_duration }}分钟</span>
        </div>
      </div>
      <div class="task-actions">
        <button class="btn-icon" @click.stop="handleEdit">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path>
            <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path>
          </svg>
        </button>
        <button class="btn-icon btn-delete" @click.stop="taskStore.deleteTask(task.id)">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="3 6 5 6 21 6"></polyline>
            <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
          </svg>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.task-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: white;
  border: 1px solid transparent;
  border-radius: 6px;
  margin: 4px 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.task-item:hover {
  background: #fafafa;
  border-color: #e0e0e0;
}

.task-item.is-completed {
  opacity: 0.6;
}

.task-item.is-overdue {
  background: #fff1f0;
  border-color: #ffa39e;
}

.task-item.is-selected {
  background: #e6f7ff;
  border-color: #1890ff;
}

.task-left {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 1;
  min-width: 0;
}

.task-checkbox input,
.task-select input {
  width: 18px;
  height: 18px;
  cursor: pointer;
}

.task-content {
  flex: 1;
  min-width: 0;
}

.task-title {
  font-size: 14px;
  font-weight: 500;
  color: #262626;
  margin: 0 0 4px 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.task-item.is-completed .task-title {
  text-decoration: line-through;
  color: #8c8c8c;
}

.task-description {
  font-size: 13px;
  color: #8c8c8c;
  margin: 0 0 6px 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.task-tags {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}

.tag {
  font-size: 11px;
  padding: 2px 8px;
  background: #f0f0f0;
  color: #595959;
  border-radius: 3px;
}

.task-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.task-meta {
  display: flex;
  gap: 12px;
}

.meta-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
}

.meta-label {
  font-size: 11px;
  color: #8c8c8c;
}

.meta-value {
  font-size: 12px;
  font-weight: 500;
  color: #262626;
}

.meta-value.priority,
.meta-value.urgency {
  font-weight: 600;
}

.meta-value.due-date {
  color: #262626;
}

.meta-value.due-date.overdue {
  color: #ff4d4f;
  font-weight: 600;
}

.task-actions {
  display: flex;
  gap: 4px;
}

.btn-icon {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  border-radius: 4px;
  cursor: pointer;
  color: #8c8c8c;
  transition: all 0.2s;
}

.btn-icon:hover {
  background: #f0f0f0;
  color: #262626;
}

.btn-icon.btn-delete:hover {
  background: #fff1f0;
  color: #ff4d4f;
}
</style>
