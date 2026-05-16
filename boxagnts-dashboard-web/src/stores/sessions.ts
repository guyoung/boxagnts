import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { api, type Session } from '@/api'

export const useSessionStore = defineStore('session', () => {
  const sessions = ref<Session[]>([])
  const currentSessionId = ref<string | null>(null)
  const loading = ref(false)

  const sessionCount = computed(() => sessions.value.length)

  function sessionLabel(s: Session): string {
    if (s.title) {
      return s.title
    }
    if (s.first_message) {
      const t = s.first_message.trim()
      return t.length > 40 ? t.slice(0, 40) + '...' : t
    }
    return 'New Session'
  }

  function sessionDate(s: Session): string {
    const d = new Date(s.created_at * 1000)
    const now = new Date()
    const diff = now.getTime() - d.getTime()
    const days = Math.floor(diff / 86400000)
    if (days === 0) return d.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' })
    if (days < 7) return `${days}d ago`
    return d.toLocaleDateString('zh-CN', { month: 'short', day: 'numeric' })
  }

  async function fetchSessions() {
    loading.value = true
    try {
      sessions.value = await api.getSessions()
    } catch (e) {
      console.error('Failed to fetch sessions:', e)
      sessions.value = []
    } finally {
      loading.value = false
    }
  }

  function selectSession(id: string | null) {
    currentSessionId.value = id
  }

  function addSession(session: Session) {
    const idx = sessions.value.findIndex(s => s.id === session.id)
    if (idx >= 0) {
      sessions.value[idx] = session
    } else {
      sessions.value.unshift(session)
    }
  }

  async function deleteSession(sessionId: string) {
    try {
      await api.deleteSession(sessionId)
      sessions.value = sessions.value.filter(s => s.id !== sessionId)
      if (currentSessionId.value === sessionId) {
        currentSessionId.value = null
      }
    } catch (e) {
      console.error('Failed to delete session:', e)
      throw e
    }
  }

  return {
    sessions,
    currentSessionId,
    loading,
    sessionCount,
    sessionLabel,
    sessionDate,
    fetchSessions,
    selectSession,
    addSession,
    deleteSession,
  }
})
