import { defineStore } from 'pinia'
import { ref } from 'vue'
import { api, type Site } from '@/api'

export const useSiteStore = defineStore('sites', () => {
  const sites = ref<Site[]>([])
  const loading = ref(false)

  async function fetchSites() {
    loading.value = true
    try {
      sites.value = await api.getSites()
    } catch (e) {
      console.error('Failed to fetch sites:', e)
      sites.value = []
    } finally {
      loading.value = false
    }
  }

  async function addSite(data: Omit<Site, 'id'>): Promise<Site> {
    const site = await api.createSite(data)
    sites.value.push(site)
    return site
  }

  async function updateSite(id: string, data: Partial<Omit<Site, 'id'>>): Promise<Site> {
    const site = await api.updateSite(id, data)
    const idx = sites.value.findIndex(s => s.id === id)
    if (idx >= 0) {
      sites.value[idx] = site
    }
    return site
  }

  async function removeSite(id: string) {
    await api.deleteSite(id)
    sites.value = sites.value.filter(s => s.id !== id)
  }

  return {
    sites,
    loading,
    fetchSites,
    addSite,
    updateSite,
    removeSite,
  }
})
