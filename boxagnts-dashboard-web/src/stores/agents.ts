import { defineStore } from 'pinia'
import { api, type Agent } from '@/api'
import { useCrudOperations } from './baseCrud'

export const useAgentStore = defineStore('agents', () => {
  const crud = useCrudOperations<Agent, Omit<Agent, 'id'>, Partial<Omit<Agent, 'id'>>>(
    {
      fetchAll: () => api.getAgents(),
      create: (data) => api.createAgent(data),
      update: (id, data) => api.updateAgent(id, data),
      remove: (id) => api.deleteAgent(id),
    },
    'agents'
  )

  return {
    agents: crud.items,
    loading: crud.loading,
    fetchAgents: crud.fetch,
    addAgent: crud.add,
    updateAgent: crud.update,
    removeAgent: crud.remove,
  }
})
