import { defineStore } from 'pinia'
import { api, type Skill } from '@/api'
import { useCrudOperations } from './baseCrud'

export const useSkillStore = defineStore('skills', () => {
  const crud = useCrudOperations<Skill, Omit<Skill, 'id'>, Partial<Omit<Skill, 'id'>>>(
    {
      fetchAll: () => api.getSkills(),
      create: (data) => api.createSkill(data),
      update: (id, data) => api.updateSkill(id, data),
      remove: (id) => api.deleteSkill(id),
    },
    'skills'
  )

  return {
    skills: crud.items,
    loading: crud.loading,
    fetchSkills: crud.fetch,
    addSkill: crud.add,
    updateSkill: crud.update,
    removeSkill: crud.remove,
  }
})
