import { defineStore } from 'pinia'
import { ref } from 'vue'
import { api } from '@/api'
import type { UsageStats, UsageEntry, ProjectUsage } from '@/types'

export const useUsageStore = defineStore('usage', () => {
  const summary = ref<UsageStats | null>(null)
  const dailyStats = ref<UsageStats | null>(null)
  const sessionStats = ref<ProjectUsage[]>([])
  const usageDetails = ref<UsageEntry[]>([])
  const loading = ref(false)

  async function fetchUsage() {
    loading.value = true
    try {
      summary.value = await api.getUsageStats()
    } catch (e) {
      console.error('Failed to fetch usage:', e)
    } finally {
      loading.value = false
    }
  }

  async function fetchByDateRange(start: string, end: string) {
    try {
      dailyStats.value = await api.getUsageByDateRange(start, end)
    } catch (e) {
      console.error('Failed to fetch usage by date range:', e)
    }
  }

  async function fetchSessionStats() {
    try {
      sessionStats.value = await api.getSessionStats()
    } catch (e) {
      console.error('Failed to fetch session stats:', e)
    }
  }

  async function fetchUsageDetails(limit?: number) {
    try {
      usageDetails.value = await api.getUsageDetails(limit)
    } catch (e) {
      console.error('Failed to fetch usage details:', e)
    }
  }

  return {
    summary,
    dailyStats,
    sessionStats,
    usageDetails,
    loading,
    fetchUsage,
    fetchByDateRange,
    fetchSessionStats,
    fetchUsageDetails,
  }
})
