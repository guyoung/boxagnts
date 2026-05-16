import { defineStore } from 'pinia'
import { ref } from 'vue'
import { api, type Skill } from '@/api'

export const useSkillStore = defineStore('skills', () => {
  const skills = ref<Skill[]>([])
  const loading = ref(false)

  async function fetchSkills() {
    loading.value = true
    try {
      skills.value = await api.getSkills()
    } catch (e) {
      console.error('Failed to fetch skills:', e)
      skills.value = []
    } finally {
      loading.value = false
    }
  }

  async function addSkill(data: Omit<Skill, 'id'>): Promise<Skill> {
    const skill = await api.createSkill(data)
    skills.value.push(skill)
    return skill
  }

  async function updateSkill(id: string, data: Partial<Omit<Skill, 'id'>>): Promise<Skill> {
    const skill = await api.updateSkill(id, data)
    const idx = skills.value.findIndex(s => s.id === id)
    if (idx >= 0) {
      skills.value[idx] = skill
    }
    return skill
  }

  async function removeSkill(id: string) {
    await api.deleteSkill(id)
    skills.value = skills.value.filter(s => s.id !== id)
  }

  return {
    skills,
    loading,
    fetchSkills,
    addSkill,
    updateSkill,
    removeSkill,
  }
})
