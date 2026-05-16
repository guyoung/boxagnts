<template>
  <div>
    <div class="d-flex align-center justify-space-between mb-6">
      <div class="d-flex align-center">
        <v-icon size="32" color="primary" class="mr-3">mdi-robot</v-icon>
        <h1 class="text-h4 font-weight-bold">Agents</h1>
      </div>
      <v-btn color="primary" prepend-icon="mdi-plus" @click="openAddDialog">
        Add Agent
      </v-btn>
    </div>

    <v-row>
      <v-col cols="12" md="6" lg="4" v-for="agent in agentStore.agents" :key="agent.id">
        <v-card class="fill-height">
          <v-card-item>
            <template #prepend>
              <v-icon :color="agent.enabled ? 'primary' : 'medium-emphasis'" size="28">
                mdi-robot
              </v-icon>
            </template>
            <v-card-title>{{ agent.name }}</v-card-title>
            <v-card-subtitle>{{ agent.model }}</v-card-subtitle>
          </v-card-item>

          <v-card-text>
            <div v-if="agent.desc" class="text-body-2 mb-3">{{ agent.desc }}</div>

            <div v-if="agent.system_prompt" class="mb-3">
              <div class="text-caption text-medium-emphasis mb-1">System Prompt</div>
              <div class="system-prompt-preview text-caption">{{ agent.system_prompt }}</div>
            </div>

            <div v-if="agent.tools" class="mb-2">
              <div class="text-caption text-medium-emphasis mb-1">Tools</div>
              <div class="d-flex flex-wrap gap-1">
                <v-chip v-for="tool in agent.tools.split(',').map(t => t.trim()).filter(Boolean)" :key="tool" size="x-small" variant="tonal">
                  {{ tool }}
                </v-chip>
              </div>
            </div>

            <div class="d-flex align-center gap-2 mt-2">
              <v-chip :color="agent.enabled ? 'success' : 'medium-emphasis'" size="x-small" variant="tonal">
                {{ agent.enabled ? 'Active' : 'Inactive' }}
              </v-chip>
            </div>
          </v-card-text>

          <v-card-actions>
            <v-switch
              :model-value="agent.enabled"
              :label="agent.enabled ? 'Enabled' : 'Disabled'"
              color="success"
              density="compact"
              hide-details
              @update:model-value="(v: boolean | null) => handleToggleEnabled(agent, !!v)"
            />
            <v-spacer />
            <v-btn variant="tonal" size="small" prepend-icon="mdi-pencil" @click="openEditDialog(agent)">
              Edit
            </v-btn>
            <v-spacer />
            <v-btn
              icon="mdi-delete"
              variant="text"
              size="small"
              color="error"
              @click="confirmRemove(agent)"
            />
          </v-card-actions>
        </v-card>
      </v-col>
    </v-row>

    <v-skeleton-loader v-if="agentStore.loading" type="card@3" />

    <div v-if="!agentStore.loading && agentStore.agents.length === 0" class="text-center py-12">
      <v-icon size="64" color="medium-emphasis">mdi-robot-off</v-icon>
      <p class="text-medium-emphasis mt-4">No agents configured</p>
      <v-btn color="primary" class="mt-4" @click="openAddDialog">Add Agent</v-btn>
    </div>

    <!-- Add / Edit Dialog -->
    <v-dialog v-model="showDialog" max-width="600">
      <v-card>
        <v-card-title>{{ editingAgent ? 'Edit Agent' : 'Add Agent' }}</v-card-title>
        <v-card-text>
          <v-row dense>
            <v-col cols="12" md="6">
              <v-text-field
                v-model="form.name"
                label="Name"
                variant="outlined"
                placeholder="code-reviewer"
                hint="Unique name for the agent"
                persistent-hint
              />
            </v-col>
            <v-col cols="12" md="6">
              <v-select
                v-model="form.model"
                label="Model"
                :items="availableModels"
                variant="outlined"
              />
            </v-col>
            <v-col cols="12">
              <v-textarea
                v-model="form.desc"
                label="Description"
                variant="outlined"
                rows="2"
                placeholder="Agent description..."
              />
            </v-col>
            <v-col cols="12">
              <v-textarea
                v-model="form.system_prompt"
                label="System Prompt"
                variant="outlined"
                rows="4"
                placeholder="You are a helpful assistant..."
              />
            </v-col>
            <v-col cols="12">
              <v-text-field
                v-model="form.tools"
                label="Tools"
                variant="outlined"
                placeholder="Read, Write, Bash, Grep, WebSearch"
                hint="Comma-separated list of tool names"
                persistent-hint
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
            {{ editingAgent ? 'Update' : 'Create' }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Delete Confirmation Dialog -->
    <v-dialog v-model="deleteDialog" max-width="400">
      <v-card>
        <v-card-title>Delete Agent</v-card-title>
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
import { useAgentStore } from '@/stores/agents'
import { useAppStore } from '@/stores/app'
import type { Agent } from '@/api'

const agentStore = useAgentStore()
const appStore = useAppStore()

const showDialog = ref(false)
const editingAgent = ref<Agent | null>(null)
const saving = ref(false)
const deleteDialog = ref(false)
const deleteTarget = ref<Agent | null>(null)
const deleting = ref(false)

const availableModels = [
  'claude-3-5-sonnet-20241022',
  'claude-3-opus-20240229',
  'claude-3-sonnet-20240229',
  'claude-3-haiku-20240307',
  'gpt-4o',
  'gpt-4-turbo',
]

const defaultForm = () => ({
  name: '',
  desc: '',
  model: 'claude-3-5-sonnet-20241022',
  system_prompt: '',
  tools: '',
  enabled: true,
})

const form = ref(defaultForm())

function openAddDialog() {
  editingAgent.value = null
  form.value = defaultForm()
  showDialog.value = true
}

function openEditDialog(agent: Agent) {
  editingAgent.value = agent
  form.value = {
    name: agent.name,
    desc: agent.desc,
    model: agent.model,
    system_prompt: agent.system_prompt,
    tools: agent.tools,
    enabled: agent.enabled,
  }
  showDialog.value = true
}

async function handleSave() {
  saving.value = true
  try {
    if (editingAgent.value) {
      await agentStore.updateAgent(editingAgent.value.id, { ...form.value })
      appStore.showMessage('Agent updated', 'success')
    } else {
      await agentStore.addAgent({ ...form.value })
      appStore.showMessage('Agent created', 'success')
    }
    showDialog.value = false
    form.value = defaultForm()
    editingAgent.value = null
  } catch {
    appStore.showMessage('Failed to save agent', 'error')
  } finally {
    saving.value = false
  }
}

function confirmRemove(agent: Agent) {
  deleteTarget.value = agent
  deleteDialog.value = true
}

async function handleDelete() {
  if (!deleteTarget.value) return
  deleting.value = true
  try {
    await agentStore.removeAgent(deleteTarget.value.id)
    appStore.showMessage('Agent deleted', 'success')
  } catch {
    appStore.showMessage('Failed to delete agent', 'error')
  } finally {
    deleting.value = false
    deleteDialog.value = false
    deleteTarget.value = null
  }
}

async function handleToggleEnabled(agent: Agent, enabled: boolean) {
  try {
    await agentStore.updateAgent(agent.id, { enabled })
    appStore.showMessage(enabled ? 'Agent enabled' : 'Agent disabled', 'success')
  } catch {
    appStore.showMessage('Failed to update agent', 'error')
  }
}

onMounted(() => {
  agentStore.fetchAgents()
})
</script>

<style scoped>
.system-prompt-preview {
  max-height: 64px;
  overflow: hidden;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  color: rgba(var(--v-theme-on-surface), 0.6);
  line-height: 1.5;
}
</style>
