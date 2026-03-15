<script setup lang="ts">
import { ref, computed } from 'vue'
import { useTaskStore } from '../stores/task'
import TaskList from './TaskList.vue'
import type { Task } from '../types'
import { formatDateToYYYYMMDD, getToday } from '../utils/dateUtils'

const taskStore = useTaskStore()

const currentMonth = ref(new Date())
const selectedDate = ref<Date | null>(null)
const showListView = ref(false)

const daysInMonth = computed(() => {
  const year = currentMonth.value.getFullYear()
  const month = currentMonth.value.getMonth()
  const firstDay = new Date(year, month, 1)
  const lastDay = new Date(year, month + 1, 0)
  const startDayOfWeek = firstDay.getDay()
  const totalDays = lastDay.getDate()

  const days = []
  const prevMonthDays = new Date(year, month, 0).getDate()

  // Previous month days
  for (let i = startDayOfWeek - 1; i >= 0; i--) {
    days.push({
      date: new Date(year, month - 1, prevMonthDays - i),
      currentMonth: false,
    })
  }

  // Current month days
  for (let i = 1; i <= totalDays; i++) {
    days.push({
      date: new Date(year, month, i),
      currentMonth: true,
    })
  }

  // Next month days
  const remaining = 42 - days.length // 6 weeks * 7 days
  for (let i = 1; i <= remaining; i++) {
    days.push({
      date: new Date(year, month + 1, i),
      currentMonth: false,
    })
  }

  return days
})

const weekDays = ['日', '一', '二', '三', '四', '五', '六']

const tasksByDate = computed(() => {
  const map = new Map<string, Task[]>()
  taskStore.tasks.forEach(task => {
    if (task.due_date) {
      const dateKey = task.due_date.split('T')[0]
      if (!map.has(dateKey)) {
        map.set(dateKey, [])
      }
      map.get(dateKey)!.push(task)
    }
  })
  return map
})

const today = computed(() => getToday())

const selectedDateTasks = computed(() => {
  if (!selectedDate.value) return []
  const dateKey = formatDateToYYYYMMDD(selectedDate.value)
  return tasksByDate.value.get(dateKey) || []
})

const monthYearText = computed(() => {
  const year = currentMonth.value.getFullYear()
  const month = currentMonth.value.getMonth() + 1
  return `${year}年${month}月`
})

function isToday(date: Date): boolean {
  return formatDateToYYYYMMDD(date) === formatDateToYYYYMMDD(today.value)
}

function isSelected(date: Date): boolean {
  return selectedDate.value && formatDateToYYYYMMDD(date) === formatDateToYYYYMMDD(selectedDate.value)
}

function getTasksForDate(date: Date): Task[] {
  const dateKey = formatDateToYYYYMMDD(date)
  return tasksByDate.value.get(dateKey) || []
}

function hasIncompleteTasks(date: Date): boolean {
  const tasks = getTasksForDate(date)
  return tasks.some(t => t.status === 'pending')
}

function hasOverdueTasks(date: Date): boolean {
  const tasks = getTasksForDate(date)
  const now = new Date()
  return tasks.some(t =>
    t.status === 'pending' &&
    t.due_date &&
    new Date(t.due_date) < now
  )
}

function selectDate(date: Date) {
  if (!isSelected(date)) {
    selectedDate.value = date
    showListView.value = true
  } else {
    showListView.value = !showListView.value
  }
}

function closeListView() {
  showListView.value = false
  selectedDate.value = null
}

function prevMonth() {
  currentMonth.value = new Date(
    currentMonth.value.getFullYear(),
    currentMonth.value.getMonth() - 1,
    1
  )
}

function nextMonth() {
  currentMonth.value = new Date(
    currentMonth.value.getFullYear(),
    currentMonth.value.getMonth() + 1,
    1
  )
}

function goToToday() {
  currentMonth.value = new Date()
  selectedDate.value = new Date()
  showListView.value = true
}

function prevWeek() {
  currentMonth.value = new Date(
    currentMonth.value.getFullYear(),
    currentMonth.value.getMonth() - 1,
    currentMonth.value.getDate() - 7
  )
}

function nextWeek() {
  currentMonth.value = new Date(
    currentMonth.value.getFullYear(),
    currentMonth.value.getMonth() + 1,
    currentMonth.value.getDate() + 7
  )
}
</script>

<template>
  <div class="calendar-view">
    <!-- Calendar Section -->
    <div class="calendar-section" :class="{ 'show-list': showListView }">
      <!-- Calendar Header -->
      <div class="calendar-header">
        <div class="header-left">
          <h2 class="month-title">{{ monthYearText }}</h2>
          <div class="calendar-nav">
            <button class="btn-nav" @click="prevMonth">◀</button>
            <button class="btn-today" @click="goToToday">今天</button>
            <button class="btn-nav" @click="nextMonth">▶</button>
          </div>
        </div>
        <div class="header-right">
          <button class="btn-toggle" @click="showListView = !showListView">
            {{ showListView ? '显示日历' : '显示列表' }}
          </button>
        </div>
      </div>

      <!-- Calendar Grid -->
      <div class="calendar-grid">
        <!-- Weekday Headers -->
        <div class="weekday-header">
          <div v-for="day in weekDays" :key="day" class="weekday">{{ day }}</div>
        </div>

        <!-- Days Grid -->
        <div class="days-grid">
          <div
            v-for="(day, index) in daysInMonth"
            :key="index"
            class="day-cell"
            :class="{
              'other-month': !day.currentMonth,
              'today': isToday(day.date),
              'selected': isSelected(day.date),
              'has-tasks': getTasksForDate(day.date).length > 0,
              'has-incomplete': hasIncompleteTasks(day.date),
              'has-overdue': hasOverdueTasks(day.date),
            }"
            @click="selectDate(day.date)"
          >
            <div class="day-number">{{ day.date.getDate() }}</div>
            <div v-if="getTasksForDate(day.date).length > 0" class="task-indicators">
              <span
                v-for="(task, i) in getTasksForDate(day.date).slice(0, 3)"
                :key="i"
                class="task-dot"
                :class="{
                  'completed': task.status === 'completed',
                  'overdue': hasOverdueTasks(day.date) && task.status === 'pending',
                }"
              ></span>
              <span v-if="getTasksForDate(day.date).length > 3" class="more-tasks">
                +{{ getTasksForDate(day.date).length - 3 }}
              </span>
            </div>
          </div>
        </div>
      </div>

      <!-- Legend -->
      <div class="legend">
        <div class="legend-item">
          <span class="dot overdue"></span>
          <span>过期</span>
        </div>
        <div class="legend-item">
          <span class="dot incomplete"></span>
          <span>未完成</span>
        </div>
        <div class="legend-item">
          <span class="dot completed"></span>
          <span>已完成</span>
        </div>
      </div>
    </div>

    <!-- List View Section -->
    <Transition name="slide">
      <div v-if="showListView && selectedDate" class="list-section">
        <div class="list-header">
          <button class="btn-back" @click="closeListView">
            ◀ 返回日历
          </button>
          <h3 class="selected-date-title">
            {{ selectedDate.toLocaleDateString('zh-CN', {
              year: 'numeric',
              month: 'long',
              day: 'numeric',
              weekday: 'long'
            }) }}
          </h3>
          <div class="task-count">
            {{ selectedDateTasks.length }} 个任务
          </div>
        </div>

        <div class="list-content">
          <div v-if="selectedDateTasks.length === 0" class="empty-state">
            <div class="empty-icon">📋</div>
            <h3>暂无任务</h3>
            <p>该日期下没有任务</p>
          </div>
          <div v-else class="task-items">
            <div
              v-for="task in selectedDateTasks"
              :key="task.id"
              class="task-item-mini"
              :class="{
                'completed': task.status === 'completed',
                'overdue': hasOverdueTasks(selectedDate!) && task.status === 'pending',
              }"
            >
              <div class="task-info">
                <h4 class="task-title">{{ task.title }}</h4>
                <p v-if="task.description" class="task-desc">{{ task.description }}</p>
              </div>
              <div class="task-meta">
                <span class="priority" :class="task.priority">{{ task.priority }}</span>
                <span class="urgency" :class="task.urgency">{{ task.urgency }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.calendar-view {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: white;
  border-radius: 8px;
  overflow: hidden;
}

.calendar-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  transition: all 0.3s ease;
}

.calendar-section.show-list {
  flex: 0;
  min-height: 400px;
  max-height: 400px;
}

.calendar-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 24px;
  border-bottom: 1px solid #f0f0f0;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 20px;
}

.month-title {
  font-size: 20px;
  font-weight: 600;
  color: #262626;
  margin: 0;
}

.calendar-nav {
  display: flex;
  gap: 8px;
}

.btn-nav {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 1px solid #d9d9d9;
  background: white;
  border-radius: 4px;
  cursor: pointer;
  font-size: 16px;
  color: #595959;
  transition: all 0.2s;
}

.btn-nav:hover {
  border-color: #1890ff;
  color: #1890ff;
}

.btn-today {
  padding: 0 16px;
  height: 32px;
  border: 1px solid #d9d9d9;
  background: white;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  color: #595959;
  transition: all 0.2s;
}

.btn-today:hover {
  border-color: #1890ff;
  color: #1890ff;
}

.header-right {
  display: flex;
  gap: 12px;
}

.btn-toggle {
  padding: 8px 16px;
  background: #f0f0f0;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  color: #595959;
  transition: all 0.2s;
}

.btn-toggle:hover {
  background: #e0e0e0;
}

.calendar-grid {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 16px;
  overflow-y: auto;
}

.weekday-header {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 2px;
  margin-bottom: 8px;
}

.weekday {
  text-align: center;
  padding: 8px;
  font-size: 14px;
  font-weight: 600;
  color: #595959;
}

.days-grid {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 2px;
  flex: 1;
}

.day-cell {
  min-height: 80px;
  padding: 8px;
  border: 1px solid #f0f0f0;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
  background: white;
  position: relative;
}

.day-cell:hover {
  border-color: #1890ff;
  background: #f0f9ff;
}

.day-cell.other-month {
  background: #fafafa;
  color: #bfbfbf;
}

.day-cell.today {
  border-color: #1890ff;
  background: #e6f7ff;
}

.day-cell.selected {
  border-color: #1890ff;
  background: #bae7ff;
}

.day-cell.has-tasks {
  background: #f6ffed;
}

.day-cell.has-incomplete:not(.has-overdue) {
  background: #fff7e6;
}

.day-cell.has-overdue {
  background: #fff1f0;
}

.day-number {
  font-size: 14px;
  font-weight: 500;
  margin-bottom: 4px;
}

.task-indicators {
  display: flex;
  flex-wrap: wrap;
  gap: 3px;
}

.task-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: #52c41a;
}

.task-dot.overdue {
  background: #ff4d4f;
}

.task-dot.completed {
  background: #bfbfbf;
}

.more-tasks {
  font-size: 10px;
  color: #8c8c8c;
}

.legend {
  display: flex;
  justify-content: center;
  gap: 24px;
  padding: 12px;
  background: #fafafa;
  border-top: 1px solid #f0f0f0;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: #595959;
}

.dot {
  width: 12px;
  height: 12px;
  border-radius: 50%;
}

.dot.overdue {
  background: #ff4d4f;
}

.dot.incomplete {
  background: #faad14;
}

.dot.completed {
  background: #52c41a;
}

.list-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  border-top: 1px solid #f0f0f0;
  background: white;
  overflow: hidden;
}

.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 24px;
  border-bottom: 1px solid #f0f0f0;
  background: #fafafa;
}

.btn-back {
  padding: 6px 12px;
  background: white;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  color: #595959;
  transition: all 0.2s;
}

.btn-back:hover {
  border-color: #1890ff;
  color: #1890ff;
}

.selected-date-title {
  font-size: 16px;
  font-weight: 600;
  color: #262626;
  margin: 0;
}

.task-count {
  font-size: 14px;
  color: #8c8c8c;
}

.list-content {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  color: #8c8c8c;
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 16px;
}

.empty-state h3 {
  font-size: 18px;
  margin: 0 0 8px 0;
}

.empty-state p {
  font-size: 14px;
  margin: 0;
}

.task-items {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.task-item-mini {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: white;
  border: 1px solid #f0f0f0;
  border-radius: 6px;
  transition: all 0.2s;
}

.task-item-mini:hover {
  border-color: #e0e0e0;
  box-shadow: 0 2px 4px rgba(0,0,0,0.05);
}

.task-item-mini.completed {
  opacity: 0.6;
}

.task-item-mini.overdue {
  border-color: #ffa39e;
  background: #fff1f0;
}

.task-info {
  flex: 1;
  min-width: 0;
}

.task-title {
  font-size: 14px;
  font-weight: 500;
  color: #262626;
  margin: 0 0 4px 0;
}

.task-item-mini.completed .task-title {
  text-decoration: line-through;
  color: #8c8c8c;
}

.task-desc {
  font-size: 13px;
  color: #8c8c8c;
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.task-meta {
  display: flex;
  gap: 8px;
}

.priority,
.urgency {
  font-size: 11px;
  padding: 3px 8px;
  border-radius: 3px;
  font-weight: 500;
}

.priority.high { background: #fff1f0; color: #ff4d4f; }
.priority.medium { background: #fffbe6; color: #faad14; }
.priority.low { background: #f6ffed; color: #52c41a; }

.urgency.high { background: #fff1f0; color: #ff4d4f; }
.urgency.medium { background: #fffbe6; color: #faad14; }
.urgency.low { background: #f6ffed; color: #52c41a; }

.slide-enter-active,
.slide-leave-active {
  transition: all 0.3s ease;
}

.slide-enter-from {
  transform: translateX(100%);
  opacity: 0;
}

.slide-leave-to {
  transform: translateX(100%);
  opacity: 0;
}

/* Responsive */
@media (max-width: 768px) {
  .calendar-section {
    padding: 12px;
  }

  .calendar-header {
    padding: 10px 12px;
    gap: 8px;
  }

  .calendar-nav {
    padding: 12px;
  }

  .month-year {
    font-size: 16px;
  }

  .week-days {
    padding: 8px 12px;
    gap: 2px;
  }

  .week-day {
    font-size: 11px;
    width: calc(100% / 7);
  }

  .calendar-grid {
    padding: 8px 12px;
    gap: 4px;
  }

  .day-cell {
    min-height: 50px;
    padding: 4px;
  }

  .day-number {
    font-size: 12px;
    width: 20px;
    height: 20px;
  }

  .task-dot {
    width: 5px;
    height: 5px;
  }

  .more-tasks {
    font-size: 9px;
  }

  .legend {
    gap: 16px;
    padding: 10px;
  }

  .legend-item {
    font-size: 12px;
    gap: 6px;
  }

  .dot {
    width: 10px;
    height: 10px;
  }

  .list-section {
    width: 100%;
  }

  .list-header {
    padding: 12px 16px;
  }

  .btn-back {
    padding: 5px 10px;
    font-size: 13px;
  }

  .selected-date-title {
    font-size: 14px;
  }

  .task-count {
    font-size: 12px;
  }

  .list-content {
    padding: 12px;
  }

  .task-item-mini {
    padding: 10px 12px;
  }

  .task-title {
    font-size: 13px;
  }

  .task-desc {
    font-size: 12px;
  }

  .empty-state {
    padding: 40px 16px;
  }

  .empty-icon {
    font-size: 40px;
  }

  .empty-state h3 {
    font-size: 16px;
  }

  .empty-state p {
    font-size: 13px;
  }
}

@media (max-width: 480px) {
  .calendar-section {
    padding: 8px;
  }

  .calendar-header {
    padding: 8px 10px;
    flex-wrap: wrap;
  }

  .month-year {
    font-size: 14px;
  }

  .week-days {
    padding: 6px 8px;
  }

  .week-day {
    font-size: 10px;
  }

  .calendar-grid {
    padding: 6px 8px;
    gap: 2px;
  }

  .day-cell {
    min-height: 40px;
    padding: 2px;
  }

  .day-number {
    font-size: 11px;
    width: 18px;
    height: 18px;
  }

  .task-dots {
    gap: 2px;
  }

  .task-dot {
    width: 4px;
    height: 4px;
  }

  .legend {
    flex-wrap: wrap;
    gap: 12px;
  }

  .list-header {
    padding: 10px 12px;
  }

  .selected-date-title {
    font-size: 13px;
  }

  .list-content {
    padding: 10px;
  }

  .task-item-mini {
    padding: 8px 10px;
  }

  .task-info {
    min-width: 0;
    flex: 1;
  }

  .task-title {
    font-size: 12px;
  }

  .task-desc {
    font-size: 11px;
  }

  .priority,
  .urgency {
    font-size: 10px;
    padding: 2px 6px;
  }
}
</style>
