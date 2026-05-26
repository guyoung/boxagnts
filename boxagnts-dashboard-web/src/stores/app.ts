import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Project } from '@/api'

export const useAppStore = defineStore('app', () => {
  const sidebarCollapsed = ref(false)
  const rightSidebarCollapsed = ref(false)
  const sidebarWidth = ref(280)
  const theme = ref<'dark' | 'light'>('dark')
  const searchQuery = ref('')
  const currentProject = ref<Project | null>(null)
  const snackbar = ref({
    show: false,
    message: '',
    color: 'info' as 'info' | 'success' | 'error' | 'warning',
    timeout: 3000,
  })

  const isDark = computed(() => theme.value === 'dark')

  function toggleSidebar() {
    sidebarCollapsed.value = !sidebarCollapsed.value
  }

  function toggleRightSidebar() {
    rightSidebarCollapsed.value = !rightSidebarCollapsed.value
  }

  function setSidebarWidth(w: number) {
    sidebarWidth.value = Math.min(500, Math.max(200, w))
  }

  function toggleTheme() {
    theme.value = theme.value === 'dark' ? 'light' : 'dark'
  }

  function showMessage(message: string, color: 'info' | 'success' | 'error' | 'warning' = 'info') {
    snackbar.value = { show: true, message, color, timeout: 3000 }
  }

  function hideSnackbar() {
    snackbar.value.show = false
  }

  function setCurrentProject(project: Project | null) {
    currentProject.value = project
  }

  return {
    sidebarCollapsed,
    rightSidebarCollapsed,
    sidebarWidth,
    theme,
    searchQuery,
    currentProject,
    snackbar,
    isDark,
    toggleSidebar,
    toggleRightSidebar,
    setSidebarWidth,
    toggleTheme,
    showMessage,
    hideSnackbar,
    setCurrentProject,
  }
})
