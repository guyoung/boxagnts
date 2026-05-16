import { defineStore } from 'pinia'
import { ref } from 'vue'
import { api, type Tool } from '@/api'

export const useToolStore = defineStore('tools', () => {
  const tools = ref<Tool[]>([])
  const loading = ref(false)

  async function fetchTools() {
    loading.value = true
    try {
      tools.value = await api.getTools()
    } catch (e) {
      console.error('Failed to fetch tools:', e)
      tools.value = []
    } finally {
      loading.value = false
    }
  }

  async function addTool(data: Omit<Tool, 'id'>): Promise<Tool> {
    const tool = await api.createTool(data)
    tools.value.push(tool)
    return tool
  }

  async function updateTool(id: string, data: Partial<Omit<Tool, 'id'>>): Promise<Tool> {
    const tool = await api.updateTool(id, data)
    const idx = tools.value.findIndex(t => t.id === id)
    if (idx >= 0) {
      tools.value[idx] = tool
    }
    return tool
  }

  async function removeTool(id: string) {
    await api.deleteTool(id)
    tools.value = tools.value.filter(t => t.id !== id)
  }

  return {
    tools,
    loading,
    fetchTools,
    addTool,
    updateTool,
    removeTool,
  }
})
