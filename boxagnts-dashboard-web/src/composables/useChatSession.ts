import { watch, onMounted, type Ref } from 'vue'
import { api, type Project } from '@/api'
import { useAppStore } from '@/stores/app'
import { useSessionStore } from '@/stores/sessions'
import { loadSessionHistory, type DisplayItem, type ChatUiState } from './useChatMessages'

export interface UseChatSessionOptions {
  sessionId: Ref<string | null>
  currentProject: Ref<Project | null>
  availableModels: Ref<string[]>
  messages: Ref<DisplayItem[]>
  uiState: ChatUiState
  scrollToBottom: () => void
  cleanupActiveStream: () => void
}

export function useChatSession(options: UseChatSessionOptions) {
  const { sessionId, currentProject, availableModels, messages, uiState, scrollToBottom, cleanupActiveStream } = options

  const appStore = useAppStore()
  const sessionStore = useSessionStore()

  async function loadAndSetHistory(sid: string) {
    messages.value = []
    try {
      const items = await loadSessionHistory(sid)
      messages.value = items
      scrollToBottom()
    } catch {
      appStore.showMessage('Failed to load session history', 'error')
    }
  }

  watch(() => sessionStore.currentSessionId, (newId) => {
    if (newId === sessionId.value) return
    cleanupActiveStream()
    uiState.isRunning = false
    messages.value = []
    if (newId) {
      sessionId.value = newId
      loadAndSetHistory(newId)
    } else {
      sessionId.value = null
    }
  }, { immediate: true })

  onMounted(async () => {
    try {
      currentProject.value = await api.getCurrentProject()
      appStore.setCurrentProject(currentProject.value)
    } catch {
      appStore.showMessage('Could not detect current project', 'warning')
    }
    try {
      const models = await api.getModels()
      availableModels.value = models
      const savedModel = localStorage.getItem('boxagnts_selected_model')
      if (savedModel && models.includes(savedModel)) {
        uiState.selectedModel = savedModel
      } else if (models.length > 0) {
        uiState.selectedModel = models[0]
      }
    } catch {
      availableModels.value = []
    }
    sessionStore.fetchSessions()
  })

  watch(() => uiState.selectedModel, (val) => {
    if (val) {
      localStorage.setItem('boxagnts_selected_model', val)
    }
  })

  function cancelExecution() {
    cleanupActiveStream()
    if (sessionId.value) {
      api.chatExecuteCancel(sessionId.value).catch(() => {})
    }
  }

  return { cancelExecution }
}
