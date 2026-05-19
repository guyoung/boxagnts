import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { Settings } from '@/api'

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<Settings>({
    system_prompt: '',
    default_model: '',
    default_provider: '',
    allowed_outbound_hosts: [],
  })

  return {
    settings,
  }
})
