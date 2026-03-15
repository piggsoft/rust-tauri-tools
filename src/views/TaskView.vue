<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useTaskStore } from '../stores/task'
import TaskList from '../components/TaskList.vue'
import CalendarView from '../components/CalendarView.vue'
import QuadrantView from '../components/QuadrantView.vue'
import SearchFilterPanel from '../components/SearchFilterPanel.vue'
import DataManager from '../components/DataManager.vue'
import type { TaskFilter } from '../types'

const taskStore = useTaskStore()
const showDataManager = ref(false)
const currentView = computed(() => taskStore.currentView)

function switchView(view: 'list' | 'calendar' | 'quadrant') {
  taskStore.switchView(view)
}

function handleFilterChange(filter: TaskFilter) {
  taskStore.updateFilters(filter)
  taskStore.fetchTasks()
}

onMounted(() => {
  taskStore.fetchTasks()
})
</script>

<template>
  <div class="task-view">
    <!-- Header -->
    <header class="header">
      <div class="header-left">
        <h1>个人工具集</h1>
        <div class="view-switcher">
          <button
            v-for="view in ['list', 'calendar', 'quadrant']"
            :key="view"
            :class="{ active: currentView === view }"
            @click="switchView(view as any)"
          >
            {{ view === 'list' ? '列表' : view === 'calendar' ? '日历' : '四象限' }}
          </button>
        </div>
      </div>
      <div class="header-right">
        <button class="btn-secondary" @click="showDataManager = true" title="数据管理">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"></path>
            <polyline points="17 21 17 13 7 13 7 21"></polyline>
            <polyline points="7 3 7 8 15 8"></polyline>
          </svg>
        </button>
        <button class="btn-primary" @click="taskStore.createTask({
          title: '新任务',
          priority: 'medium',
          urgency: 'medium'
        })">
          + 新建任务
        </button>
      </div>
    </header>

    <!-- Main Content -->
    <main class="main-content">
      <!-- Search and Filter Panel -->
      <div class="filter-section">
        <SearchFilterPanel @filter-change="handleFilterChange" />
      </div>

      <!-- View Content -->
      <TaskList v-if="currentView === 'list'" />
      <CalendarView v-else-if="currentView === 'calendar'" />
      <QuadrantView v-else-if="currentView === 'quadrant'" />
    </main>

    <!-- Data Manager Modal -->
    <DataManager v-if="showDataManager" @close="showDataManager = false" />
  </div>
</template>

<style scoped>
.task-view {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  background: #f5f5f5;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 24px;
  background: white;
  border-bottom: 1px solid #e0e0e0;
  box-shadow: 0 2px 4px rgba(0,0,0,0.05);
  flex-wrap: wrap;
  gap: 12px;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 16px;
  flex: 1;
  min-width: 0;
}

.header-left h1 {
  font-size: 18px;
  font-weight: 600;
  color: #1a1a1a;
  margin: 0;
  white-space: nowrap;
}

.view-switcher {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}

.view-switcher button {
  padding: 6px 12px;
  border: 1px solid #e0e0e0;
  background: white;
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
  color: #666;
  transition: all 0.2s;
  white-space: nowrap;
}

.view-switcher button:hover {
  border-color: #1890ff;
  color: #1890ff;
}

.view-switcher button.active {
  background: #1890ff;
  border-color: #1890ff;
  color: white;
}

.header-right {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.btn-secondary {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 6px 12px;
  background: white;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  color: #595959;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-secondary:hover {
  border-color: #40a9ff;
  color: #40a9ff;
}

.btn-primary {
  padding: 6px 16px;
  background: #1890ff;
  border: none;
  border-radius: 4px;
  color: white;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  white-space: nowrap;
}

.btn-primary:hover {
  background: #40a9ff;
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(24, 144, 255, 0.3);
}

.main-content {
  flex: 1;
  overflow: hidden;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
  min-width: 0;
}

.filter-section {
  flex-shrink: 0;
}

/* Responsive */
@media (max-width: 768px) {
  .header {
    padding: 12px 16px;
  }

  .header-left {
    width: 100%;
    gap: 12px;
  }

  .header-left h1 {
    font-size: 16px;
  }

  .view-switcher {
    width: 100%;
  }

  .view-switcher button {
    flex: 1;
    justify-content: center;
  }

  .header-right {
    width: 100%;
    justify-content: space-between;
  }

  .btn-secondary,
  .btn-primary {
    flex: 1;
  }

  .main-content {
    padding: 12px;
  }
}

@media (max-width: 480px) {
  .header {
    padding: 8px 12px;
  }

  .header-left h1 {
    font-size: 14px;
  }

  .view-switcher button {
    font-size: 12px;
    padding: 4px 8px;
  }

  .btn-secondary,
  .btn-primary {
    font-size: 12px;
    padding: 4px 8px;
  }

  .main-content {
    padding: 8px;
  }
}
</style>
