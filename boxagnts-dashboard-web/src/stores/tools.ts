import { defineStore } from 'pinia'
import { api, type Tool } from '@/api'
import { useCrudOperations } from './baseCrud'

export const useToolStore = defineStore('tools', () => {
  const crud = useCrudOperations<Tool, Omit<Tool, 'id'>, Partial<Omit<Tool, 'id'>>>(
    {
      fetchAll: () => api.getTools(),
      create: (data) => api.createTool(data),
      update: (id, data) => api.updateTool(id, data),
      remove: (id) => api.deleteTool(id),
    },
    'tools'
  )

  return {
    tools: crud.items,
    loading: crud.loading,
    fetchTools: crud.fetch,
    addTool: crud.add,
    updateTool: crud.update,
    removeTool: crud.remove,
  }
})
