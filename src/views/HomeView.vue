<script setup lang="ts">
import { ref } from 'vue'

const emit = defineEmits<{
  navigate: [page: 'todo' | 'password']
}>()

interface Feature {
  id: string
  name: string
  description: string
  icon: string
  color: string
}

const features: Feature[] = [
  {
    id: 'todo',
    name: '待办事项',
    description: '管理您的任务、子任务,支持搜索筛选和多种视图',
    icon: '✓',
    color: '#1890ff',
  },
  {
    id: 'password',
    name: '密码本',
    description: '安全存储和管理您的账号密码,支持分类搜索',
    icon: '🔐',
    color: '#52c41a',
  },
]

function handleFeatureClick(feature: Feature) {
  emit('navigate', feature.id as any)
}
</script>

<template>
  <div class="home-view">
    <div class="home-content">
      <div class="welcome-section">
        <h1 class="welcome-title">欢迎使用个人工具集</h1>
        <p class="welcome-subtitle">高效管理您的日常任务</p>
      </div>

      <div class="features-grid">
        <div
          v-for="feature in features"
          :key="feature.id"
          class="feature-card"
          :style="{ '--feature-color': feature.color }"
          @click="handleFeatureClick(feature)"
        >
          <div class="feature-icon" :style="{ background: feature.color }">
            {{ feature.icon }}
          </div>
          <div class="feature-info">
            <h3 class="feature-name">{{ feature.name }}</h3>
            <p class="feature-description">{{ feature.description }}</p>
          </div>
          <div class="feature-arrow">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="5" y1="12" x2="19" y2="12"></line>
              <polyline points="12 5 19 12 12 19"></polyline>
            </svg>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.home-view {
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
}

.home-content {
  width: 100%;
  max-width: 900px;
}

.welcome-section {
  text-align: center;
  color: white;
  margin-bottom: 48px;
}

.welcome-title {
  font-size: 42px;
  font-weight: 700;
  margin: 0 0 12px 0;
  letter-spacing: -0.5px;
}

.welcome-subtitle {
  font-size: 18px;
  margin: 0;
  opacity: 0.9;
}

.features-grid {
  display: grid;
  gap: 20px;
}

.feature-card {
  background: white;
  border-radius: 16px;
  padding: 32px;
  display: flex;
  align-items: center;
  gap: 24px;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
}

.feature-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
}

.feature-icon {
  width: 72px;
  height: 72px;
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 32px;
  color: white;
  flex-shrink: 0;
}

.feature-info {
  flex: 1;
}

.feature-name {
  font-size: 24px;
  font-weight: 600;
  color: #262626;
  margin: 0 0 8px 0;
}

.feature-description {
  font-size: 14px;
  color: #8c8c8c;
  margin: 0;
  line-height: 1.6;
}

.feature-arrow {
  color: var(--feature-color);
  flex-shrink: 0;
}

/* Responsive */
@media (max-width: 768px) {
  .welcome-title {
    font-size: 32px;
  }

  .welcome-subtitle {
    font-size: 16px;
  }

  .feature-card {
    padding: 24px;
    flex-direction: column;
    text-align: center;
    gap: 16px;
  }

  .feature-icon {
    width: 64px;
    height: 64px;
    font-size: 28px;
  }

  .feature-name {
    font-size: 20px;
  }

  .feature-description {
    font-size: 13px;
  }

  .feature-arrow {
    transform: rotate(90deg);
  }
}

@media (max-width: 480px) {
  .welcome-section {
    margin-bottom: 32px;
  }

  .welcome-title {
    font-size: 28px;
  }

  .welcome-subtitle {
    font-size: 14px;
  }

  .feature-card {
    padding: 20px;
    gap: 12px;
  }

  .feature-icon {
    width: 56px;
    height: 56px;
    font-size: 24px;
  }

  .feature-name {
    font-size: 18px;
  }

  .feature-description {
    font-size: 12px;
  }
}
</style>
