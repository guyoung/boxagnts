import { defineStore } from 'pinia'
import { ref } from 'vue'
import { api, type Agent } from '@/api'

export const useAgentStore = defineStore('agents', () => {
  const agents = ref<Agent[]>([])
  const loading = ref(false)

  async function fetchAgents() {
    loading.value = true
    try {
      agents.value = await api.getAgents()
    } catch (e) {
      console.error('Failed to fetch agents:', e)
      agents.value = []
    } finally {
      loading.value = false
    }
  }

  async function addAgent(data: Omit<Agent, 'id'>): Promise<Agent> {
    const agent = await api.createAgent(data)
    agents.value.push(agent)
    return agent
  }

  async function updateAgent(id: string, data: Partial<Omit<Agent, 'id'>>): Promise<Agent> {
    const agent = await api.updateAgent(id, data)
    const idx = agents.value.findIndex(a => a.id === id)
    if (idx >= 0) {
      agents.value[idx] = agent
    }
    return agent
  }

  async function removeAgent(id: string) {
    await api.deleteAgent(id)
    agents.value = agents.value.filter(a => a.id !== id)
  }

  return {
    agents,
    loading,
    fetchAgents,
    addAgent,
    updateAgent,
    removeAgent,
  }
})
