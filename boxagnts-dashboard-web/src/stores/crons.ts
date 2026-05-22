import { defineStore } from 'pinia'
import { api, type CronJob, type CronLog } from '@/api'
import { useCrudOperations } from './baseCrud'

export const useCronStore = defineStore('crons', () => {
  const crud = useCrudOperations<CronJob, Omit<CronJob, 'id' | 'last_run_at' | 'last_run_success'>, Partial<Omit<CronJob, 'id'>>>(
    {
      fetchAll: () => api.getCrons(),
      create: (data) => api.createCron(data),
      update: (id, data) => api.updateCron(id, data),
      remove: (id) => api.deleteCron(id),
    },
    'crons'
  )

  async function fetchCronLogs(jobId: string): Promise<CronLog[]> {
    return api.getCronLogs(jobId)
  }

  return {
    crons: crud.items,
    loading: crud.loading,
    fetchCrons: crud.fetch,
    addCron: crud.add,
    updateCron: crud.update,
    removeCron: crud.remove,
    fetchCronLogs,
  }
})
