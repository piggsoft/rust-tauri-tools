<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useTaskStore } from '../stores/task'
import type { TaskFilter } from '../types'

const emit = defineEmits<{
  filterChange: [filters: TaskFilter]
}>()

const taskStore = useTaskStore()

const isOpen = ref(false)
const searchQuery = ref('')
const selectedStatuses = ref<string[]>(['pending'])
const selectedPriorities = ref<string[]>([])
const selectedUrgencies = ref<string[]>([])
const selectedTags = ref<string[]>([])
const dateStart = ref('')
const dateEnd = ref('')

// Get all available tags from tasks
const availableTags = computed(() => {
  const tagSet = new Set<string>()
  taskStore.tasks.forEach(task => {
    task.tags?.forEach(tag => tagSet.add(tag))
  })
  return Array.from(tagSet).sort()
})

// Build filter object
const currentFilter = computed((): TaskFilter => ({
  search: searchQuery.value || undefined,
  tags: selectedTags.value.length > 0 ? selectedTags.value : undefined,
  status: selectedStatuses.value.length > 0 ? selectedStatuses.value : undefined,
  priority: selectedPriorities.value.length > 0 ? selectedPriorities.value : undefined,
  urgency: selectedUrgencies.value.length > 0 ? selectedUrgencies.value : undefined,
  date_start: dateStart.value || undefined,
  date_end: dateEnd.value || undefined,
}))

// Emit filter changes
watch(currentFilter, (filter) => {
  emit('filterChange', filter)
}, { deep: true })

// Quick filter options
const quickFilters = [
  { id: 'pending', label: '待完成', icon: '⏳', status: ['pending'] },
  { id: 'completed', label: '已完成', icon: '✅', status: ['completed'] },
  { id: 'overdue', label: '已逾期', icon: '⚠️', status: ['pending'] },
  { id: 'all', label: '全部', icon: '📋', status: ['pending', 'completed', 'archived'] },
]

function applyQuickFilter(filter: typeof quickFilters[0]) {
  selectedStatuses.value = [...filter.status]
}

function toggleStatus(status: string) {
  const index = selectedStatuses.value.indexOf(status)
  if (index === -1) {
    selectedStatuses.value.push(status)
  } else {
    selectedStatuses.value.splice(index, 1)
  }
}

function togglePriority(priority: string) {
  const index = selectedPriorities.value.indexOf(priority)
  if (index === -1) {
    selectedPriorities.value.push(priority)
  } else {
    selectedPriorities.value.splice(index, 1)
  }
}

function toggleUrgency(urgency: string) {
  const index = selectedUrgencies.value.indexOf(urgency)
  if (index === -1) {
    selectedUrgencies.value.push(urgency)
  } else {
    selectedUrgencies.value.splice(index, 1)
  }
}

function toggleTag(tag: string) {
  const index = selectedTags.value.indexOf(tag)
  if (index === -1) {
    selectedTags.value.push(tag)
  } else {
    selectedTags.value.splice(index, 1)
  }
}

function resetFilters() {
  searchQuery.value = ''
  selectedStatuses.value = ['pending']
  selectedPriorities.value = []
  selectedUrgencies.value = []
  selectedTags.value = []
  dateStart.value = ''
  dateEnd.value = ''
}

function getActiveFilterCount() {
  let count = 0
  if (searchQuery.value) count++
  if (selectedStatuses.value.length > 0 && selectedStatuses.value.join() !== 'pending') count++
  if (selectedPriorities.value.length > 0) count++
  if (selectedUrgencies.value.length > 0) count++
  if (selectedTags.value.length > 0) count++
  if (dateStart.value || dateEnd.value) count++
  return count
}
</script>

<template>
  <div class="search-filter-panel">
    <!-- Search Bar -->
    <div class="search-bar">
      <svg class="search-icon" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="11" cy="11" r="8"></circle>
        <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
      </svg>
      <input
        v-model="searchQuery"
        type="text"
        class="search-input"
        placeholder="搜索任务标题或描述..."
      />
      <button
        v-if="searchQuery"
        class="btn-clear-search"
        @click="searchQuery = ''"
        title="清除搜索"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="18" y1="6" x2="6" y2="18"></line>
          <line x1="6" y1="6" x2="18" y2="18"></line>
        </svg>
      </button>
    </div>

    <!-- Filter Toggle -->
    <button class="filter-toggle" @click="isOpen = !isOpen">
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <polygon points="22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3"></polygon>
      </svg>
      <span>筛选</span>
      <span v-if="getActiveFilterCount() > 0" class="filter-badge">{{ getActiveFilterCount() }}</span>
      <svg
        class="chevron"
        :class="{ rotated: isOpen }"
        width="16" height="16"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
      >
        <polyline points="6 9 12 15 18 9"></polyline>
      </svg>
    </button>

    <!-- Filter Panel -->
    <div v-if="isOpen" class="filter-panel">
      <!-- Quick Filters -->
      <div class="filter-section">
        <h3 class="section-title">快速筛选</h3>
        <div class="quick-filters">
          <button
            v-for="filter in quickFilters"
            :key="filter.id"
            class="quick-filter-btn"
            :class="{ active: selectedStatuses.join() === filter.status.join() }"
            @click="applyQuickFilter(filter)"
          >
            <span class="quick-filter-icon">{{ filter.icon }}</span>
            <span>{{ filter.label }}</span>
          </button>
        </div>
      </div>

      <!-- Status Filter -->
      <div class="filter-section">
        <h3 class="section-title">状态</h3>
        <div class="checkbox-group">
          <label class="checkbox-label">
            <input
              type="checkbox"
              :checked="selectedStatuses.includes('pending')"
              @change="toggleStatus('pending')"
            />
            <span>待完成</span>
          </label>
          <label class="checkbox-label">
            <input
              type="checkbox"
              :checked="selectedStatuses.includes('completed')"
              @change="toggleStatus('completed')"
            />
            <span>已完成</span>
          </label>
          <label class="checkbox-label">
            <input
              type="checkbox"
              :checked="selectedStatuses.includes('archived')"
              @change="toggleStatus('archived')"
            />
            <span>已归档</span>
          </label>
        </div>
      </div>

      <!-- Priority Filter -->
      <div class="filter-section">
        <h3 class="section-title">优先级</h3>
        <div class="checkbox-group">
          <label class="checkbox-label">
            <input
              type="checkbox"
              :checked="selectedPriorities.includes('high')"
              @change="togglePriority('high')"
            />
            <span class="priority-badge high">高</span>
          </label>
          <label class="checkbox-label">
            <input
              type="checkbox"
              :checked="selectedPriorities.includes('medium')"
              @change="togglePriority('medium')"
            />
            <span class="priority-badge medium">中</span>
          </label>
          <label class="checkbox-label">
            <input
              type="checkbox"
              :checked="selectedPriorities.includes('low')"
              @change="togglePriority('low')"
            />
            <span class="priority-badge low">低</span>
          </label>
        </div>
      </div>

      <!-- Urgency Filter -->
      <div class="filter-section">
        <h3 class="section-title">紧急度</h3>
        <div class="checkbox-group">
          <label class="checkbox-label">
            <input
              type="checkbox"
              :checked="selectedUrgencies.includes('high')"
              @change="toggleUrgency('high')"
            />
            <span>紧急</span>
          </label>
          <label class="checkbox-label">
            <input
              type="checkbox"
              :checked="selectedUrgencies.includes('medium')"
              @change="toggleUrgency('medium')"
            />
            <span>一般</span>
          </label>
          <label class="checkbox-label">
            <input
              type="checkbox"
              :checked="selectedUrgencies.includes('low')"
              @change="toggleUrgency('low')"
            />
            <span>不急</span>
          </label>
        </div>
      </div>

      <!-- Tags Filter -->
      <div v-if="availableTags.length > 0" class="filter-section">
        <h3 class="section-title">标签</h3>
        <div class="tags-group">
          <button
            v-for="tag in availableTags"
            :key="tag"
            class="tag-btn"
            :class="{ active: selectedTags.includes(tag) }"
            @click="toggleTag(tag)"
          >
            {{ tag }}
          </button>
        </div>
      </div>

      <!-- Date Range Filter -->
      <div class="filter-section">
        <h3 class="section-title">日期范围</h3>
        <div class="date-inputs">
          <div class="date-input-group">
            <label>开始日期</label>
            <input
              v-model="dateStart"
              type="date"
              class="date-input"
            />
          </div>
          <div class="date-input-group">
            <label>截止日期</label>
            <input
              v-model="dateEnd"
              type="date"
              class="date-input"
            />
          </div>
        </div>
      </div>

      <!-- Reset Button -->
      <button class="btn-reset" @click="resetFilters">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"></path>
          <path d="M3 3v5h5"></path>
        </svg>
        重置筛选
      </button>
    </div>
  </div>
</template>

<style scoped>
.search-filter-panel {
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  overflow: hidden;
}

.search-bar {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  border-bottom: 1px solid #f0f0f0;
}

.search-icon {
  color: #8c8c8c;
  flex-shrink: 0;
}

.search-input {
  flex: 1;
  border: none;
  outline: none;
  font-size: 14px;
  color: #262626;
}

.search-input::placeholder {
  color: #bfbfbf;
}

.btn-clear-search {
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  color: #8c8c8c;
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.2s;
}

.btn-clear-search:hover {
  background: #f0f0f0;
  color: #262626;
}

.filter-toggle {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  padding: 12px 16px;
  background: white;
  border: none;
  border-bottom: 1px solid #f0f0f0;
  cursor: pointer;
  font-size: 14px;
  color: #262626;
  transition: background 0.2s;
}

.filter-toggle:hover {
  background: #fafafa;
}

.filter-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 18px;
  height: 18px;
  padding: 0 6px;
  background: #ff4d4f;
  color: white;
  font-size: 12px;
  font-weight: 600;
  border-radius: 9px;
}

.filter-toggle .chevron {
  margin-left: auto;
  transition: transform 0.2s;
}

.filter-toggle .chevron.rotated {
  transform: rotate(180deg);
}

.filter-panel {
  padding: 16px;
  background: #fafafa;
}

.filter-section {
  margin-bottom: 20px;
}

.filter-section:last-of-type {
  margin-bottom: 0;
}

.section-title {
  font-size: 13px;
  font-weight: 600;
  color: #595959;
  margin: 0 0 12px 0;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.quick-filters {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 8px;
}

.quick-filter-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  background: white;
  border: 1px solid #d9d9d9;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  color: #595959;
  transition: all 0.2s;
}

.quick-filter-btn:hover {
  border-color: #40a9ff;
  color: #40a9ff;
}

.quick-filter-btn.active {
  background: #e6f7ff;
  border-color: #1890ff;
  color: #1890ff;
}

.quick-filter-icon {
  font-size: 16px;
}

.checkbox-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  font-size: 14px;
  color: #262626;
  user-select: none;
}

.checkbox-label input[type="checkbox"] {
  width: 16px;
  height: 16px;
  cursor: pointer;
}

.priority-badge {
  display: inline-block;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 600;
}

.priority-badge.high {
  background: #fff1f0;
  color: #ff4d4f;
}

.priority-badge.medium {
  background: #fffbe6;
  color: #faad14;
}

.priority-badge.low {
  background: #f6ffed;
  color: #52c41a;
}

.tags-group {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.tag-btn {
  padding: 4px 10px;
  background: white;
  border: 1px solid #d9d9d9;
  border-radius: 12px;
  font-size: 12px;
  color: #595959;
  cursor: pointer;
  transition: all 0.2s;
}

.tag-btn:hover {
  border-color: #40a9ff;
  color: #40a9ff;
}

.tag-btn.active {
  background: #e6f7ff;
  border-color: #1890ff;
  color: #1890ff;
}

.date-inputs {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
}

.date-input-group {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.date-input-group label {
  font-size: 12px;
  color: #595959;
}

.date-input {
  padding: 6px 8px;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  font-size: 14px;
}

.date-input:focus {
  outline: none;
  border-color: #1890ff;
}

.btn-reset {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  width: 100%;
  padding: 10px;
  background: white;
  border: 1px solid #d9d9d9;
  border-radius: 6px;
  font-size: 14px;
  color: #595959;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-reset:hover {
  background: #f0f0f0;
  border-color: #bfbfbf;
}

/* Responsive */
@media (max-width: 768px) {
  .search-filter-panel {
    border-radius: 6px;
  }

  .search-bar {
    padding: 10px 12px;
    gap: 8px;
  }

  .search-input {
    font-size: 13px;
  }

  .filter-toggle {
    padding: 10px 12px;
    font-size: 13px;
  }

  .filter-panel {
    padding: 12px;
  }

  .filter-section {
    margin-bottom: 16px;
  }

  .section-title {
    font-size: 12px;
    margin-bottom: 10px;
  }

  .quick-filters {
    grid-template-columns: 1fr;
    gap: 6px;
  }

  .quick-filter-btn {
    padding: 8px 10px;
    font-size: 13px;
  }

  .checkbox-label {
    font-size: 13px;
  }

  .tags-group {
    gap: 4px;
  }

  .tag-btn {
    padding: 3px 8px;
    font-size: 11px;
  }

  .date-inputs {
    grid-template-columns: 1fr;
    gap: 8px;
  }

  .date-input {
    padding: 5px 8px;
    font-size: 13px;
  }

  .btn-reset {
    font-size: 13px;
    padding: 8px;
  }
}

@media (max-width: 480px) {
  .search-bar {
    padding: 8px 10px;
  }

  .search-input {
    font-size: 12px;
  }

  .filter-toggle {
    padding: 8px 10px;
    font-size: 12px;
  }

  .filter-panel {
    padding: 10px;
  }

  .quick-filter-btn {
    padding: 6px 8px;
    font-size: 12px;
  }

  .date-input {
    font-size: 12px;
    padding: 4px 6px;
  }
}
</style>
