<template>
  <div>
    <div class="d-flex align-center justify-space-between mb-6">
      <div class="d-flex align-center">
        <v-icon size="32" color="primary" class="mr-3">mdi-star</v-icon>
        <h1 class="text-h4 font-weight-bold">Skills</h1>
      </div>
      <v-btn color="primary" prepend-icon="mdi-plus" @click="openAddDialog">
        Add Skill
      </v-btn>
    </div>

    <v-row>
      <v-col cols="12" md="6" lg="4" v-for="skill in skillStore.skills" :key="skill.id">
        <v-card class="fill-height">
          <v-card-item>
            <template #prepend>
              <v-icon :color="skill.enabled ? 'primary' : 'medium-emphasis'" size="28">
                mdi-star
              </v-icon>
            </template>
            <v-card-title>{{ skill.name }}</v-card-title>
            <v-card-subtitle>{{ skill.type }}</v-card-subtitle>
          </v-card-item>

          <v-card-text>
            <div v-if="skill.description" class="text-body-2 mb-3 description-preview">{{ skill.description }}</div>

            <div v-if="skill.config" class="mb-3">
              <div class="text-caption text-medium-emphasis mb-1">Config</div>
              <div class="config-preview text-caption">{{ skill.config }}</div>
            </div>
          </v-card-text>

          <v-card-actions>
            <v-switch
              :model-value="skill.enabled"
              :label="skill.enabled ? 'Enabled' : 'Disabled'"
              color="success"
              density="compact"
              hide-details
              @update:model-value="(v: boolean | null) => handleToggleEnabled(skill, !!v)"
            />
            <v-spacer />
            <v-btn variant="tonal" size="small" prepend-icon="mdi-pencil" @click="openEditDialog(skill)">
              Edit
            </v-btn>
            <v-spacer />
            <v-btn
              icon="mdi-delete"
              variant="text"
              size="small"
              color="error"
              @click="confirmRemove(skill)"
            />
          </v-card-actions>
        </v-card>
      </v-col>
    </v-row>

    <v-skeleton-loader v-if="skillStore.loading" type="card@3" />

    <div v-if="!skillStore.loading && skillStore.skills.length === 0" class="text-center py-12">
      <v-icon size="64" color="medium-emphasis">mdi-star-off</v-icon>
      <p class="text-medium-emphasis mt-4">No skills configured</p>
      <v-btn color="primary" class="mt-4" @click="openAddDialog">Add Skill</v-btn>
    </div>

    <!-- Add / Edit Dialog -->
    <v-dialog v-model="showDialog" max-width="600">
      <v-card>
        <v-card-title>{{ editingSkill ? 'Edit Skill' : 'Add Skill' }}</v-card-title>
        <v-card-text>
          <v-row dense>
            <v-col cols="12" md="6">
              <v-text-field
                v-model="form.name"
                label="Name"
                variant="outlined"
                placeholder="code-review"
                hint="Unique name for the skill"
                persistent-hint
              />
            </v-col>
            <v-col cols="12" md="6">
              <v-select
                v-model="form.type"
                label="Type"
                :items="availableTypes"
                variant="outlined"
              />
            </v-col>
            <v-col cols="12">
              <v-textarea
                v-model="form.description"
                label="Description"
                variant="outlined"
                rows="2"
                placeholder="Skill description..."
              />
            </v-col>
            <v-col cols="12">
              <v-textarea
                v-model="form.config"
                label="Config"
                variant="outlined"
                rows="6"
                placeholder='{"prompt": "...", "max_tokens": 1000}'
              />
            </v-col>
            <v-col cols="12">
              <v-switch
                v-model="form.enabled"
                label="Enabled"
                color="success"
                hide-details
              />
            </v-col>
          </v-row>
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="showDialog = false">Cancel</v-btn>
          <v-btn
            color="primary"
            @click="handleSave"
            :loading="saving"
            :disabled="!form.name.trim()"
          >
            {{ editingSkill ? 'Update' : 'Create' }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Delete Confirmation Dialog -->
    <v-dialog v-model="deleteDialog" max-width="400">
      <v-card>
        <v-card-title>Delete Skill</v-card-title>
        <v-card-text>
          <p>
            Are you sure you want to delete
            <strong>{{ deleteTarget?.name }}</strong>?
          </p>
          <p class="text-caption text-error mt-2">This action cannot be undone.</p>
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="deleteDialog = false">Cancel</v-btn>
          <v-btn color="error" @click="handleDelete" :loading="deleting">Delete</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useSkillStore } from '@/stores/skills'
import { useAppStore } from '@/stores/app'
import type { Skill } from '@/api'

const skillStore = useSkillStore()
const appStore = useAppStore()

const showDialog = ref(false)
const editingSkill = ref<Skill | null>(null)
const saving = ref(false)
const deleteDialog = ref(false)
const deleteTarget = ref<Skill | null>(null)
const deleting = ref(false)

const availableTypes = [
  'prompt',
  'tool',
  'workflow',
  'template',
  'custom',
]

const defaultForm = () => ({
  name: '',
  description: '',
  type: 'prompt',
  config: '',
  enabled: true,
})

const form = ref(defaultForm())

function openAddDialog() {
  editingSkill.value = null
  form.value = defaultForm()
  showDialog.value = true
}

function openEditDialog(skill: Skill) {
  editingSkill.value = skill
  form.value = {
    name: skill.name,
    description: skill.description,
    type: skill.type,
    config: skill.config,
    enabled: skill.enabled,
  }
  showDialog.value = true
}

async function handleSave() {
  saving.value = true
  try {
    if (editingSkill.value) {
      await skillStore.updateSkill(editingSkill.value.id, { ...form.value })
      appStore.showMessage('Skill updated', 'success')
    } else {
      await skillStore.addSkill({ ...form.value })
      appStore.showMessage('Skill created', 'success')
    }
    showDialog.value = false
    form.value = defaultForm()
    editingSkill.value = null
  } catch {
    appStore.showMessage('Failed to save skill', 'error')
  } finally {
    saving.value = false
  }
}

function confirmRemove(skill: Skill) {
  deleteTarget.value = skill
  deleteDialog.value = true
}

async function handleDelete() {
  if (!deleteTarget.value) return
  deleting.value = true
  try {
    await skillStore.removeSkill(deleteTarget.value.id)
    appStore.showMessage('Skill deleted', 'success')
  } catch {
    appStore.showMessage('Failed to delete skill', 'error')
  } finally {
    deleting.value = false
    deleteDialog.value = false
    deleteTarget.value = null
  }
}

async function handleToggleEnabled(skill: Skill, enabled: boolean) {
  try {
    await skillStore.updateSkill(skill.id, { enabled })
    appStore.showMessage(enabled ? 'Skill enabled' : 'Skill disabled', 'success')
  } catch {
    appStore.showMessage('Failed to update skill', 'error')
  }
}

onMounted(() => {
  skillStore.fetchSkills()
})
</script>

<style scoped>
.description-preview {
  max-height: 48px;
  overflow: hidden;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  color: rgba(var(--v-theme-on-surface), 0.7);
  line-height: 1.5;
}

.config-preview {
  max-height: 64px;
  overflow: hidden;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  color: rgba(var(--v-theme-on-surface), 0.6);
  line-height: 1.5;
}
</style>
