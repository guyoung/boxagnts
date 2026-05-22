import { defineStore } from 'pinia'
import { api, type Site } from '@/api'
import { useCrudOperations } from './baseCrud'

export const useSiteStore = defineStore('sites', () => {
  const crud = useCrudOperations<Site, Omit<Site, 'id'>, Partial<Omit<Site, 'id'>>>(
    {
      fetchAll: () => api.getSites(),
      create: (data) => api.createSite(data),
      update: (id, data) => api.updateSite(id, data),
      remove: (id) => api.deleteSite(id),
    },
    'sites'
  )

  return {
    sites: crud.items,
    loading: crud.loading,
    fetchSites: crud.fetch,
    addSite: crud.add,
    updateSite: crud.update,
    removeSite: crud.remove,
  }
})
