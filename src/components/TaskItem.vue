<!--
  TaskItem Component
  
  A single task display component that shows task information with completion controls.
  
  Props:
  - task: Task object containing task details
  
  Features:
  - Shows task title, description, priority, and due date
  - Visual indicators for overdue and completed tasks
  - Checkbox for task completion
  - Edit and delete functionality
  
  Usage:
  <TaskItem :task="task" @edit="handleEdit" @delete="handleDelete" />
-->
<script setup lang="ts">
import { computed } from 'vue'
import { useTaskStore } from '../stores/task'
import { isOverdue, formatDate, formatTime } from '../utils/dateUtils'
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

const taskIsOverdue = computed(() => {
  if (!props.task.due_date || props.task.status === 'completed') return false
  const overdue = isOverdue(props.task.due_date)
  return overdue
})

const dueDateText = computed(() => {
  if (!props.task.due_date) return null
  return formatDate(props.task.due_date)
})

const dueTimeText = computed(() => {
  if (!props.task.due_date) return null
  return formatTime(props.task.due_date)
})

function handleCheckboxChange(e: Event) {
  const checked = (e.target as HTMLInputElement).checked
  console.log('Task selection changed:', props.task.id, checked)
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
      <!-- Complete Checkbox with Icon -->
      <div class="task-checkbox" @click.stop title="✓ 点击标记任务完成">
        <div class="checkbox-wrapper complete-checkbox">
          <input
            type="checkbox"
            :checked="task.status === 'completed'"
            @change="handleToggleComplete"
          />
          <svg v-if="task.status === 'completed'" class="check-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
            <polyline points="20 6 9 17 4 12"></polyline>
          </svg>
        </div>
      </div>
      
      <!-- Select Checkbox with Icon -->
      <div class="task-select" @click.stop title="□ 点击批量选择此任务">
        <div class="checkbox-wrapper select-checkbox" :class="{ 'is-selected': isSelected }">
          <input
            type="checkbox"
            :checked="isSelected"
            @change="handleCheckboxChange"
          />
          <svg v-if="isSelected" class="check-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
            <polyline points="20 6 9 17 4 12"></polyline>
          </svg>
        </div>
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
  padding: 10px 14px;
  background: white;
  border: 1px solid transparent;
  border-radius: 6px;
  margin: 4px 6px;
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
  gap: 10px;
  flex: 1;
  min-width: 0;
}

/* Checkbox Wrapper */
.checkbox-wrapper {
  position: relative;
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s;
  flex-shrink: 0;
}

.checkbox-wrapper input {
  position: absolute;
  width: 100%;
  height: 100%;
  opacity: 0;
  cursor: pointer;
}

.check-icon {
  position: absolute;
  pointer-events: none;
  transition: all 0.2s;
}

/* Complete Checkbox - Green Circle */
.complete-checkbox {
  width: 20px;
  height: 20px;
  border: 2px solid #b7eb8f;
  border-radius: 50%;
  background: white;
  transition: all 0.2s;
}

.complete-checkbox:hover {
  border-color: #52c41a;
  background: #f6ffed;
}

.complete-checkbox.is-checked {
  background: #52c41a;
  border-color: #52c41a;
}

.task-checkbox .check-icon {
  color: white;
  opacity: 0;
  transform: scale(0.5);
  transition: all 0.2s;
}

.task-checkbox:has(input:checked) .check-icon {
  opacity: 1;
  transform: scale(1);
}

.task-checkbox:has(input:checked) .complete-checkbox {
  background: #52c41a;
  border-color: #52c41a;
}

/* Select Checkbox - Blue Square */
.select-checkbox {
  width: 20px;
  height: 20px;
  border: 2px solid #d9d9d9;
  border-radius: 4px;
  background: white;
  transition: all 0.2s;
}

.select-checkbox:hover {
  border-color: #40a9ff;
  background: #e6f7ff;
}

.task-select .check-icon {
  color: white;
  opacity: 0;
  transform: scale(0.5);
  transition: all 0.2s;
}

.task-select:has(input:checked) .check-icon {
  opacity: 1;
  transform: scale(1);
}

.task-select:has(input:checked) .select-checkbox {
  background: #1890ff;
  border-color: #1890ff;
}

.task-content {
  flex: 1;
  min-width: 0;
}

.task-title {
  font-size: 13px;
  font-weight: 500;
  color: #262626;
  margin: 0 0 3px 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.task-item.is-completed .task-title {
  text-decoration: line-through;
  color: #8c8c8c;
}

.task-description {
  font-size: 12px;
  color: #8c8c8c;
  margin: 0 0 5px 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.task-tags {
  display: flex;
  gap: 4px;
  flex-wrap: wrap;
}

.tag {
  font-size: 10px;
  padding: 1px 6px;
  background: #f0f0f0;
  color: #595959;
  border-radius: 3px;
}

.task-right {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-shrink: 0;
}

.task-meta {
  display: flex;
  gap: 10px;
}

.meta-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
}

.meta-label {
  font-size: 10px;
  color: #8c8c8c;
}

.meta-value {
  font-size: 11px;
  font-weight: 500;
  color: #262626;
  white-space: nowrap;
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
  gap: 2px;
}

.btn-icon {
  width: 24px;
  height: 24px;
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

/* Responsive */
@media (max-width: 768px) {
  .task-item {
    padding: 8px 10px;
    margin: 3px 4px;
  }

  .checkbox-wrapper {
    width: 18px;
    height: 18px;
  }

  .complete-checkbox,
  .select-checkbox {
    width: 18px;
    height: 18px;
  }

  .task-title {
    font-size: 12px;
  }

  .task-description {
    font-size: 11px;
  }

  .task-right {
    gap: 8px;
  }

  .task-meta {
    gap: 6px;
  }

  .meta-item {
    align-items: flex-start;
  }

  .meta-label {
    font-size: 9px;
  }

  .meta-value {
    font-size: 10px;
  }

  .btn-icon {
    width: 22px;
    height: 22px;
  }
}

@media (max-width: 480px) {
  .task-item {
    flex-direction: column;
    align-items: flex-start;
    padding: 8px;
  }

  .task-left {
    width: 100%;
  }

  .checkbox-wrapper {
    width: 16px;
    height: 16px;
  }

  .complete-checkbox,
  .select-checkbox {
    width: 16px;
    height: 16px;
  }

  .task-right {
    width: 100%;
    justify-content: space-between;
    margin-top: 8px;
    padding-top: 8px;
    border-top: 1px solid #f0f0f0;
  }

  .task-meta {
    flex-wrap: wrap;
    gap: 8px;
  }

  .task-actions {
    margin-left: auto;
  }
}
</style>
