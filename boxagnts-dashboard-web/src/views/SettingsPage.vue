<template>
  <div>
    <div class="d-flex align-center mb-6">
      <v-icon size="32" color="primary" class="mr-3">mdi-cog</v-icon>
      <h1 class="text-h4 font-weight-bold">Settings</h1>
    </div>

    <v-row>
      <v-col cols="12">
        <v-card class="mb-4">
          <v-card-title class="d-flex align-center">
            <v-icon start>mdi-text</v-icon>
            System Prompt
          </v-card-title>
          <v-card-text>
            <v-textarea
              v-model="settings.system_prompt"
              variant="outlined"
              rows="6"
              placeholder="You are a helpful AI assistant..."
            />
          </v-card-text>
        </v-card>

        <v-card class="mb-4">
          <v-card-title class="d-flex align-center">
            <v-icon start>mdi-robot</v-icon>
            Model Providers
          </v-card-title>
          <v-card-text>
            <v-list density="compact" class="mb-4">
              <v-list-item v-for="provider in settings.model_providers" :key="provider.id" rounded="lg">
                <template #prepend>
                  <v-icon>mdi-server</v-icon>
                </template>
                <v-list-item-title>{{ provider.name }}</v-list-item-title>
                <template #append>
                  <v-btn icon="mdi-pencil" variant="text" @click="editProvider(provider)" />
                  <v-btn icon="mdi-delete" variant="text" color="error" @click="deleteProvider(provider.id)" />
                </template>
              </v-list-item>
              <v-list-item v-if="!settings.model_providers || settings.model_providers.length === 0">
                <v-list-item-title class="text-medium-emphasis">No providers configured</v-list-item-title>
              </v-list-item>
            </v-list>

            <v-btn color="primary" prepend-icon="mdi-plus" @click="openAddProviderDialog">
              Add Provider
            </v-btn>
          </v-card-text>
        </v-card>

        <v-card class="mb-4">
          <v-card-title class="d-flex align-center">
            <v-icon start>mdi-star</v-icon>
            Default Settings
          </v-card-title>
          <v-card-text>
            <v-select
              v-model="settings.default_provider"
              label="Default Provider"
              :items="providerOptions"
              variant="outlined"
              class="mb-4"
              :disabled="!providerOptions.length"
            />
            <v-select
              v-model="settings.default_model"
              label="Default Model"
              :items="modelOptions"
              variant="outlined"
              class="mb-4"
              :disabled="!modelOptions.length"
            />
          </v-card-text>
        </v-card>

        <v-card>
          <v-card-title class="d-flex align-center">
            <v-icon start>mdi-security</v-icon>
            Allowed Outbound Hosts
          </v-card-title>
          <v-card-text>
            <v-list density="compact" class="mb-4">
              <v-list-item v-for="(host, idx) in settings.allowed_outbound_hosts" :key="idx" rounded="lg">
                <template #prepend>
                  <v-icon>mdi-server</v-icon>
                </template>
                <v-list-item-title>{{ host }}</v-list-item-title>
                <template #append>
                  <v-btn icon="mdi-delete" variant="text" color="error" @click="deleteOutboundHost(idx)" />
                </template>
              </v-list-item>
              <v-list-item v-if="!settings.allowed_outbound_hosts || settings.allowed_outbound_hosts.length === 0">
                <v-list-item-title class="text-medium-emphasis">No outbound hosts configured</v-list-item-title>
              </v-list-item>
            </v-list>

            <v-row dense align="center">
              <v-col cols="10">
                <v-text-field
                  v-model="newOutboundHost"
                  label="Add Host"
                  variant="outlined"
                  placeholder="api.example.com"
                  @keyup.enter="addOutboundHost"
                />
              </v-col>
              <v-col cols="2">
                <v-btn color="primary" block @click="addOutboundHost" :disabled="!newOutboundHost.trim()">
                  Add
                </v-btn>
              </v-col>
            </v-row>
          </v-card-text>
        </v-card>

        <div class="d-flex justify-end mt-4">
          <v-btn color="primary" size="large" :loading="saving" @click="saveSettings">
            <v-icon start>mdi-content-save</v-icon> Save Settings
          </v-btn>
        </div>
      </v-col>
    </v-row>

    <!-- Provider Dialog -->
    <v-dialog v-model="providerDialogOpen" max-width="500">
      <v-card>
        <v-card-title>{{ editingProvider ? 'Edit Provider' : 'Add Provider' }}</v-card-title>
        <v-card-text>
          <v-row dense>
            <v-col cols="12">
              <v-text-field
                v-model="providerForm.name"
                label="Name"
                variant="outlined"
                placeholder="e.g., OpenAI"
              />
            </v-col>
            <v-col cols="12">
              <v-text-field
                v-model="providerForm.api_key"
                label="API Key"
                variant="outlined"
                :type="passwordVisible ? 'text' : 'password'"
                :append-inner-icon="passwordVisible ? 'mdi-eye-off' : 'mdi-eye'"
                @click:append-inner="passwordVisible = !passwordVisible"
              />
            </v-col>
            <v-col cols="12">
              <v-text-field
                v-model="providerForm.base_url"
                label="Base URL"
                variant="outlined"
                placeholder="https://api.openai.com/v1"
              />
            </v-col>
            <v-col cols="12">
              <v-text-field
                v-model="modelsInput"
                label="Models"
                variant="outlined"
                placeholder="gpt-4o, gpt-4, gpt-3.5-turbo"
                hint="Comma-separated list of model names"
                persistent-hint
              />
            </v-col>
          </v-row>
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="closeProviderDialog">Cancel</v-btn>
          <v-btn color="primary" @click="saveProvider">
            {{ editingProvider ? 'Update' : 'Add' }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { api, type Settings, type ModelProvider } from '@/api'
import { useAppStore } from '@/stores/app'

const appStore = useAppStore()

const saving = ref(false)
const providerDialogOpen = ref(false)
const editingProvider = ref<ModelProvider | null>(null)
const passwordVisible = ref(false)

const defaultSettings: Settings = {
  system_prompt: '',
  model_providers: [],
  default_model: '',
  default_provider: '',
  allowed_outbound_hosts: [],
}

const settings = ref<Settings>({ ...defaultSettings })
const newOutboundHost = ref('')

const providerForm = ref<Partial<ModelProvider>>({
  name: '',
  api_key: '',
  base_url: '',
  models: [],
})

const modelsInput = ref('')

const providerOptions = computed(() => {
  if (!settings.value.model_providers) return []
  return settings.value.model_providers.map(p => ({
    title: p.name,
    value: p.id,
  }))
})

const modelOptions = computed(() => {
  if (!settings.value.default_provider || !settings.value.model_providers) return []
  const provider = settings.value.model_providers.find(p => p.id === settings.value.default_provider)
  return provider?.models || []
})

async function loadSettings() {
  try {
    const data = await api.getSettings()
    settings.value = { ...defaultSettings, ...data }
  } catch (e) {
    console.error('Failed to load settings:', e)
  }
}

async function saveSettings() {
  saving.value = true
  try {
    await api.saveSettings(settings.value)
    appStore.showMessage('Settings saved!', 'success')
  } catch {
    appStore.showMessage('Failed to save settings', 'error')
  } finally {
    saving.value = false
  }
}

function openAddProviderDialog() {
  editingProvider.value = null
  providerForm.value = {
    name: '',
    api_key: '',
    base_url: '',
    models: [],
  }
  modelsInput.value = ''
  providerDialogOpen.value = true
}

function editProvider(provider: ModelProvider) {
  editingProvider.value = { ...provider }
  providerForm.value = { ...provider }
  modelsInput.value = provider.models.join(', ')
  providerDialogOpen.value = true
}

function closeProviderDialog() {
  providerDialogOpen.value = false
  editingProvider.value = null
  providerForm.value = {}
  modelsInput.value = ''
}

function saveProvider() {
  if (!providerForm.value.name) return

  const newProvider: ModelProvider = {
    id: editingProvider.value ? editingProvider.value.id : 'provider_' + Date.now(),
    name: providerForm.value.name || '',
    api_key: providerForm.value.api_key,
    base_url: providerForm.value.base_url,
    models: modelsInput.value.split(',').map(m => m.trim()).filter(Boolean),
  }

  if (!settings.value.model_providers) {
    settings.value.model_providers = []
  }

  if (editingProvider.value) {
    const idx = settings.value.model_providers.findIndex(p => p.id === editingProvider.value!.id)
    if (idx >= 0) {
      settings.value.model_providers[idx] = newProvider
    }
  } else {
    settings.value.model_providers.push(newProvider)
  }

  closeProviderDialog()
}

function deleteProvider(id: string) {
  if (!settings.value.model_providers) return
  settings.value.model_providers = settings.value.model_providers.filter(p => p.id !== id)
  if (settings.value.default_provider === id) {
    settings.value.default_provider = ''
    settings.value.default_model = ''
  }
}

function addOutboundHost() {
  const host = newOutboundHost.value.trim()
  if (!host) return
  if (!settings.value.allowed_outbound_hosts) {
    settings.value.allowed_outbound_hosts = []
  }
  if (!settings.value.allowed_outbound_hosts.includes(host)) {
    settings.value.allowed_outbound_hosts.push(host)
  }
  newOutboundHost.value = ''
}

function deleteOutboundHost(idx: number) {
  if (!settings.value.allowed_outbound_hosts) return
  settings.value.allowed_outbound_hosts.splice(idx, 1)
}

onMounted(() => {
  loadSettings()
})
</script>
