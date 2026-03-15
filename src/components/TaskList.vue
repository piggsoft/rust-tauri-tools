<script setup lang="ts">
import { ref, computed } from 'vue'
import { useTaskStore } from '../stores/task'
import TaskItem from './TaskItem.vue'
import TaskForm from './TaskForm.vue'
import type { Task } from '../types'

const taskStore = useTaskStore()

const showTaskForm = ref(false)
const editingTask = ref<Task | null>(null)

// Extract date calculations to separate computed to avoid recalculation
const dateReferences = computed(() => {
  const now = new Date()
  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate())
  const tomorrow = new Date(today)
  tomorrow.setDate(tomorrow.getDate() + 1)
  const weekEnd = new Date(today.getTime() + 7 * 24 * 60 * 60 * 1000)
  
  return { today, tomorrow, weekEnd }
})

const groupedTasks = computed(() => {
  const pending = taskStore.tasks.filter(t => t.status === 'pending')
  const completed = taskStore.tasks.filter(t => t.status === 'completed')

  // Use cached date references
  const { today, tomorrow, weekEnd } = dateReferences.value

  const overdue: Task[] = []
  const todayTasks: Task[] = []
  const thisWeek: Task[] = []
  const later: Task[] = []

  pending.forEach(task => {
    if (!task.due_date) {
      later.push(task)
      return
    }

    const dueDate = new Date(task.due_date)
    if (dueDate < today) {
      overdue.push(task)
    } else if (dueDate.toDateString() === today.toDateString()) {
      todayTasks.push(task)
    } else if (dueDate <= weekEnd) {
      thisWeek.push(task)
    } else {
      later.push(task)
    }
  })

  return {
    overdue,
    today: todayTasks,
    thisWeek,
    later,
    completed,
  }
})

function openTaskForm(task?: Task) {
  editingTask.value = task || null
  showTaskForm.value = true
}

function closeTaskForm() {
  showTaskForm.value = false
  editingTask.value = null
}

function handleTaskSaved() {
  closeTaskForm()
  taskStore.fetchTasks()
}

function handleTaskDeleted() {
  closeTaskForm()
  taskStore.fetchTasks()
}
</script>

<template>
  <div class="task-list">
    <!-- Toolbar -->
    <div class="toolbar">
      <div class="toolbar-left">
        <div class="batch-actions" v-if="taskStore.selectedTasks.size > 0">
          <span class="selected-count">{{ taskStore.selectedTasks.size }} 已选择</span>
          <button class="btn-small btn-complete" @click="taskStore.batchComplete()">完成</button>
          <button class="btn-small btn-archive" @click="taskStore.batchArchive()">归档</button>
          <button class="btn-small btn-delete" @click="taskStore.batchDelete()">删除</button>
          <button class="btn-small" @click="taskStore.clearSelection()">取消选择</button>
        </div>
      </div>

      <div class="toolbar-right">
        <span v-if="taskStore.tasks.length > 0" class="task-count">{{ taskStore.tasks.length }} 个任务</span>
      </div>
    </div>

    <!-- Task Groups -->
    <div class="task-groups">
      <!-- Overdue -->
      <div v-if="groupedTasks.overdue.length > 0" class="task-group">
        <h3 class="group-header">
          <span class="group-title">已过期</span>
          <span class="group-count">{{ groupedTasks.overdue.length }}</span>
        </h3>
        <div class="task-items">
          <TaskItem
            v-for="task in groupedTasks.overdue"
            :key="task.id"
            :task="task"
            @edit="openTaskForm"
          />
        </div>
      </div>

      <!-- Today -->
      <div v-if="groupedTasks.today.length > 0" class="task-group">
        <h3 class="group-header">
          <span class="group-title">今天</span>
          <span class="group-count">{{ groupedTasks.today.length }}</span>
        </h3>
        <div class="task-items">
          <TaskItem
            v-for="task in groupedTasks.today"
            :key="task.id"
            :task="task"
            @edit="openTaskForm"
          />
        </div>
      </div>

      <!-- This Week -->
      <div v-if="groupedTasks.thisWeek.length > 0" class="task-group">
        <h3 class="group-header">
          <span class="group-title">本周</span>
          <span class="group-count">{{ groupedTasks.thisWeek.length }}</span>
        </h3>
        <div class="task-items">
          <TaskItem
            v-for="task in groupedTasks.thisWeek"
            :key="task.id"
            :task="task"
            @edit="openTaskForm"
          />
        </div>
      </div>

      <!-- Later -->
      <div v-if="groupedTasks.later.length > 0" class="task-group">
        <h3 class="group-header">
          <span class="group-title">未来</span>
          <span class="group-count">{{ groupedTasks.later.length }}</span>
        </h3>
        <div class="task-items">
          <TaskItem
            v-for="task in groupedTasks.later"
            :key="task.id"
            :task="task"
            @edit="openTaskForm"
          />
        </div>
      </div>

      <!-- Completed -->
      <div v-if="groupedTasks.completed.length > 0" class="task-group">
        <h3 class="group-header completed">
          <span class="group-title">已完成</span>
          <span class="group-count">{{ groupedTasks.completed.length }}</span>
        </h3>
        <div class="task-items completed">
          <TaskItem
            v-for="task in groupedTasks.completed"
            :key="task.id"
            :task="task"
            @edit="openTaskForm"
          />
        </div>
      </div>

      <!-- Empty State -->
      <div v-if="groupedTasks.overdue.length === 0 &&
                   groupedTasks.today.length === 0 &&
                   groupedTasks.thisWeek.length === 0 &&
                   groupedTasks.later.length === 0 &&
                   groupedTasks.completed.length === 0"
        class="empty-state">
        <div class="empty-icon">📝</div>
        <h3>暂无任务</h3>
        <p>点击上方"新建任务"按钮开始创建你的第一个任务</p>
      </div>
    </div>

    <!-- Task Form Modal -->
    <TaskForm
      v-if="showTaskForm"
      :task="editingTask"
      @close="closeTaskForm"
      @saved="handleTaskSaved"
      @deleted="handleTaskDeleted"
    />
  </div>
</template>

<style scoped>
.task-list {
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 12px;
  overflow: hidden;
}

.toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 14px;
  background: white;
  border-radius: 6px;
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
  flex-wrap: wrap;
  gap: 8px;
}

.toolbar-left {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
  flex: 1;
  min-width: 0;
}

.btn-filter {
  padding: 5px 12px;
  border: 1px solid #d9d9d9;
  background: white;
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
  color: #595959;
  transition: all 0.2s;
}

.btn-filter:hover {
  border-color: #1890ff;
  color: #1890ff;
}

.btn-filter.active {
  background: #e6f7ff;
  border-color: #1890ff;
  color: #1890ff;
}

.batch-actions {
  display: flex;
  align-items: center;
  gap: 6px;
  padding-left: 12px;
  border-left: 1px solid #e0e0e0;
  flex-wrap: wrap;
}

.selected-count {
  font-size: 13px;
  color: #1890ff;
  font-weight: 500;
  white-space: nowrap;
}

.btn-small {
  padding: 3px 10px;
  border: 1px solid #d9d9d9;
  background: white;
  border-radius: 3px;
  cursor: pointer;
  font-size: 12px;
  color: #595959;
  transition: all 0.2s;
  white-space: nowrap;
}

.btn-small:hover {
  border-color: #1890ff;
  color: #1890ff;
}

.btn-complete {
  background: #52c41a;
  border-color: #52c41a;
  color: white;
}

.btn-complete:hover {
  background: #73d13d;
  border-color: #73d13d;
}

.btn-archive {
  background: #faad14;
  border-color: #faad14;
  color: white;
}

.btn-archive:hover {
  background: #ffc53d;
  border-color: #ffc53d;
}

.btn-delete {
  background: #ff4d4f;
  border-color: #ff4d4f;
  color: white;
}

.btn-delete:hover {
  background: #ff7875;
  border-color: #ff7875;
}

.toolbar-right {
  display: flex;
  gap: 8px;
  align-items: center;
}

.task-count {
  font-size: 13px;
  color: #8c8c8c;
  white-space: nowrap;
}

.task-groups {
  flex: 1;
  overflow-y: auto;
  padding-right: 4px;
}

.task-groups::-webkit-scrollbar {
  width: 6px;
}

.task-groups::-webkit-scrollbar-thumb {
  background: #d9d9d9;
  border-radius: 3px;
}

.task-groups::-webkit-scrollbar-thumb:hover {
  background: #bfbfbf;
}

.task-group {
  background: white;
  border-radius: 6px;
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
  overflow: hidden;
  margin-bottom: 10px;
}

.group-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 14px;
  background: #fafafa;
  border-bottom: 1px solid #f0f0f0;
  font-size: 13px;
  font-weight: 600;
  color: #262626;
}

.group-header.completed {
  background: #f6ffed;
}

.group-title {
  text-transform: capitalize;
}

.group-count {
  font-size: 12px;
  color: #8c8c8c;
  font-weight: 400;
  background: #f0f0f0;
  padding: 2px 6px;
  border-radius: 8px;
}

.task-items {
  padding: 6px 0;
}

.task-items.completed {
  opacity: 0.7;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px 16px;
  background: white;
  border-radius: 6px;
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
}

.empty-icon {
  font-size: 40px;
  margin-bottom: 12px;
}

.empty-state h3 {
  font-size: 16px;
  color: #262626;
  margin-bottom: 6px;
}

.empty-state p {
  font-size: 13px;
  color: #8c8c8c;
  text-align: center;
}

/* Responsive */
@media (max-width: 768px) {
  .toolbar {
    padding: 8px 10px;
  }

  .batch-actions {
    width: 100%;
    padding: 8px 0 0 0;
    border-left: none;
    border-top: 1px solid #e0e0e0;
    justify-content: flex-start;
  }

  .btn-small {
    font-size: 11px;
    padding: 4px 8px;
  }

  .task-group {
    margin-bottom: 8px;
  }

  .group-header {
    padding: 8px 10px;
  }

  .empty-state {
    padding: 30px 12px;
  }
}
</style>
