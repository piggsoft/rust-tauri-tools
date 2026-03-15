import { invoke } from '@tauri-apps/api/core'
import type { ApiResponse } from '../types'

export interface Password {
  id: number
  category: string
  subcategory: string | null
  account: string
  password: string
  login_url: string | null
  notes: string | null
  created_at: string
  updated_at: string
}

export interface PasswordInput {
  category: string
  subcategory: string | null
  account: string
  password: string
  login_url: string | null
  notes: string | null
}

export interface PasswordFilter {
  search?: string
  category?: string
  subcategory?: string
}

export const passwordApi = {
  async createPassword(input: PasswordInput): Promise<Password> {
    const response: ApiResponse<Password> = await invoke('create_password', { password: input })
    if (!response.success || !response.data) {
      throw new Error(response.error || 'Failed to create password')
    }
    return response.data
  },

  async getPassword(id: number): Promise<Password> {
    const response: ApiResponse<Password> = await invoke('get_password', { id })
    if (!response.success || !response.data) {
      throw new Error(response.error || 'Failed to get password')
    }
    return response.data
  },

  async listPasswords(filter?: PasswordFilter): Promise<Password[]> {
    const response: ApiResponse<Password[]> = await invoke('list_passwords', { filter })
    if (!response.success || !response.data) {
      throw new Error(response.error || 'Failed to list passwords')
    }
    return response.data
  },

  async updatePassword(id: number, input: PasswordInput): Promise<Password> {
    const response: ApiResponse<Password> = await invoke('update_password', { id, password: input })
    if (!response.success || !response.data) {
      throw new Error(response.error || 'Failed to update password')
    }
    return response.data
  },

  async deletePassword(id: number): Promise<void> {
    const response: ApiResponse<void> = await invoke('delete_password', { id })
    if (!response.success) {
      throw new Error(response.error || 'Failed to delete password')
    }
  },

  async getCategories(): Promise<string[]> {
    const response: ApiResponse<string[]> = await invoke('get_password_categories')
    if (!response.success || !response.data) {
      throw new Error(response.error || 'Failed to get categories')
    }
    return response.data
  },

  async getSubcategories(category?: string): Promise<string[]> {
    const response: ApiResponse<string[]> = await invoke('get_password_subcategories', { category })
    if (!response.success || !response.data) {
      throw new Error(response.error || 'Failed to get subcategories')
    }
    return response.data
  },
}
