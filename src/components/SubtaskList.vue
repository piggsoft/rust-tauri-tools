<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useTaskStore } from '../stores/task'
import type { Subtask } from '../types'

const props = defineProps<{
  taskId: number
}>()

const emit = defineEmits<{
  progressUpdate: [completed: number, total: number]
}>()

const taskStore = useTaskStore()

const subtasks = ref<Subtask[]>([])
const newSubtaskTitle = ref('')
const editingSubtask = ref<number | null>(null)
const editingTitle = ref('')

const completedCount = computed(() => subtasks.value.filter(s => s.status === 'completed').length)
const totalCount = computed(() => subtasks.value.length)
const progress = computed(() => totalCount.value === 0 ? 0 : (completedCount.value / totalCount.value) * 100)

async function fetchSubtasks() {
  try {
    subtasks.value = await taskStore.getSubtasks(props.taskId)
    emit('progressUpdate', completedCount.value, totalCount.value)
  } catch (error) {
    console.error('Failed to fetch subtasks:', error)
  }
}

async function createSubtask() {
  if (!newSubtaskTitle.value.trim()) return

  try {
    await taskStore.createSubtask(props.taskId, newSubtaskTitle.value)
    newSubtaskTitle.value = ''
    await fetchSubtasks()
  } catch (error) {
    console.error('Failed to create subtask:', error)
  }
}

async function updateSubtaskStatus(id: number, status: 'pending' | 'completed') {
  try {
    await taskStore.updateSubtask(id, status)
    await fetchSubtasks()
  } catch (error) {
    console.error('Failed to update subtask:', error)
  }
}

function startEdit(subtask: Subtask) {
  editingSubtask.value = subtask.id
  editingTitle.value = subtask.title
}

async function saveEdit() {
  if (!editingSubtask.value || !editingTitle.value.trim()) return

  try {
    await taskStore.updateSubtask(editingSubtask.value, editingTitle.value)
    editingSubtask.value = null
    editingTitle.value = ''
    await fetchSubtasks()
  } catch (error) {
    console.error('Failed to update subtask:', error)
  }
}

function cancelEdit() {
  editingSubtask.value = null
  editingTitle.value = ''
}

async function deleteSubtask(id: number) {
  if (!confirm('确定要删除这个子任务吗？')) return

  try {
    await taskStore.deleteSubtask(id)
    await fetchSubtasks()
  } catch (error) {
    console.error('Failed to delete subtask:', error)
  }
}

onMounted(() => {
  fetchSubtasks()
})
</script>

<template>
  <div class="subtask-list">
    <!-- Progress Bar -->
    <div class="progress-section">
      <div class="progress-info">
        <span class="progress-text">进度：{{ completedCount }}/{{ totalCount }}</span>
        <span class="progress-percent">{{ Math.round(progress) }}%</span>
      </div>
      <div class="progress-bar">
        <div class="progress-fill" :style="{ width: `${progress}%` }"></div>
      </div>
    </div>

    <!-- Add Subtask -->
    <div class="add-subtask">
      <input
        v-model="newSubtaskTitle"
        type="text"
        placeholder="添加子任务..."
        class="subtask-input"
        @keyup.enter="createSubtask"
      />
      <button
        class="btn-add"
        @click="createSubtask"
        :disabled="!newSubtaskTitle.trim()"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="12" y1="5" x2="12" y2="19"></line>
          <line x1="5" y1="12" x2="19" y2="12"></line>
        </svg>
        添加
      </button>
    </div>

    <!-- Subtasks -->
    <div class="subtasks">
      <div
        v-for="subtask in subtasks"
        :key="subtask.id"
        class="subtask-item"
        :class="{
          'completed': subtask.status === 'completed',
          'editing': editingSubtask === subtask.id
        }"
      >
        <div class="subtask-left">
          <input
            type="checkbox"
            :checked="subtask.status === 'completed'"
            @change="updateSubtaskStatus(
              subtask.id,
              subtask.status === 'completed' ? 'pending' : 'completed'
            )"
            class="subtask-checkbox"
          />
          <div class="subtask-content">
            <input
              v-if="editingSubtask === subtask.id"
              v-model="editingTitle"
              type="text"
              class="subtask-edit-input"
              @keyup.enter="saveEdit"
              @keyup.esc="cancelEdit"
              @blur="saveEdit"
              ref="editInput"
            />
            <span v-else class="subtask-title">{{ subtask.title }}</span>
          </div>
        </div>
        <div class="subtask-actions">
          <button
            v-if="editingSubtask !== subtask.id"
            class="btn-icon"
            @click="startEdit(subtask)"
            title="编辑"
          >
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path>
              <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path>
            </svg>
          </button>
          <button
            v-if="editingSubtask !== subtask.id"
            class="btn-icon btn-delete"
            @click="deleteSubtask(subtask.id)"
            title="删除"
          >
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="3 6 5 6 21 6"></polyline>
              <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
            </svg>
          </button>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-if="subtasks.length === 0" class="empty-state">
      <span class="empty-icon">📝</span>
      <p>暂无子任务</p>
      <p class="empty-hint">添加子任务来更好地跟踪任务进度</p>
    </div>
  </div>
</template>

<style scoped>
.subtask-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.progress-section {
  padding: 16px;
  background: #fafafa;
  border-radius: 8px;
}

.progress-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.progress-text {
  font-size: 14px;
  color: #595959;
}

.progress-percent {
  font-size: 16px;
  font-weight: 600;
  color: #1890ff;
}

.progress-bar {
  height: 8px;
  background: #e0e0e0;
  border-radius: 4px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #1890ff 0%, #40a9ff 100%);
  transition: width 0.3s ease;
  border-radius: 4px;
}

.add-subtask {
  display: flex;
  gap: 8px;
}

.subtask-input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  font-size: 14px;
}

.subtask-input:focus {
  outline: none;
  border-color: #1890ff;
}

.btn-add {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 8px 16px;
  background: #1890ff;
  border: none;
  border-radius: 4px;
  color: white;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-add:hover:not(:disabled) {
  background: #40a9ff;
}

.btn-add:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.subtasks {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.subtask-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 12px;
  background: white;
  border: 1px solid #f0f0f0;
  border-radius: 6px;
  transition: all 0.2s;
}

.subtask-item:hover {
  border-color: #e0e0e0;
  box-shadow: 0 2px 4px rgba(0,0,0,0.05);
}

.subtask-item.completed {
  opacity: 0.6;
}

.subtask-item.completed .subtask-title {
  text-decoration: line-through;
  color: #8c8c8c;
}

.subtask-left {
  display: flex;
  align-items: center;
  gap: 10px;
  flex: 1;
  min-width: 0;
}

.subtask-checkbox {
  width: 18px;
  height: 18px;
  cursor: pointer;
  flex-shrink: 0;
}

.subtask-content {
  flex: 1;
  min-width: 0;
}

.subtask-title {
  font-size: 14px;
  color: #262626;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.subtask-edit-input {
  width: 100%;
  padding: 4px 8px;
  border: 1px solid #1890ff;
  border-radius: 4px;
  font-size: 14px;
}

.subtask-edit-input:focus {
  outline: none;
}

.subtask-actions {
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

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px 20px;
  color: #8c8c8c;
  text-align: center;
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 12px;
}

.empty-state p {
  font-size: 14px;
  margin: 4px 0;
}

.empty-hint {
  font-size: 13px;
}
</style>
