<!--
  PasswordForm Component
  
  A modal form for creating and editing password entries.
  
  Props:
  - password: Optional existing Password for editing mode (null for new password)
  
  Emits:
  - close: Form closed without saving
  - submit: Password successfully saved/created
  
  Features:
  - Password category and subcategory inputs
  - Account and password fields
  - Password visibility toggle
  - Login URL field
  - Notes textarea
  - Form validation
  
  Usage:
  <PasswordForm :password="existingPassword" @submit="handleSubmit" @close="handleClose" />
-->
<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { usePasswordStore } from '../stores/password'
import type { PasswordInput, Password } from '../api/passwords'

const props = defineProps<{
  password?: Password
}>()

const emit = defineEmits<{
  close: []
  submit: []
}>()

const passwordStore = usePasswordStore()

const form = ref<PasswordInput>({
  category: '',
  subcategory: '',
  account: '',
  password: '',
  login_url: '',
  notes: '',
})

const showPassword = ref(false)
const isSubmitting = ref(false)

// Watch for password prop changes (edit mode)
watch(() => props.password, (newPassword) => {
  if (newPassword) {
    form.value = {
      category: newPassword.category,
      subcategory: newPassword.subcategory || '',
      account: newPassword.account,
      password: newPassword.password,
      login_url: newPassword.login_url || '',
      notes: newPassword.notes || '',
    }
  } else {
    form.value = {
      category: '',
      subcategory: '',
      account: '',
      password: '',
      login_url: '',
      notes: '',
    }
  }
}, { immediate: true })

function togglePasswordVisibility() {
  showPassword.value = !showPassword.value
}

async function handleSubmit() {
  if (!form.value.category || !form.value.account || !form.value.password) {
    alert('请填写必填字段')
    return
  }

  isSubmitting.value = true
  try {
    if (props.password?.id) {
      await passwordStore.updatePassword(props.password.id, form.value)
    } else {
      await passwordStore.createPassword(form.value)
    }
    emit('submit')
    emit('close')
  } catch (error) {
    console.error('Failed to save password:', error)
    alert('保存失败，请重试')
  } finally {
    isSubmitting.value = false
  }
}

function handleCancel() {
  emit('close')
}
</script>

<template>
  <div class="password-form-overlay" @click.self="handleCancel">
    <div class="password-form-container">
      <div class="form-header">
        <h2>{{ password?.id ? '编辑密码' : '新增密码' }}</h2>
        <button class="btn-close" @click="handleCancel" aria-label="关闭">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      </div>

      <form @submit.prevent="handleSubmit" class="password-form">
        <div class="form-group">
          <label for="category">密码分类 *</label>
          <input
            id="category"
            v-model="form.category"
            type="text"
            required
            placeholder="例如：社交媒体、工作、购物"
            class="form-input"
          />
        </div>

        <div class="form-group">
          <label for="subcategory">二级分类</label>
          <input
            id="subcategory"
            v-model="form.subcategory"
            type="text"
            placeholder="例如：Google、Twitter"
            class="form-input"
          />
        </div>

        <div class="form-group">
          <label for="account">账号 *</label>
          <input
            id="account"
            v-model="form.account"
            type="text"
            required
            placeholder="用户名或邮箱"
            class="form-input"
          />
        </div>

        <div class="form-group">
          <label for="password">密码 *</label>
          <div class="password-input-wrapper">
            <input
              id="password"
              v-model="form.password"
              :type="showPassword ? 'text' : 'password'"
              required
              placeholder="输入密码"
              class="form-input password-input"
            />
            <button
              type="button"
              class="btn-toggle-password"
              @click="togglePasswordVisibility"
              :aria-label="showPassword ? '隐藏密码' : '显示密码'"
            >
              <svg v-if="showPassword" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M1 1l22 22"></path>
                <path d="M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"></path>
              </svg>
              <svg v-else width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path>
                <circle cx="12" cy="12" r="3"></circle>
              </svg>
            </button>
          </div>
        </div>

        <div class="form-group">
          <label for="login_url">登录地址</label>
          <input
            id="login_url"
            v-model="form.login_url"
            type="text"
            placeholder="https://example.com/login"
            class="form-input"
          />
        </div>

        <div class="form-group">
          <label for="notes">备注</label>
          <textarea
            id="notes"
            v-model="form.notes"
            rows="3"
            placeholder="其他备注信息"
            class="form-input form-textarea"
          ></textarea>
        </div>

        <div class="form-actions">
          <button type="button" class="btn btn-secondary" @click="handleCancel" :disabled="isSubmitting">
            取消
          </button>
          <button type="submit" class="btn btn-primary" :disabled="isSubmitting">
            {{ isSubmitting ? '保存中...' : (password?.id ? '更新' : '保存') }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<style scoped>
.password-form-overlay {
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

.password-form-container {
  background: white;
  border-radius: 12px;
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.15);
  width: 100%;
  max-width: 500px;
  max-height: 90vh;
  overflow-y: auto;
}

.form-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid #f0f0f0;
}

.form-header h2 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: #262626;
}

.btn-close {
  width: 32px;
  height: 32px;
  border: none;
  background: transparent;
  cursor: pointer;
  border-radius: 4px;
  color: #8c8c8c;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.btn-close:hover {
  background: #f5f5f5;
  color: #262626;
}

.password-form {
  padding: 24px;
}

.form-group {
  margin-bottom: 20px;
}

.form-group label {
  display: block;
  font-size: 14px;
  font-weight: 500;
  color: #262626;
  margin-bottom: 8px;
}

.form-input {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid #d9d9d9;
  border-radius: 6px;
  font-size: 14px;
  color: #262626;
  transition: all 0.2s;
}

.form-input:focus {
  outline: none;
  border-color: #1890ff;
  box-shadow: 0 0 0 2px rgba(24, 144, 255, 0.1);
}

.form-input::placeholder {
  color: #bfbfbf;
}

.form-textarea {
  resize: vertical;
  min-height: 80px;
}

.password-input-wrapper {
  position: relative;
  display: flex;
  align-items: center;
}

.password-input {
  padding-right: 48px;
}

.btn-toggle-password {
  position: absolute;
  right: 8px;
  width: 32px;
  height: 32px;
  border: none;
  background: transparent;
  cursor: pointer;
  border-radius: 4px;
  color: #8c8c8c;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.btn-toggle-password:hover {
  background: #f5f5f5;
  color: #1890ff;
}

.form-actions {
  display: flex;
  gap: 12px;
  margin-top: 24px;
}

.btn {
  flex: 1;
  padding: 10px 20px;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-primary {
  background: #1890ff;
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: #40a9ff;
}

.btn-primary:disabled {
  background: #bfbfbf;
  cursor: not-allowed;
}

.btn-secondary {
  background: white;
  border: 1px solid #d9d9d9;
  color: #595959;
}

.btn-secondary:hover:not(:disabled) {
  border-color: #40a9ff;
  color: #40a9ff;
}

.btn-secondary:disabled {
  background: #fafafa;
  cursor: not-allowed;
}

/* Responsive */
@media (max-width: 480px) {
  .password-form-overlay {
    padding: 10px;
  }

  .password-form-container {
    max-height: 95vh;
  }

  .form-header {
    padding: 16px;
  }

  .form-header h2 {
    font-size: 18px;
  }

  .password-form {
    padding: 16px;
  }

  .form-group {
    margin-bottom: 16px;
  }

  .form-actions {
    flex-direction: column;
  }
}
</style>
