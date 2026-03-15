import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { passwordApi, type Password, type PasswordInput, type PasswordFilter } from '../api/passwords'

export const usePasswordStore = defineStore('password', () => {
  // State
  const passwords = ref<Password[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)
  const filter = ref<PasswordFilter>({})
  const selectedPassword = ref<Password | undefined>(undefined)
  const showForm = ref(false)
  const searchQuery = ref('')
  const selectedCategory = ref<string>('')
  const selectedSubcategory = ref<string>('')

  // Computed - Break down into smaller computed properties for better performance
  const passwordsMatchingSearch = computed(() => {
    if (!searchQuery.value) return passwords.value
    const query = searchQuery.value.toLowerCase()
    return passwords.value.filter(p =>
      p.account.toLowerCase().includes(query) ||
      p.category.toLowerCase().includes(query) ||
      (p.subcategory && p.subcategory.toLowerCase().includes(query)) ||
      (p.notes && p.notes.toLowerCase().includes(query))
    )
  })

  const passwordsMatchingCategory = computed(() => {
    if (!selectedCategory.value) return passwordsMatchingSearch.value
    return passwordsMatchingSearch.value.filter(p => 
      p.category === selectedCategory.value
    )
  })

  const filteredPasswords = computed(() => {
    if (!selectedSubcategory.value) return passwordsMatchingCategory.value
    return passwordsMatchingCategory.value.filter(p => 
      p.subcategory === selectedSubcategory.value
    )
  })

  const categories = computed(() => {
    const unique = new Set(passwords.value.map(p => p.category))
    return Array.from(unique).sort()
  })

  const subcategories = computed(() => {
    if (!selectedCategory.value) return []
    const unique = new Set(
      passwords.value
        .filter(p => p.category === selectedCategory.value && p.subcategory)
        .map(p => p.subcategory)
        .filter(Boolean) as string[]
    )
    return Array.from(unique).sort()
  })

  // Actions
  async function fetchPasswords() {
    loading.value = true
    error.value = null
    try {
      passwords.value = await passwordApi.listPasswords(filter.value)
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to fetch passwords'
    } finally {
      loading.value = false
    }
  }

  async function createPassword(input: PasswordInput) {
    loading.value = true
    error.value = null
    try {
      const newPassword = await passwordApi.createPassword(input)
      passwords.value.unshift(newPassword)
      return newPassword
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to create password'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function updatePassword(id: number, input: PasswordInput) {
    loading.value = true
    error.value = null
    try {
      const updatedPassword = await passwordApi.updatePassword(id, input)
      const index = passwords.value.findIndex(p => p.id === id)
      if (index !== -1) {
        passwords.value[index] = updatedPassword
      }
      return updatedPassword
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to update password'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function deletePassword(id: number) {
    loading.value = true
    error.value = null
    try {
      await passwordApi.deletePassword(id)
      passwords.value = passwords.value.filter(p => p.id !== id)
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to delete password'
      throw e
    } finally {
      loading.value = false
    }
  }

  function selectPassword(password: Password | undefined) {
    selectedPassword.value = password
    showForm.value = true
  }

  function openCreateForm() {
    selectedPassword.value = undefined
    showForm.value = true
  }

  function closeForm() {
    selectedPassword.value = undefined
    showForm.value = false
  }

  function setSearchQuery(query: string) {
    searchQuery.value = query
  }

  function setCategory(category: string) {
    selectedCategory.value = category
    selectedSubcategory.value = ''
  }

  function setSubcategory(subcategory: string) {
    selectedSubcategory.value = subcategory
  }

  function resetFilters() {
    searchQuery.value = ''
    selectedCategory.value = ''
    selectedSubcategory.value = ''
  }

  return {
    // State
    passwords,
    loading,
    error,
    filter,
    selectedPassword,
    showForm,
    searchQuery,
    selectedCategory,
    selectedSubcategory,
    // Computed
    filteredPasswords,
    categories,
    subcategories,
    // Actions
    fetchPasswords,
    createPassword,
    updatePassword,
    deletePassword,
    selectPassword,
    openCreateForm,
    closeForm,
    setSearchQuery,
    setCategory,
    setSubcategory,
    resetFilters,
  }
})
