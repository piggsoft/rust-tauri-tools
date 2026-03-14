<script setup lang="ts">
import { ref, watch } from 'vue'
import { useTaskStore } from '../stores/task'
import SubtaskList from './SubtaskList.vue'
import type { Task, TaskInput } from '../types'

const props = defineProps<{
  task?: Task | null
}>()

const emit = defineEmits<{
  close: []
  saved: []
  deleted: []
}>()

const taskStore = useTaskStore()

const form = ref<TaskInput>({
  title: '',
  description: '',
  priority: 'medium',
  urgency: 'medium',
  tags: [],
  start_date: undefined,
  due_date: undefined,
  reminder_time: undefined,
  estimated_duration: undefined,
  repeat_pattern: undefined,
  repeat_until: undefined,
})

const tagInput = ref('')
const showSubtasks = ref(false)

watch(() => props.task, (task) => {
  if (task) {
    form.value = {
      title: task.title,
      description: task.description,
      priority: task.priority,
      urgency: task.urgency,
      tags: task.tags || [],
      start_date: task.start_date,
      due_date: task.due_date,
      reminder_time: task.reminder_time,
      estimated_duration: task.estimated_duration,
      repeat_pattern: task.repeat_pattern,
      repeat_until: task.repeat_until,
    }
  }
}, { immediate: true })

function addTag() {
  const tag = tagInput.value.trim()
  if (tag && !form.value.tags?.includes(tag)) {
    form.value.tags = [...(form.value.tags || []), tag]
  }
  tagInput.value = ''
}

function removeTag(tag: string) {
  form.value.tags = form.value.tags?.filter(t => t !== tag)
}

async function handleSubmit() {
  try {
    if (props.task) {
      await taskStore.updateTask(props.task.id, form.value)
    } else {
      await taskStore.createTask(form.value)
    }
    emit('saved')
  } catch (error) {
    console.error('Failed to save task:', error)
  }
}

async function handleDelete() {
  if (props.task && confirm('确定要删除这个任务吗？')) {
    try {
      await taskStore.deleteTask(props.task.id)
      emit('deleted')
    } catch (error) {
      console.error('Failed to delete task:', error)
    }
  }
}

function handleCancel() {
  emit('close')
}

function setQuadrant(quadrant: string) {
  switch (quadrant) {
    case 'first':
      form.value.priority = 'high'
      form.value.urgency = 'high'
      break
    case 'second':
      form.value.priority = 'high'
      form.value.urgency = 'medium'
      break
    case 'third':
      form.value.priority = 'medium'
      form.value.urgency = 'high'
      break
    case 'fourth':
      form.value.priority = 'medium'
      form.value.urgency = 'medium'
      break
  }
}

const quadrants = [
  { id: 'first', name: '第一象限', desc: '重要且紧急', color: '#ff4d4f' },
  { id: 'second', name: '第二象限', desc: '重要但不紧急', color: '#faad14' },
  { id: 'third', name: '第三象限', desc: '不重要但紧急', color: '#1890ff' },
  { id: 'fourth', name: '第四象限', desc: '不重要且不紧急', color: '#52c41a' },
]

</script>

<template>
  <div class="modal-overlay" @click="handleCancel">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h2>{{ task ? '编辑任务' : '新建任务' }}</h2>
        <button class="btn-close" @click="handleCancel">×</button>
      </div>

      <div class="modal-body">
        <form @submit.prevent="handleSubmit">
          <div class="form-group">
            <label class="form-label">标题 *</label>
            <input
              v-model="form.title"
              type="text"
              class="form-input"
              placeholder="任务标题"
              required
            />
          </div>

          <div class="form-group">
            <label class="form-label">描述</label>
            <textarea
              v-model="form.description"
              class="form-textarea"
              placeholder="任务描述"
              rows="3"
            />
          </div>

          <div class="form-row">
            <div class="form-group">
              <label class="form-label">优先级</label>
              <select v-model="form.priority" class="form-select">
                <option value="high">高</option>
                <option value="medium">中</option>
                <option value="low">低</option>
              </select>
            </div>

            <div class="form-group">
              <label class="form-label">紧急度</label>
              <select v-model="form.urgency" class="form-select">
                <option value="high">紧急</option>
                <option value="medium">一般</option>
                <option value="low">不急</option>
              </select>
            </div>
          </div>

          <div class="form-group">
            <label class="form-label">四象限快速设置</label>
            <div class="quadrant-selector">
              <button
                v-for="quadrant in quadrants"
                :key="quadrant.id"
                type="button"
                class="quadrant-btn"
                :class="{ active: form.priority === 'high' && form.urgency === 'high' && quadrant.id === 'first' }"
                :style="{ '--quadrant-color': quadrant.color }"
                @click="setQuadrant(quadrant.id)"
              >
                <div class="quadrant-icon">{{ quadrant.id === 'first' ? '🔥' : quadrant.id === 'second' ? '📋' : quadrant.id === 'third' ? '⏰' : '🗑️' }}</div>
                <div class="quadrant-info">
                  <div class="quadrant-name">{{ quadrant.name }}</div>
                  <div class="quadrant-desc">{{ quadrant.desc }}</div>
                </div>
              </button>
            </div>
          </div>


          <div class="form-row">
            <div class="form-group">
              <label class="form-label">开始日期</label>
              <input
                v-model="form.start_date"
                type="date"
                class="form-input"
              />
            </div>

            <div class="form-group">
              <label class="form-label">截止日期</label>
              <input
                v-model="form.due_date"
                type="date"
                class="form-input"
              />
            </div>
          </div>

          <div class="form-row">
            <div class="form-group">
              <label class="form-label">提醒时间</label>
              <input
                v-model="form.reminder_time"
                type="datetime-local"
                class="form-input"
              />
            </div>

            <div class="form-group">
              <label class="form-label">预估耗时（分钟）</label>
              <input
                v-model.number="form.estimated_duration"
                type="number"
                class="form-input"
                min="1"
              />
            </div>
          </div>

          <div class="form-group">
            <label class="form-label">标签</label>
            <div class="tag-input-group">
              <input
                v-model="tagInput"
                type="text"
                class="form-input"
                placeholder="输入标签后按回车添加"
                @keyup.enter="addTag"
              />
              <button type="button" class="btn-add-tag" @click="addTag">添加</button>
            </div>
            <div class="tag-list">
              <span v-for="tag in form.tags" :key="tag" class="tag">
                {{ tag }}
                <button type="button" class="tag-remove" @click="removeTag(tag)">×</button>
              </span>
            </div>
          </div>

          <div class="form-row">
            <div class="form-group">
              <label class="form-label">重复模式</label>
              <select v-model="form.repeat_pattern" class="form-select">
                <option :value="undefined">不重复</option>
                <option value="daily">每天</option>
                <option value="weekly">每周</option>
                <option value="monthly">每月</option>
              </select>
            </div>

            <div class="form-group">
              <label class="form-label">重复直到</label>
              <input
                v-model="form.repeat_until"
                type="date"
                class="form-input"
              />
            </div>
          </div>
        </form>
      </div>

      <!-- Subtasks Section (only for editing existing tasks) -->
      <div v-if="task" class="subtasks-section">
        <button
          class="subtasks-toggle"
          @click="showSubtasks = !showSubtasks"
        >
          <span>子任务</span>
          <svg
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            :class="{ rotated: showSubtasks }"
          >
            <polyline points="6 9 12 15 18 9"></polyline>
          </svg>
        </button>
        <div v-show="showSubtasks" class="subtasks-content">
          <SubtaskList :task-id="task.id" />
        </div>
      </div>


      <div class="modal-footer">
        <button type="button" class="btn-secondary" @click="handleCancel">取消</button>
        <button v-if="task" type="button" class="btn-danger" @click="handleDelete">删除</button>
        <button type="button" class="btn-primary" @click="handleSubmit">保存</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 20px;
}

.modal-content {
  background: white;
  border-radius: 8px;
  width: 100%;
  max-width: 600px;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid #f0f0f0;
}

.modal-header h2 {
  font-size: 18px;
  font-weight: 600;
  margin: 0;
  color: #262626;
}

.btn-close {
  width: 32px;
  height: 32px;
  border: none;
  background: transparent;
  font-size: 24px;
  color: #8c8c8c;
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.2s;
}

.btn-close:hover {
  background: #f0f0f0;
  color: #262626;
}

.modal-body {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.form-group {
  margin-bottom: 16px;
}

.form-label {
  display: block;
  font-size: 14px;
  font-weight: 500;
  color: #262626;
  margin-bottom: 6px;
}

.form-input,
.form-textarea,
.form-select {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  font-size: 14px;
  transition: border-color 0.2s;
}

.form-input:focus,
.form-textarea:focus,
.form-select:focus {
  outline: none;
  border-color: #1890ff;
}

.form-textarea {
  resize: vertical;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
}

.tag-input-group {
  display: flex;
  gap: 8px;
}

.tag-input-group .form-input {
  flex: 1;
}

.btn-add-tag {
  padding: 8px 16px;
  background: #f0f0f0;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  color: #595959;
  transition: all 0.2s;
}

.btn-add-tag:hover {
  background: #e0e0e0;
  border-color: #bfbfbf;
}

.tag-list {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-top: 8px;
}

.tag {
  font-size: 12px;
  padding: 4px 8px 4px 12px;
  background: #f0f0f0;
  color: #595959;
  border-radius: 12px;
  display: flex;
  align-items: center;
  gap: 6px;
}

.tag-remove {
  width: 16px;
  height: 16px;
  border: none;
  background: transparent;
  color: #8c8c8c;
  cursor: pointer;
  font-size: 14px;
  line-height: 1;
  padding: 0;
}

.tag-remove:hover {
  color: #262626;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 20px;
  border-top: 1px solid #f0f0f0;
}

.btn-secondary,
.btn-primary,
.btn-danger {
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

.btn-primary:hover {
  background: #40a9ff;
  border-color: #40a9ff;
}

.btn-danger {
  background: #ff4d4f;
  border: 1px solid #ff4d4f;
  color: white;
}

.btn-danger:hover {
  background: #ff7875;
  border-color: #ff7875;
}

.quadrant-selector {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

.quadrant-btn {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  border: 2px solid #f0f0f0;
  background: white;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.quadrant-btn:hover {
  border-color: var(--quadrant-color);
  background: rgba(var(--quadrant-color), 0.05);
}

.quadrant-btn.active {
  border-color: var(--quadrant-color);
  background: rgba(var(--quadrant-color), 0.1);
}

.quadrant-icon {
  font-size: 24px;
}

.quadrant-info {
  text-align: left;
}

.quadrant-name {
  font-size: 14px;
  font-weight: 600;
  color: #262626;
  margin-bottom: 2px;
}

.quadrant-desc {
  font-size: 12px;
  color: #8c8c8c;
}

.subtasks-section {
  border-top: 1px solid #f0f0f0;
}

.subtasks-toggle {
  width: 100%;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  background: #fafafa;
  border: none;
  border-bottom: 1px solid #f0f0f0;
  cursor: pointer;
  font-size: 15px;
  font-weight: 600;
  color: #262626;
  transition: background 0.2s;
}

.subtasks-toggle:hover {
  background: #f0f0f0;
}

.subtasks-toggle svg {
  transition: transform 0.2s;
}

.subtasks-toggle svg.rotated {
  transform: rotate(180deg);
}

.subtasks-content {
  padding: 16px 20px;
  background: white;
}


</style>
