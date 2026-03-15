<script setup lang="ts">
import { ref } from 'vue'
import { useTaskStore } from './stores/task'
import { usePasswordStore } from './stores/password'
import HomeView from './views/HomeView.vue'
import TaskView from './views/TaskView.vue'
import PasswordView from './views/PasswordView.vue'
import Sidebar from './components/Sidebar.vue'

const taskStore = useTaskStore()
const passwordStore = usePasswordStore()
const currentPage = ref<'home' | 'todo' | 'password'>('home')
const isSidebarExpanded = ref(false)

function navigateTo(page: 'home' | 'todo' | 'password') {
  currentPage.value = page
  if (page === 'todo') {
    taskStore.fetchTasks()
  } else if (page === 'password') {
    passwordStore.fetchPasswords()
  }
}
</script>

<template>
  <div id="app">
    <HomeView v-if="currentPage === 'home'" @navigate="(page: 'todo' | 'password') => navigateTo(page)" />

    <div v-else-if="currentPage === 'todo'" class="todo-page">
      <Sidebar
        :current-page="currentPage"
        @navigate="navigateTo"
        @expand="isSidebarExpanded = $event"
      />
      <div class="todo-content" :class="{ expanded: isSidebarExpanded }">
        <TaskView />
      </div>
    </div>

    <div v-else-if="currentPage === 'password'" class="password-page">
      <Sidebar
        :current-page="currentPage"
        @navigate="navigateTo"
        @expand="isSidebarExpanded = $event"
      />
      <div class="todo-content" :class="{ expanded: isSidebarExpanded }">
        <PasswordView />
      </div>
    </div>
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

#app {
  width: 100vw;
  height: 100vh;
  overflow: hidden;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

.todo-page,
.password-page {
  width: 100%;
  height: 100%;
  display: flex;
}

/* Main Content */
.todo-content {
  flex: 1;
  margin-left: 60px;
  width: calc(100% - 60px);
  transition: margin-left 0.3s cubic-bezier(0.4, 0, 0.2, 1), width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.todo-content.expanded {
  margin-left: 200px;
  width: calc(100% - 200px);
}

/* Responsive */
@media (max-width: 768px) {
  .todo-content {
    margin-left: 50px;
    width: calc(100% - 50px);
  }

  .todo-content.expanded {
    margin-left: 160px;
    width: calc(100% - 160px);
  }
}

@media (max-width: 480px) {
  .todo-content {
    margin-left: 44px;
    width: calc(100% - 44px);
  }

  .todo-content.expanded {
    margin-left: 140px;
    width: calc(100% - 140px);
  }
}
</style>
