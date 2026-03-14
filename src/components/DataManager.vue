<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { save, open } from '@tauri-apps/plugin-dialog'
import { readTextFile, writeTextFile } from '@tauri-apps/plugin-fs'

const emit = defineEmits<{
  close: []
}>()

const isLoading = ref(false)
const message = ref<{ type: 'success' | 'error', text: string } | null>(null)
const backups = ref<string[]>([])
const selectedBackup = ref<string | null>(null)

// Export functions
async function exportTasksJSON() {
  try {
    isLoading.value = true
    const data: string = await invoke('export_tasks', { format: 'json' })
    
    const filePath = await save({
      defaultPath: 'tasks.json',
      filters: [{ name: 'JSON Files', extensions: ['json'] }]
    })
    
    if (filePath) {
      await writeTextFile(filePath, data)
      showMessage('success', '任务数据已导出为 JSON 格式')
    }
  } catch (error) {
    showMessage('error', `导出失败: ${error}`)
  } finally {
    isLoading.value = false
  }
}

async function exportTasksCSV() {
  try {
    isLoading.value = true
    const data: string = await invoke('export_tasks', { format: 'csv' })
    
    const filePath = await save({
      defaultPath: 'tasks.csv',
      filters: [{ name: 'CSV Files', extensions: ['csv'] }]
    })
    
    if (filePath) {
      await writeTextFile(filePath, data)
      showMessage('success', '任务数据已导出为 CSV 格式')
    }
  } catch (error) {
    showMessage('error', `导出失败: ${error}`)
  } finally {
    isLoading.value = false
  }
}

// Import function
async function importTasks() {
  try {
    const filePath = await open({
      multiple: false,
      filters: [
        { name: 'JSON Files', extensions: ['json'] },
        { name: 'CSV Files', extensions: ['csv'] }
      ]
    })
    
    if (!filePath) return
    
    isLoading.value = true
    const content = await readTextFile(filePath as string)
    const count = await invoke('import_tasks', { data: content })
    showMessage('success', `成功导入 ${count} 个任务`)
  } catch (error) {
    showMessage('error', `导入失败: ${error}`)
  } finally {
    isLoading.value = false
  }
}

// Backup functions
async function createBackup() {
  try {
    isLoading.value = true
    const backupPath = await invoke('backup_database')
    await fetchBackups()
    showMessage('success', '数据库备份已创建')
  } catch (error) {
    showMessage('error', `备份失败: ${error}`)
  } finally {
    isLoading.value = false
  }
}

async function fetchBackups() {
  try {
    const result = await invoke('list_backups')
    backups.value = result as string[]
  } catch (error) {
    console.error('Failed to fetch backups:', error)
  }
}

async function restoreBackup() {
  if (!selectedBackup.value) return
  
  if (!confirm('确定要恢复此备份吗？当前数据库将被替换。恢复前会自动创建当前数据库的备份。')) {
    return
  }
  
  try {
    isLoading.value = true
    await invoke('restore_database', { backupPath: selectedBackup.value })
    await fetchBackups()
    showMessage('success', '数据库已恢复，请重新加载应用')
  } catch (error) {
    showMessage('error', `恢复失败: ${error}`)
  } finally {
    isLoading.value = false
  }
}

async function deleteBackup() {
  if (!selectedBackup.value) return
  
  if (!confirm('确定要删除此备份吗？')) return
  
  try {
    isLoading.value = true
    await invoke('delete_backup', { backupPath: selectedBackup.value })
    await fetchBackups()
    selectedBackup.value = null
    showMessage('success', '备份已删除')
  } catch (error) {
    showMessage('error', `删除失败: ${error}`)
  } finally {
    isLoading.value = false
  }
}

function getBackupName(path: string): string {
  return path.split('\\').pop()?.split('/').pop() || path
}

function getBackupDate(path: string): string {
  const match = path.match(/backup_(\d{8}_\d{6})\.db/)
  if (!match) return ''
  
  const [datePart, timePart] = match[1].split('_')
  const year = datePart.substring(0, 4)
  const month = datePart.substring(4, 6)
  const day = datePart.substring(6, 8)
  const hour = timePart.substring(0, 2)
  const minute = timePart.substring(2, 4)
  const second = timePart.substring(4, 6)
  
  return `${year}-${month}-${day} ${hour}:${minute}:${second}`
}

function showMessage(type: 'success' | 'error', text: string) {
  message.value = { type, text }
  setTimeout(() => {
    message.value = null
  }, 3000)
}

onMounted(() => {
  fetchBackups()
})
</script>

<template>
  <div class="data-manager">
    <div class="manager-header">
      <h2>数据管理</h2>
      <button class="btn-close" @click="emit('close')">×</button>
    </div>

    <!-- Message -->
    <div v-if="message" class="message" :class="message.type">
      {{ message.text }}
    </div>

    <div class="manager-content">
      <!-- Export/Import Section -->
      <div class="section">
        <h3 class="section-title">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
            <polyline points="7 10 12 15 17 10"></polyline>
            <line x1="12" y1="15" x2="12" y2="3"></line>
          </svg>
          导出/导入
        </h3>
        <div class="action-grid">
          <button class="action-card" @click="exportTasksJSON" :disabled="isLoading">
            <div class="card-icon json">JSON</div>
            <div class="card-label">导出为 JSON</div>
            <div class="card-desc">导出所有任务数据为 JSON 格式</div>
          </button>
          
          <button class="action-card" @click="exportTasksCSV" :disabled="isLoading">
            <div class="card-icon csv">CSV</div>
            <div class="card-label">导出为 CSV</div>
            <div class="card-desc">导出所有任务数据为 CSV 格式</div>
          </button>
          
          <button class="action-card" @click="importTasks" :disabled="isLoading">
            <div class="card-icon import">
              <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
                <polyline points="17 8 12 3 7 8"></polyline>
                <line x1="12" y1="3" x2="12" y2="15"></line>
              </svg>
            </div>
            <div class="card-label">导入数据</div>
            <div class="card-desc">从 JSON 或 CSV 文件导入任务</div>
          </button>
        </div>
      </div>

      <!-- Backup Section -->
      <div class="section">
        <h3 class="section-title">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"></path>
            <polyline points="17 21 17 13 7 13 7 21"></polyline>
            <polyline points="7 3 7 8 15 8"></polyline>
          </svg>
          数据库备份
        </h3>
        <div class="backup-actions">
          <button class="btn-primary" @click="createBackup" :disabled="isLoading">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"></path>
              <polyline points="17 21 17 13 7 13 7 21"></polyline>
              <polyline points="7 3 7 8 15 8"></polyline>
            </svg>
            创建备份
          </button>
        </div>

        <!-- Backups List -->
        <div v-if="backups.length > 0" class="backups-list">
          <div
            v-for="backup in backups"
            :key="backup"
            class="backup-item"
            :class="{ selected: selectedBackup === backup }"
            @click="selectedBackup = backup"
          >
            <div class="backup-info">
              <div class="backup-name">{{ getBackupName(backup) }}</div>
              <div class="backup-date">{{ getBackupDate(backup) }}</div>
            </div>
            <div class="backup-actions">
              <button
                v-if="selectedBackup === backup"
                class="btn-restore"
                @click.stop="restoreBackup"
                :disabled="isLoading"
              >
                恢复
              </button>
              <button
                v-if="selectedBackup === backup"
                class="btn-delete"
                @click.stop="deleteBackup"
                :disabled="isLoading"
              >
                删除
              </button>
            </div>
          </div>
        </div>

        <div v-else class="empty-state">
          <span class="empty-icon">💾</span>
          <p>暂无备份</p>
          <p class="empty-hint">点击"创建备份"按钮创建第一个备份</p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.data-manager {
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

.manager-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px;
  border-bottom: 1px solid #f0f0f0;
}

.manager-header h2 {
  font-size: 20px;
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

.message {
  padding: 12px 20px;
  margin: 0 20px;
  border-radius: 6px;
  font-size: 14px;
}

.message.success {
  background: #f6ffed;
  color: #52c41a;
  border: 1px solid #b7eb8f;
}

.message.error {
  background: #fff1f0;
  color: #ff4d4f;
  border: 1px solid #ffa39e;
}

.manager-content {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.section {
  background: white;
  border-radius: 8px;
  padding: 24px;
  margin-bottom: 20px;
}

.section:last-child {
  margin-bottom: 0;
}

.section-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 16px;
  font-weight: 600;
  color: #262626;
  margin: 0 0 20px 0;
}

.action-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  gap: 16px;
}

.action-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 24px 20px;
  background: #fafafa;
  border: 2px solid #f0f0f0;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.action-card:hover:not(:disabled) {
  border-color: #1890ff;
  background: #e6f7ff;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(24, 144, 255, 0.15);
}

.action-card:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.card-icon {
  width: 64px;
  height: 64px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 12px;
  font-size: 20px;
  font-weight: 700;
  margin-bottom: 12px;
}

.card-icon.json {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.card-icon.csv {
  background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
  color: white;
}

.card-icon.import {
  background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
  color: white;
}

.card-label {
  font-size: 15px;
  font-weight: 600;
  color: #262626;
  margin-bottom: 4px;
}

.card-desc {
  font-size: 13px;
  color: #8c8c8c;
  text-align: center;
  line-height: 1.5;
}

.backup-actions {
  margin-bottom: 20px;
}

.btn-primary {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  width: 100%;
  padding: 12px 24px;
  background: #1890ff;
  border: none;
  border-radius: 6px;
  color: white;
  font-size: 15px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-primary:hover:not(:disabled) {
  background: #40a9ff;
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(24, 144, 255, 0.3);
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.backups-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.backup-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: #fafafa;
  border: 2px solid #f0f0f0;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
}

.backup-item:hover {
  border-color: #d9d9d9;
  background: white;
}

.backup-item.selected {
  border-color: #1890ff;
  background: #e6f7ff;
}

.backup-info {
  flex: 1;
}

.backup-name {
  font-size: 14px;
  font-weight: 500;
  color: #262626;
  margin-bottom: 4px;
}

.backup-date {
  font-size: 12px;
  color: #8c8c8c;
}

.backup-actions {
  display: flex;
  gap: 8px;
}

.btn-restore {
  padding: 6px 12px;
  background: #52c41a;
  border: none;
  border-radius: 4px;
  color: white;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-restore:hover:not(:disabled) {
  background: #73d13d;
}

.btn-delete {
  padding: 6px 12px;
  background: #ff4d4f;
  border: none;
  border-radius: 4px;
  color: white;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-delete:hover:not(:disabled) {
  background: #ff7875;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px 20px;
  text-align: center;
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 12px;
}

.empty-state p {
  font-size: 14px;
  margin: 4px 0;
  color: #8c8c8c;
}

.empty-hint {
  font-size: 13px;
}
</style>
