<template>
  <div>
    <div class="d-flex align-center justify-space-between mb-6">
      <div class="d-flex align-center">
        <v-icon size="32" color="primary" class="mr-3">mdi-hammer-wrench</v-icon>
        <h1 class="text-h4 font-weight-bold">Tools</h1>
      </div>
      <v-btn color="primary" prepend-icon="mdi-plus" @click="openAddDialog">
        Add Tool
      </v-btn>
    </div>

    <v-row>
      <v-col cols="12" md="6" lg="4" v-for="tool in toolStore.tools" :key="tool.id">
        <v-card class="fill-height">
          <v-card-item>
            <template #prepend>
              <v-icon :color="tool.enabled ? 'primary' : 'medium-emphasis'" size="28">
                mdi-hammer-wrench
              </v-icon>
            </template>
            <v-card-title>{{ tool.name }}</v-card-title>
            <v-card-subtitle>{{ tool.type }}</v-card-subtitle>
          </v-card-item>

          <v-card-text>
            <div v-if="tool.description" class="text-body-2 mb-3 description-preview">{{ tool.description }}</div>

            <div v-if="tool.config" class="mb-3">
              <div class="text-caption text-medium-emphasis mb-1">Config</div>
              <div class="config-preview text-caption">{{ tool.config }}</div>
            </div>
          </v-card-text>

          <v-card-actions>
            <v-switch
              :model-value="tool.enabled"
              :label="tool.enabled ? 'Enabled' : 'Disabled'"
              color="success"
              density="compact"
              hide-details
              @update:model-value="(v: boolean | null) => handleToggleEnabled(tool, !!v)"
            />
            <v-spacer />
            <v-btn variant="tonal" size="small" prepend-icon="mdi-pencil" @click="openEditDialog(tool)">
              Edit
            </v-btn>
            <v-spacer />
            <v-btn
              icon="mdi-delete"
              variant="text"
              size="small"
              color="error"
              @click="confirmRemove(tool)"
            />
          </v-card-actions>
        </v-card>
      </v-col>
    </v-row>

    <v-skeleton-loader v-if="toolStore.loading" type="card@3" />

    <div v-if="!toolStore.loading && toolStore.tools.length === 0" class="text-center py-12">
      <v-icon size="64" color="medium-emphasis">mdi-hammer-wrench</v-icon>
      <p class="text-medium-emphasis mt-4">No tools configured</p>
      <v-btn color="primary" class="mt-4" @click="openAddDialog">Add Tool</v-btn>
    </div>

    <!-- Add / Edit Dialog -->
    <v-dialog v-model="showDialog" max-width="600">
      <v-card>
        <v-card-title>{{ editingTool ? 'Edit Tool' : 'Add Tool' }}</v-card-title>
        <v-card-text>
          <v-row dense>
            <v-col cols="12" md="6">
              <v-text-field
                v-model="form.name"
                label="Name"
                variant="outlined"
                placeholder="file-reader"
                hint="Unique name for the tool"
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
                placeholder="Tool description..."
              />
            </v-col>
            <v-col cols="12">
              <v-textarea
                v-model="form.config"
                label="Config"
                variant="outlined"
                rows="6"
                placeholder='{"type": "function", "function": {...}}'
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
            {{ editingTool ? 'Update' : 'Create' }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Delete Confirmation Dialog -->
    <v-dialog v-model="deleteDialog" max-width="400">
      <v-card>
        <v-card-title>Delete Tool</v-card-title>
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
import { useToolStore } from '@/stores/tools'
import { useAppStore } from '@/stores/app'
import type { Tool } from '@/api'

const toolStore = useToolStore()
const appStore = useAppStore()

const showDialog = ref(false)
const editingTool = ref<Tool | null>(null)
const saving = ref(false)
const deleteDialog = ref(false)
const deleteTarget = ref<Tool | null>(null)
const deleting = ref(false)

const availableTypes = [
  'function',
  'browser',
  'search',
  'file',
  'custom',
]

const defaultForm = () => ({
  name: '',
  description: '',
  type: 'function',
  config: '',
  enabled: true,
})

const form = ref(defaultForm())

function openAddDialog() {
  editingTool.value = null
  form.value = defaultForm()
  showDialog.value = true
}

function openEditDialog(tool: Tool) {
  editingTool.value = tool
  form.value = {
    name: tool.name,
    description: tool.description,
    type: tool.type,
    config: tool.config,
    enabled: tool.enabled,
  }
  showDialog.value = true
}

async function handleSave() {
  saving.value = true
  try {
    if (editingTool.value) {
      await toolStore.updateTool(editingTool.value.id, { ...form.value })
      appStore.showMessage('Tool updated', 'success')
    } else {
      await toolStore.addTool({ ...form.value })
      appStore.showMessage('Tool created', 'success')
    }
    showDialog.value = false
    form.value = defaultForm()
    editingTool.value = null
  } catch {
    appStore.showMessage('Failed to save tool', 'error')
  } finally {
    saving.value = false
  }
}

function confirmRemove(tool: Tool) {
  deleteTarget.value = tool
  deleteDialog.value = true
}

async function handleDelete() {
  if (!deleteTarget.value) return
  deleting.value = true
  try {
    await toolStore.removeTool(deleteTarget.value.id)
    appStore.showMessage('Tool deleted', 'success')
  } catch {
    appStore.showMessage('Failed to delete tool', 'error')
  } finally {
    deleting.value = false
    deleteDialog.value = false
    deleteTarget.value = null
  }
}

async function handleToggleEnabled(tool: Tool, enabled: boolean) {
  try {
    await toolStore.updateTool(tool.id, { enabled })
    appStore.showMessage(enabled ? 'Tool enabled' : 'Tool disabled', 'success')
  } catch {
    appStore.showMessage('Failed to update tool', 'error')
  }
}

onMounted(() => {
  toolStore.fetchTools()
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
