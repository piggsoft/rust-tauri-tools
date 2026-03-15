<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { usePasswordStore } from '../stores/password'
import PasswordForm from '../components/PasswordForm.vue'
import type { Password } from '../api/passwords'

const passwordStore = usePasswordStore()

const showPasswordForm = ref(false)
const showPassword = ref<Record<number, boolean>>({})
const copiedPassword = ref<number | null>(null)

onMounted(() => {
  passwordStore.fetchPasswords()
})

function openCreateForm() {
  passwordStore.openCreateForm()
}

function editPassword(password: Password) {
  passwordStore.selectPassword(password)
}

function deletePassword(password: Password) {
  if (confirm(`确定要删除这个密码吗？\n账号: ${password.account}`)) {
    passwordStore.deletePassword(password.id)
  }
}

function togglePasswordVisibility(id: number) {
  showPassword.value[id] = !showPassword.value[id]
}

async function copyPassword(password: string, id: number) {
  try {
    await navigator.clipboard.writeText(password)
    copiedPassword.value = id
    setTimeout(() => {
      copiedPassword.value = null
    }, 2000)
  } catch (error) {
    console.error('Failed to copy password:', error)
    alert('复制失败，请手动复制')
  }
}

function openUrl(url: string | null) {
  if (url) {
    window.open(url, '_blank')
  }
}

function handleFormClose() {
  passwordStore.closeForm()
}

function handleFormSubmit() {
  passwordStore.fetchPasswords()
}
</script>

<template>
  <div class="password-view">
    <!-- Header -->
    <div class="password-header">
      <div class="header-left">
        <h2 class="page-title">密码本</h2>
        <p class="page-subtitle">安全存储和管理您的账号密码</p>
      </div>
      <button class="btn-add" @click="openCreateForm">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="12" y1="5" x2="12" y2="19"></line>
          <line x1="5" y1="12" x2="19" y2="12"></line>
        </svg>
        新增密码
      </button>
    </div>

    <!-- Search and Filters -->
    <div class="search-filters">
      <div class="search-box">
        <svg class="search-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="11" cy="11" r="8"></circle>
          <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
        </svg>
        <input
          v-model="passwordStore.searchQuery"
          type="text"
          placeholder="搜索账号、分类或备注..."
          class="search-input"
        />
        <button
          v-if="passwordStore.searchQuery"
          class="btn-clear-search"
          @click="passwordStore.resetFilters"
          aria-label="清除搜索"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      </div>

      <div class="filter-dropdowns">
        <select
          v-model="passwordStore.selectedCategory"
          @change="passwordStore.setSubcategory('')"
          class="filter-select"
        >
          <option value="">所有分类</option>
          <option v-for="cat in passwordStore.categories" :key="cat" :value="cat">
            {{ cat }}
          </option>
        </select>

        <select
          v-if="passwordStore.selectedCategory"
          v-model="passwordStore.selectedSubcategory"
          class="filter-select"
        >
          <option value="">所有二级分类</option>
          <option v-for="sub in passwordStore.subcategories" :key="sub" :value="sub">
            {{ sub }}
          </option>
        </select>
      </div>
    </div>

    <!-- Password List -->
    <div class="password-list">
      <div v-if="passwordStore.loading" class="loading-state">
        <div class="spinner"></div>
        <p>加载中...</p>
      </div>

      <div v-else-if="passwordStore.error" class="error-state">
        <div class="error-icon">⚠️</div>
        <p>{{ passwordStore.error }}</p>
        <button class="btn-retry" @click="passwordStore.fetchPasswords">重试</button>
      </div>

      <div v-else-if="passwordStore.filteredPasswords.length === 0" class="empty-state">
        <div class="empty-icon">🔐</div>
        <h3>暂无密码</h3>
        <p v-if="passwordStore.searchQuery || passwordStore.selectedCategory">
          没有找到匹配的密码
        </p>
        <p v-else>点击"新增密码"开始添加您的第一个密码</p>
      </div>

      <div v-else class="password-cards">
        <div
          v-for="password in passwordStore.filteredPasswords"
          :key="password.id"
          class="password-card"
        >
          <div class="card-header">
            <div class="card-category">
              <span class="category-badge">{{ password.category }}</span>
              <span v-if="password.subcategory" class="subcategory-badge">
                / {{ password.subcategory }}
              </span>
            </div>
            <div class="card-actions">
              <button
                class="btn-icon"
                @click="editPassword(password)"
                title="编辑"
                aria-label="编辑密码"
              >
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path>
                  <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path>
                </svg>
              </button>
              <button
                class="btn-icon btn-danger"
                @click="deletePassword(password)"
                title="删除"
                aria-label="删除密码"
              >
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="3 6 5 6 21 6"></polyline>
                  <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
                </svg>
              </button>
            </div>
          </div>

          <div class="card-body">
            <div class="account-info">
              <div class="account-label">账号</div>
              <div class="account-value">{{ password.account }}</div>
            </div>

            <div class="password-info">
              <div class="password-label">
                <span>密码</span>
                <button
                  class="btn-copy"
                  @click="copyPassword(password.password, password.id)"
                  :class="{ 'copied': copiedPassword === password.id }"
                  :aria-label="copiedPassword === password.id ? '已复制' : '复制密码'"
                >
                  <svg v-if="copiedPassword !== password.id" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                    <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
                  </svg>
                  <svg v-else width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline points="20 6 9 17 4 12"></polyline>
                  </svg>
                  {{ copiedPassword === password.id ? '已复制' : '复制' }}
                </button>
              </div>
              <div class="password-value-wrapper">
                <input
                  :type="showPassword[password.id] ? 'text' : 'password'"
                  :value="password.password"
                  readonly
                  class="password-input"
                />
                <button
                  class="btn-toggle"
                  @click="togglePasswordVisibility(password.id)"
                  :aria-label="showPassword[password.id] ? '隐藏密码' : '显示密码'"
                >
                  <svg v-if="showPassword[password.id]" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M1 1l22 22"></path>
                  </svg>
                  <svg v-else width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path>
                    <circle cx="12" cy="12" r="3"></circle>
                  </svg>
                </button>
              </div>
            </div>

            <div v-if="password.login_url" class="url-info">
              <button class="url-link" @click="openUrl(password.login_url)">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"></path>
                  <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"></path>
                </svg>
                打开登录页面
              </button>
            </div>

            <div v-if="password.notes" class="notes-info">
              <div class="notes-label">备注</div>
              <div class="notes-value">{{ password.notes }}</div>
            </div>
          </div>

          <div class="card-footer">
            <div class="timestamp">
              {{ new Date(password.updated_at).toLocaleString('zh-CN', {
                year: 'numeric',
                month: '2-digit',
                day: '2-digit',
                hour: '2-digit',
                minute: '2-digit'
              }) }}
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Password Form Modal -->
    <Transition name="modal">
      <PasswordForm
        v-if="passwordStore.showForm"
        :password="passwordStore.selectedPassword"
        @close="handleFormClose"
        @submit="handleFormSubmit"
      />
    </Transition>
  </div>
</template>

<style scoped>
.password-view {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: #f5f5f5;
  overflow: hidden;
}

.password-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 24px;
  background: white;
  border-bottom: 1px solid #e0e0e0;
}

.header-left {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.page-title {
  font-size: 24px;
  font-weight: 600;
  color: #262626;
  margin: 0;
}

.page-subtitle {
  font-size: 14px;
  color: #8c8c8c;
  margin: 0;
}

.btn-add {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 10px 20px;
  background: #1890ff;
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-add:hover {
  background: #40a9ff;
}

.search-filters {
  padding: 16px 24px;
  background: white;
  border-bottom: 1px solid #e0e0e0;
  display: flex;
  gap: 16px;
  align-items: center;
}

.search-box {
  flex: 1;
  position: relative;
  display: flex;
  align-items: center;
}

.search-icon {
  position: absolute;
  left: 12px;
  color: #8c8c8c;
}

.search-input {
  width: 100%;
  padding: 10px 36px 10px 36px;
  border: 1px solid #d9d9d9;
  border-radius: 6px;
  font-size: 14px;
  color: #262626;
  transition: all 0.2s;
}

.search-input:focus {
  outline: none;
  border-color: #1890ff;
  box-shadow: 0 0 0 2px rgba(24, 144, 255, 0.1);
}

.search-input::placeholder {
  color: #bfbfbf;
}

.btn-clear-search {
  position: absolute;
  right: 8px;
  width: 20px;
  height: 20px;
  border: none;
  background: #f5f5f5;
  border-radius: 50%;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #8c8c8c;
  transition: all 0.2s;
}

.btn-clear-search:hover {
  background: #e0e0e0;
  color: #262626;
}

.filter-dropdowns {
  display: flex;
  gap: 12px;
}

.filter-select {
  padding: 10px 32px 10px 12px;
  border: 1px solid #d9d9d9;
  border-radius: 6px;
  font-size: 14px;
  color: #262626;
  background: white;
  cursor: pointer;
  transition: all 0.2s;
  appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%238c8c8c' stroke-width='2'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 10px center;
  padding-right: 32px;
  min-width: 150px;
}

.filter-select:focus {
  outline: none;
  border-color: #1890ff;
  box-shadow: 0 0 0 2px rgba(24, 144, 255, 0.1);
}

.password-list {
  flex: 1;
  overflow-y: auto;
  padding: 16px 24px;
}

.password-list::-webkit-scrollbar {
  width: 8px;
}

.password-list::-webkit-scrollbar-thumb {
  background: #d9d9d9;
  border-radius: 4px;
}

.password-list::-webkit-scrollbar-thumb:hover {
  background: #bfbfbf;
}

.loading-state,
.error-state,
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  color: #8c8c8c;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 3px solid #f0f0f0;
  border-top-color: #1890ff;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
  margin-bottom: 16px;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.error-icon,
.empty-icon {
  font-size: 48px;
  margin-bottom: 16px;
}

.error-state p,
.empty-state p {
  margin: 0 0 16px 0;
  font-size: 14px;
}

.empty-state h3 {
  margin: 0 0 8px 0;
  font-size: 18px;
  color: #262626;
}

.btn-retry {
  padding: 8px 16px;
  background: #1890ff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
}

.password-cards {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.password-card {
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
  transition: all 0.2s;
  overflow: hidden;
}

.password-card:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: 16px;
  background: #fafafa;
  border-bottom: 1px solid #f0f0f0;
}

.card-category {
  display: flex;
  align-items: center;
  gap: 6px;
  flex-wrap: wrap;
}

.category-badge {
  padding: 4px 10px;
  background: #e6f7ff;
  color: #1890ff;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
}

.subcategory-badge {
  font-size: 12px;
  color: #8c8c8c;
}

.card-actions {
  display: flex;
  gap: 8px;
}

.btn-icon {
  width: 32px;
  height: 32px;
  border: none;
  background: white;
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #595959;
  transition: all 0.2s;
}

.btn-icon:hover {
  background: #f5f5f5;
  color: #1890ff;
}

.btn-icon.btn-danger:hover {
  background: #fff1f0;
  color: #ff4d4f;
}

.card-body {
  padding: 16px;
}

.account-info {
  margin-bottom: 16px;
}

.account-label {
  font-size: 12px;
  color: #8c8c8c;
  margin-bottom: 6px;
}

.account-value {
  font-size: 15px;
  font-weight: 500;
  color: #262626;
}

.password-info {
  margin-bottom: 12px;
}

.password-label {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 6px;
}

.password-label span {
  font-size: 12px;
  color: #8c8c8c;
}

.btn-copy {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 10px;
  background: #f5f5f5;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
  color: #595959;
  transition: all 0.2s;
}

.btn-copy:hover {
  background: #e0e0e0;
  color: #1890ff;
}

.btn-copy.copied {
  background: #f6ffed;
  color: #52c41a;
}

.password-value-wrapper {
  position: relative;
  display: flex;
  align-items: center;
}

.password-input {
  width: 100%;
  padding: 8px 40px 8px 12px;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  font-size: 14px;
  font-family: 'Courier New', monospace;
  color: #262626;
  background: #fafafa;
}

.btn-toggle {
  position: absolute;
  right: 8px;
  width: 32px;
  height: 32px;
  border: none;
  background: transparent;
  cursor: pointer;
  color: #8c8c8c;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.btn-toggle:hover {
  color: #1890ff;
}

.url-info {
  margin-bottom: 12px;
}

.url-link {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background: #e6f7ff;
  color: #1890ff;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
  font-weight: 500;
  transition: all 0.2s;
}

.url-link:hover {
  background: #bae7ff;
}

.notes-info {
  padding-top: 12px;
  border-top: 1px solid #f0f0f0;
}

.notes-label {
  font-size: 12px;
  color: #8c8c8c;
  margin-bottom: 6px;
}

.notes-value {
  font-size: 14px;
  color: #595959;
  line-height: 1.5;
  white-space: pre-wrap;
}

.card-footer {
  padding: 12px 16px;
  background: #fafafa;
  border-top: 1px solid #f0f0f0;
}

.timestamp {
  font-size: 12px;
  color: #bfbfbf;
}

.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

/* Responsive */
@media (max-width: 768px) {
  .password-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 16px;
    padding: 16px;
  }

  .page-title {
    font-size: 20px;
  }

  .search-filters {
    flex-direction: column;
    align-items: stretch;
    padding: 12px 16px;
  }

  .filter-dropdowns {
    flex-direction: column;
  }

  .filter-select {
    min-width: 100%;
  }

  .password-list {
    padding: 12px 16px;
  }

  .card-header {
    padding: 12px;
  }

  .card-body {
    padding: 12px;
  }
}

@media (max-width: 480px) {
  .password-header {
    padding: 12px;
  }

  .page-title {
    font-size: 18px;
  }

  .search-filters {
    padding: 10px 12px;
  }

  .search-input {
    padding: 8px 32px 8px 32px;
    font-size: 13px;
  }

  .password-list {
    padding: 8px 12px;
  }

  .card-category {
    flex-direction: column;
    align-items: flex-start;
    gap: 4px;
  }

  .card-actions {
    margin-top: 8px;
  }

  .account-value {
    font-size: 14px;
  }

  .password-input {
    font-size: 13px;
  }
}
</style>
