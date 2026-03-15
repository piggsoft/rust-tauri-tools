<script setup lang="ts">
import { computed, ref, watch, onMounted } from 'vue'
import { useTaskStore } from '../stores/task'
import TaskItem from './TaskItem.vue'
import type { Task } from '../types'

const taskStore = useTaskStore()

const quadrants = ref({
  first: [] as Task[],
  second: [] as Task[],
  third: [] as Task[],
  fourth: [] as Task[],
})

const quadrantDefinitions = {
  first: {
    title: '第一象限',
    subtitle: '重要且紧急',
    description: '立即做',
    color: '#ff4d4f',
    priority: 'high',
    urgency: 'high',
  },
  second: {
    title: '第二象限',
    subtitle: '重要但不紧急',
    description: '计划做',
    color: '#faad14',
    priority: 'high',
    urgency: ['medium', 'low'],
  },
  third: {
    title: '第三象限',
    subtitle: '不重要但紧急',
    description: '快速做/授权',
    color: '#1890ff',
    priority: ['medium', 'low'],
    urgency: 'high',
  },
  fourth: {
    title: '第四象限',
    subtitle: '不重要且不紧急',
    description: '删除/延后',
    color: '#52c41a',
    priority: ['medium', 'low'],
    urgency: ['medium', 'low'],
  },
}

function isInQuadrant(task: Task, quadrant: string): boolean {
  // Ensure task is a valid pending task
  if (task.status !== 'pending') return false

  switch (quadrant) {
    case 'first':
      return task.priority === 'high' && task.urgency === 'high'
    case 'second':
      return task.priority === 'high' && (task.urgency === 'medium' || task.urgency === 'low')
    case 'third':
      return (task.priority === 'medium' || task.priority === 'low') && task.urgency === 'high'
    case 'fourth':
      return (task.priority === 'medium' || task.priority === 'low') &&
             (task.urgency === 'medium' || task.urgency === 'low')
    default:
      return false
  }
}

function updateQuadrants() {
  const pendingTasks = taskStore.tasks.filter(t => t.status === 'pending')

  quadrants.value = {
    first: pendingTasks.filter((t: Task) => isInQuadrant(t, 'first')),
    second: pendingTasks.filter((t: Task) => isInQuadrant(t, 'second')),
    third: pendingTasks.filter((t: Task) => isInQuadrant(t, 'third')),
    fourth: pendingTasks.filter((t: Task) => isInQuadrant(t, 'fourth')),
  }
}

const totalTasks = computed(() => {
  return quadrants.value.first.length +
         quadrants.value.second.length +
         quadrants.value.third.length +
         quadrants.value.fourth.length
})

const mostUrgentQuadrant = computed(() => {
  const counts = {
    first: quadrants.value.first.length,
    second: quadrants.value.second.length,
    third: quadrants.value.third.length,
    fourth: quadrants.value.fourth.length,
  }

  return Object.entries(counts).sort((a, b) => b[1] - a[1])[0]
})

function moveTaskToQuadrant(task: any, targetQuadrant: 'first' | 'second' | 'third' | 'fourth') {
  const quadrant = quadrantDefinitions[targetQuadrant]

  let priority = task.priority
  let urgency = task.urgency

  if (targetQuadrant === 'first') {
    priority = 'high'
    urgency = 'high'
  } else if (targetQuadrant === 'second') {
    priority = 'high'
    urgency = 'medium'
  } else if (targetQuadrant === 'third') {
    priority = 'medium'
    urgency = 'high'
  } else if (targetQuadrant === 'fourth') {
    priority = 'medium'
    urgency = 'medium'
  }

  taskStore.updateTask(task.id, {
    title: task.title,
    description: task.description,
    priority: priority as 'high' | 'medium' | 'low',
    urgency: urgency as 'high' | 'medium' | 'low',
    tags: task.tags,
    start_date: task.start_date,
    due_date: task.due_date,
    reminder_time: task.reminder_time,
    estimated_duration: task.estimated_duration,
    repeat_pattern: task.repeat_pattern,
    repeat_until: task.repeat_until,
  }  )
}

// Initialize quadrants when component mounts
onMounted(() => {
  updateQuadrants()
})

// Watch for task changes
watch(() => taskStore.tasks, () => {
  updateQuadrants()
}, { deep: true })
</script>

<template>
  <div class="quadrant-view">
    <!-- Header -->
    <div class="quadrant-header">
      <div class="header-left">
        <h2>艾森豪威尔矩阵</h2>
        <p class="subtitle">基于重要性和紧急度对任务进行分类</p>
      </div>
      <div class="header-right">
        <div class="stats">
          <span class="stat-item">
            <span class="stat-label">总任务</span>
            <span class="stat-value">{{ totalTasks }}</span>
          </span>
          <span v-if="mostUrgentQuadrant" class="stat-item">
            <span class="stat-label">最多</span>
            <span
              class="stat-value"
              :style="{ color: quadrantDefinitions[mostUrgentQuadrant[0] as keyof typeof quadrantDefinitions].color }"
            >
              {{ mostUrgentQuadrant[0] === 'first' ? '第一' : mostUrgentQuadrant[0] === 'second' ? '第二' : mostUrgentQuadrant[0] === 'third' ? '第三' : '第四' }}象限
            </span>
          </span>
        </div>
      </div>
    </div>

    <!-- Quadrant Grid -->
    <div class="quadrant-grid">
      <!-- First Quadrant -->
      <div class="quadrant first-quadrant">
        <div class="quadrant-header">
          <div class="quadrant-info">
            <h3>{{ quadrantDefinitions.first.title }}</h3>
            <p class="quadrant-subtitle">{{ quadrantDefinitions.first.subtitle }}</p>
          </div>
          <div class="quadrant-badge" :style="{ background: quadrantDefinitions.first.color }">
            {{ quadrantDefinitions.first.description }}
          </div>
        </div>
        <div class="quadrant-content">
          <div v-if="quadrants.first.length === 0" class="quadrant-empty">
            <span class="empty-icon">🔥</span>
            <p>没有紧急重要的任务</p>
          </div>
          <div v-else class="task-list">
            <TaskItem
              v-for="task in quadrants.first"
              :key="task.id"
              :task="task"
              @edit="() => taskStore.selectTask(task)"
            />
          </div>
        </div>
      </div>

      <!-- Second Quadrant -->
      <div class="quadrant second-quadrant">
        <div class="quadrant-header">
          <div class="quadrant-info">
            <h3>{{ quadrantDefinitions.second.title }}</h3>
            <p class="quadrant-subtitle">{{ quadrantDefinitions.second.subtitle }}</p>
          </div>
          <div class="quadrant-badge" :style="{ background: quadrantDefinitions.second.color }">
            {{ quadrantDefinitions.second.description }}
          </div>
        </div>
        <div class="quadrant-content">
          <div v-if="quadrants.second.length === 0" class="quadrant-empty">
            <span class="empty-icon">📋</span>
            <p>没有重要不紧急的任务</p>
          </div>
          <div v-else class="task-list">
            <TaskItem
              v-for="task in quadrants.second"
              :key="task.id"
              :task="task"
              @edit="() => taskStore.selectTask(task)"
            />
          </div>
        </div>
      </div>

      <!-- Third Quadrant -->
      <div class="quadrant third-quadrant">
        <div class="quadrant-header">
          <div class="quadrant-info">
            <h3>{{ quadrantDefinitions.third.title }}</h3>
            <p class="quadrant-subtitle">{{ quadrantDefinitions.third.subtitle }}</p>
          </div>
          <div class="quadrant-badge" :style="{ background: quadrantDefinitions.third.color }">
            {{ quadrantDefinitions.third.description }}
          </div>
        </div>
        <div class="quadrant-content">
          <div v-if="quadrants.third.length === 0" class="quadrant-empty">
            <span class="empty-icon">⏰</span>
            <p>没有紧急不重要的任务</p>
          </div>
          <div v-else class="task-list">
            <TaskItem
              v-for="task in quadrants.third"
              :key="task.id"
              :task="task"
              @edit="() => taskStore.selectTask(task)"
            />
          </div>
        </div>
      </div>

      <!-- Fourth Quadrant -->
      <div class="quadrant fourth-quadrant">
        <div class="quadrant-header">
          <div class="quadrant-info">
            <h3>{{ quadrantDefinitions.fourth.title }}</h3>
            <p class="quadrant-subtitle">{{ quadrantDefinitions.fourth.subtitle }}</p>
          </div>
          <div class="quadrant-badge" :style="{ background: quadrantDefinitions.fourth.color }">
            {{ quadrantDefinitions.fourth.description }}
          </div>
        </div>
        <div class="quadrant-content">
          <div v-if="quadrants.fourth.length === 0" class="quadrant-empty">
            <span class="empty-icon">🗑️</span>
            <p>没有不紧急不重要的任务</p>
          </div>
          <div v-else class="task-list">
            <TaskItem
              v-for="task in quadrants.fourth"
              :key="task.id"
              :task="task"
              @edit="() => taskStore.selectTask(task)"
            />
          </div>
        </div>
      </div>
    </div>

    <!-- Legend -->
    <div class="quadrant-legend">
      <h4>象限说明</h4>
      <div class="legend-items">
        <div class="legend-item">
          <span class="legend-color" :style="{ background: quadrantDefinitions.first.color }"></span>
          <span class="legend-text">
            <strong>第一象限：</strong>必须立即处理，否则会带来严重后果
          </span>
        </div>
        <div class="legend-item">
          <span class="legend-color" :style="{ background: quadrantDefinitions.second.color }"></span>
          <span class="legend-text">
            <strong>第二象限：</strong>重要但不紧急，应制定计划逐步完成
          </span>
        </div>
        <div class="legend-item">
          <span class="legend-color" :style="{ background: quadrantDefinitions.third.color }"></span>
          <span class="legend-text">
            <strong>第三象限：</strong>紧急但重要性低，可以快速处理或委托他人
          </span>
        </div>
        <div class="legend-item">
          <span class="legend-color" :style="{ background: quadrantDefinitions.fourth.color }"></span>
          <span class="legend-text">
            <strong>第四象限：</strong>不重要也不紧急，可以删除或延后处理
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.quadrant-view {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: #f5f5f5;
  overflow: hidden;
}

.quadrant-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  background: white;
  border-bottom: 1px solid #e0e0e0;
}

.header-left h2 {
  font-size: 24px;
  font-weight: 600;
  color: #262626;
  margin: 0 0 4px 0;
}

.subtitle {
  font-size: 14px;
  color: #8c8c8c;
  margin: 0;
}

.header-right {
  display: flex;
  gap: 24px;
}

.stats {
  display: flex;
  gap: 24px;
}

.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.stat-label {
  font-size: 12px;
  color: #8c8c8c;
}

.stat-value {
  font-size: 20px;
  font-weight: 600;
  color: #262626;
}

.quadrant-grid {
  flex: 1;
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  grid-template-rows: repeat(2, 1fr);
  gap: 16px;
  padding: 16px;
  overflow-y: auto;
  overflow-x: hidden;
  min-height: 0; /* Important for nested flex/grid overflow */
}

.quadrant {
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-height: 300px; /* Increased minimum height for better visibility */
}

.quadrant-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 14px 18px;
  border-bottom: 1px solid #f0f0f0;
  flex-shrink: 0; /* Don't shrink header */
}

.quadrant-info h3 {
  font-size: 16px;
  font-weight: 600;
  color: #262626;
  margin: 0 0 2px 0;
}

.quadrant-subtitle {
  font-size: 12px;
  color: #8c8c8c;
  margin: 0;
}

.quadrant-badge {
  padding: 4px 12px;
  border-radius: 16px;
  color: white;
  font-size: 12px;
  font-weight: 500;
  white-space: nowrap;
}

.quadrant-content {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  padding: 10px;
  min-height: 0; /* Important for nested overflow */
}

.quadrant-content::-webkit-scrollbar {
  width: 6px;
}

.quadrant-content::-webkit-scrollbar-thumb {
  background: #d9d9d9;
  border-radius: 3px;
}

.quadrant-content::-webkit-scrollbar-thumb:hover {
  background: #bfbfbf;
}

.quadrant-grid::-webkit-scrollbar {
  width: 8px;
}

.quadrant-grid::-webkit-scrollbar-thumb {
  background: #d9d9d9;
  border-radius: 4px;
}

.quadrant-grid::-webkit-scrollbar-thumb:hover {
  background: #bfbfbf;
}

.quadrant-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #8c8c8c;
  padding: 40px 20px;
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 12px;
}

.quadrant-empty p {
  font-size: 14px;
  margin: 0;
  text-align: center;
}

.task-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.quadrant-legend {
  background: white;
  border-top: 1px solid #e0e0e0;
  padding: 16px 24px;
}

.quadrant-legend h4 {
  font-size: 14px;
  font-weight: 600;
  color: #262626;
  margin: 0 0 12px 0;
}

.legend-items {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

.legend-item {
  display: flex;
  align-items: flex-start;
  gap: 10px;
}

.legend-color {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  margin-top: 4px;
  flex-shrink: 0;
}

.legend-text {
  font-size: 13px;
  color: #595959;
  line-height: 1.5;
}

.legend-text strong {
  color: #262626;
}

/* Responsive adjustments */
@media (max-width: 1200px) {
  .quadrant-grid {
    grid-template-columns: 1fr;
    grid-template-rows: repeat(4, 1fr);
  }

  .legend-items {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 768px) {
  .quadrant-header {
    padding: 16px;
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }

  .header-left h2 {
    font-size: 20px;
  }

  .subtitle {
    font-size: 13px;
  }

  .header-right {
    width: 100%;
  }

  .stats {
    width: 100%;
    justify-content: space-around;
  }

  .quadrant-grid {
    padding: 12px;
    gap: 12px;
  }

  .quadrant {
    min-height: 250px;
  }

  .quadrant-header {
    padding: 12px 16px;
  }

  .quadrant-info h3 {
    font-size: 16px;
  }

  .quadrant-subtitle {
    font-size: 12px;
  }

  .quadrant-badge {
    padding: 4px 12px;
    font-size: 12px;
  }

  .quadrant-content {
    padding: 8px;
  }

  .quadrant-empty {
    padding: 24px 16px;
  }

  .empty-icon {
    font-size: 36px;
  }

  .quadrant-empty p {
    font-size: 13px;
  }

  .task-list {
    gap: 6px;
  }

  .quadrant-legend {
    padding: 12px 16px;
  }

  .quadrant-legend h4 {
    font-size: 13px;
    margin-bottom: 10px;
  }

  .legend-items {
    gap: 8px;
  }

  .legend-item {
    gap: 8px;
  }

  .legend-color {
    width: 10px;
    height: 10px;
  }

  .legend-text {
    font-size: 12px;
  }
}

@media (max-width: 480px) {
  .quadrant-view {
    overflow-y: auto;
  }

  .quadrant-header {
    padding: 12px;
  }

  .header-left h2 {
    font-size: 18px;
  }

  .subtitle {
    font-size: 12px;
  }

  .quadrant-grid {
    padding: 8px;
    gap: 8px;
  }

  .quadrant {
    min-height: 200px;
  }

  .quadrant-header {
    padding: 10px 12px;
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }

  .quadrant-info h3 {
    font-size: 15px;
    margin: 0 0 2px 0;
  }

  .quadrant-subtitle {
    font-size: 11px;
  }

  .quadrant-badge {
    padding: 3px 10px;
    font-size: 11px;
    align-self: flex-start;
  }

  .quadrant-content {
    padding: 6px;
  }

  .quadrant-empty {
    padding: 20px 12px;
  }

  .empty-icon {
    font-size: 32px;
    margin-bottom: 8px;
  }

  .quadrant-empty p {
    font-size: 12px;
  }

  .task-list {
    gap: 4px;
  }

  .quadrant-legend {
    padding: 10px 12px;
  }

  .quadrant-legend h4 {
    font-size: 12px;
    margin-bottom: 8px;
  }

  .legend-items {
    gap: 6px;
  }

  .legend-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 4px;
  }

  .legend-color {
    margin-top: 0;
  }

  .legend-text {
    font-size: 11px;
    line-height: 1.4;
  }
}
</style>
