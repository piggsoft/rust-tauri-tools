<!--
  Sidebar Component
  
  Unified navigation sidebar for the entire application.
  
  Props:
  - currentPage: Current active page ('home' | 'todo' | 'password')
  
  Emits:
  - navigate: Event triggered when navigation item is clicked with target page
  
  Features:
  - Collapsible sidebar with hover-to-expand behavior
  - Active page highlighting with blue indicator
  - Three navigation items: Home, Todo, Password
  - Responsive design for mobile/tablet
  
  Usage:
  <Sidebar :current-page="'todo'" @navigate="handleNavigate" />
-->
<script setup lang="ts">
import { ref } from 'vue'

const props = defineProps<{
  currentPage: 'home' | 'todo' | 'password'
}>()

const emit = defineEmits<{
  navigate: [page: 'home' | 'todo' | 'password']
  expand: [expanded: boolean]
}>()

const isExpanded = ref(false)

function navigateTo(page: 'home' | 'todo' | 'password') {
  emit('navigate', page)
}

function handleMouseEnter() {
  isExpanded.value = true
  emit('expand', true)
}

function handleMouseLeave() {
  isExpanded.value = false
  emit('expand', false)
}
</script>

<template>
  <nav
    class="sidebar"
    @mouseenter="handleMouseEnter"
    @mouseleave="handleMouseLeave"
  >
    <div class="sidebar-content">
      <button
        class="nav-item home-nav"
        :class="{ active: currentPage === 'home' }"
        @click="navigateTo('home')"
        title="返回首页"
      >
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path>
          <polyline points="9 22 9 12 15 12 15 22"></polyline>
        </svg>
        <span class="nav-text">首页</span>
      </button>

      <button
        class="nav-item todo-nav"
        :class="{ active: currentPage === 'todo' }"
        @click="navigateTo('todo')"
        title="Todo任务管理"
      >
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M9 11l3 3L22 4"></path>
          <path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11"></path>
        </svg>
        <span class="nav-text">Todo</span>
      </button>

      <button
        class="nav-item password-nav"
        :class="{ active: currentPage === 'password' }"
        @click="navigateTo('password')"
        title="密码本"
      >
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect>
          <path d="M7 11V7a5 5 0 0 1 10 0v4"></path>
        </svg>
        <span class="nav-text">密码本</span>
      </button>
    </div>
  </nav>
</template>

<style scoped>
.sidebar {
  position: fixed;
  left: 0;
  top: 0;
  height: 100%;
  background: white;
  border-right: 1px solid #e0e0e0;
  box-shadow: 2px 0 8px rgba(0, 0, 0, 0.08);
  z-index: 1000;
  width: 60px;
  transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.sidebar:hover {
  width: 200px;
}

.sidebar-content {
  display: flex;
  flex-direction: column;
  padding: 16px 0;
  height: 100%;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 12px 16px;
  width: 100%;
  border: none;
  background: transparent;
  color: #595959;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  white-space: nowrap;
  overflow: hidden;
}

.nav-item:hover {
  background: #f5f5f5;
  color: #1890ff;
}

.nav-item:active {
  background: #e6f7ff;
}

.nav-item:active:hover {
  color: #1890ff;
}

.nav-item.active {
  background: #e6f7ff;
  color: #1890ff;
  border-left: 3px solid #1890ff;
  padding-left: 13px;
}

.nav-item svg {
  flex-shrink: 0;
  width: 20px;
  height: 20px;
}

.nav-text {
  opacity: 0;
  transition: opacity 0.3s;
}

.sidebar:hover .nav-text {
  opacity: 1;
}

/* Responsive */
@media (max-width: 768px) {
  .sidebar {
    width: 50px;
  }

  .sidebar:hover {
    width: 160px;
  }

  .sidebar-content {
    padding: 12px 0;
  }

  .nav-item {
    padding: 10px 14px;
    font-size: 13px;
  }

  .nav-item.active {
    padding-left: 11px;
  }

  .nav-item svg {
    width: 18px;
    height: 18px;
  }
}

@media (max-width: 480px) {
  .sidebar {
    width: 44px;
  }

  .sidebar:hover {
    width: 140px;
  }

  .sidebar-content {
    padding: 10px 0;
  }

  .nav-item {
    padding: 8px 12px;
    font-size: 12px;
  }

  .nav-item.active {
    padding-left: 9px;
  }

  .nav-item svg {
    width: 16px;
    height: 16px;
  }
}
</style>
