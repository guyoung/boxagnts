import { defineStore } from 'pinia'
import { ref } from 'vue'
import { api, type CronJob, type CronLog } from '@/api'

export const useCronStore = defineStore('crons', () => {
  const crons = ref<CronJob[]>([])
  const loading = ref(false)

  async function fetchCrons() {
    loading.value = true
    try {
      crons.value = await api.getCrons()
    } catch (e) {
      console.error('Failed to fetch crons:', e)
      crons.value = []
    } finally {
      loading.value = false
    }
  }

  async function addCron(data: Omit<CronJob, 'id' | 'last_run_at' | 'last_run_success'>): Promise<CronJob> {
    const cron = await api.createCron(data)
    crons.value.push(cron)
    return cron
  }

  async function updateCron(id: string, data: Partial<Omit<CronJob, 'id'>>): Promise<CronJob> {
    const cron = await api.updateCron(id, data)
    const idx = crons.value.findIndex(c => c.id === id)
    if (idx >= 0) {
      crons.value[idx] = cron
    }
    return cron
  }

  async function removeCron(id: string) {
    await api.deleteCron(id)
    crons.value = crons.value.filter(c => c.id !== id)
  }

  async function fetchCronLogs(jobId: string): Promise<CronLog[]> {
    return api.getCronLogs(jobId)
  }

  return {
    crons,
    loading,
    fetchCrons,
    addCron,
    updateCron,
    removeCron,
    fetchCronLogs,
  }
})
