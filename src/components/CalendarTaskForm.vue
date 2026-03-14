<script setup lang="ts">
import { ref } from 'vue'
import { useTaskStore } from '../stores/task'

const props = defineProps<{
  date: Date
}>()

const emit = defineEmits<{
  close: []
  created: []
}>()

const taskStore = useTaskStore()

const title = ref('')
const priority = ref<'high' | 'medium' | 'low'>('medium')
const urgency = ref<'high' | 'medium' | 'low'>('medium')

const dateStr = props.date.toISOString().split('T')[0]

async function handleCreate() {
  if (!title.value.trim()) return

  try {
    await taskStore.createTask({
      title: title.value,
      priority: priority.value,
      urgency: urgency.value,
      due_date: dateStr,
    })
    emit('created')
    emit('close')
  } catch (error) {
    console.error('Failed to create task:', error)
  }
}

function handleCancel() {
  emit('close')
}
</script>

<template>
  <div class="calendar-task-form">
    <div class="form-header">
      <h3>新建任务 - {{ date }}</h3>
      <button class="btn-close" @click="handleCancel">×</button>
    </div>
    <div class="form-body">
      <div class="form-group">
        <label>任务标题</label>
        <input
          v-model="title"
          type="text"
          placeholder="输入任务标题..."
          @keyup.enter="handleCreate"
        />
      </div>
      <div class="form-row">
        <div class="form-group">
          <label>优先级</label>
          <select v-model="priority">
            <option value="high">高</option>
            <option value="medium">中</option>
            <option value="low">低</option>
          </select>
        </div>
        <div class="form-group">
          <label>紧急度</label>
          <select v-model="urgency">
            <option value="high">紧急</option>
            <option value="medium">一般</option>
            <option value="low">不急</option>
          </select>
        </div>
      </div>
    </div>
    <div class="form-footer">
      <button class="btn-secondary" @click="handleCancel">取消</button>
      <button class="btn-primary" @click="handleCreate" :disabled="!title.trim()">
        创建
      </button>
    </div>
  </div>
</template>

<style scoped>
.calendar-task-form {
  background: white;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  max-width: 400px;
}

.form-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid #f0f0f0;
}

.form-header h3 {
  font-size: 16px;
  font-weight: 600;
  margin: 0;
  color: #262626;
}

.btn-close {
  width: 28px;
  height: 28px;
  border: none;
  background: transparent;
  font-size: 20px;
  color: #8c8c8c;
  cursor: pointer;
  border-radius: 4px;
}

.btn-close:hover {
  background: #f0f0f0;
}

.form-body {
  padding: 20px;
}

.form-group {
  margin-bottom: 16px;
}

.form-group:last-child {
  margin-bottom: 0;
}

.form-group label {
  display: block;
  font-size: 14px;
  font-weight: 500;
  color: #262626;
  margin-bottom: 6px;
}

.form-group input,
.form-group select {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  font-size: 14px;
}

.form-group input:focus,
.form-group select:focus {
  outline: none;
  border-color: #1890ff;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
}

.form-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 20px;
  border-top: 1px solid #f0f0f0;
}

.btn-secondary,
.btn-primary {
  padding: 8px 20px;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-secondary {
  background: white;
  border: 1px solid #d9d9d9;
  color: #595959;
}

.btn-secondary:hover {
  border-color: #40a9ff;
  color: #40a9ff;
}

.btn-primary {
  background: #1890ff;
  border: 1px solid #1890ff;
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: #40a9ff;
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
