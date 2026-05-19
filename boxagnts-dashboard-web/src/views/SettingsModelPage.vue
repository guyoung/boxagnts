<template>
  <div>
    <div class="d-flex align-center mb-2">
      <v-icon color="primary" class="mr-2">mdi-robot</v-icon>
      <h2 class="text-h5 font-weight-bold">Model Settings</h2>
    </div>
    <p class="text-body-2 text-medium-emphasis mb-4">
      Manage model providers and configure default model settings.
    </p>

    <v-card class="mb-4">
      <v-card-title class="d-flex align-center">
        <v-icon start>mdi-server</v-icon>
        Model Providers
      </v-card-title>
      <v-card-text>
        <v-list v-if="providersLoading" class="pa-4 text-center">
          <v-progress-circular indeterminate size="24" width="2" color="primary" />
        </v-list>

        <v-list v-else density="compact">
          <template v-for="provider in providers" :key="provider.id">
            <v-list-item rounded="lg" class="mb-1" @click="toggleExpand(provider.id)">
              <template #prepend>
                <v-icon :color="provider.enabled ? 'success' : 'medium-emphasis'">
                  {{ provider.enabled ? 'mdi-server' : 'mdi-server-off' }}
                </v-icon>
              </template>

              <v-list-item-title class="font-weight-medium">{{ provider.name }}</v-list-item-title>
              <v-list-item-subtitle class="text-caption">
                <span class="text-medium-emphasis">{{ provider.id }}</span>
                <span v-if="provider.models.length"> · {{ provider.models.length }} models</span>
              </v-list-item-subtitle>

              <template #append>
                <v-chip :color="provider.enabled ? 'success' : 'medium-emphasis'" size="x-small" variant="tonal" class="mr-1">
                  {{ provider.enabled ? 'On' : 'Off' }}
                </v-chip>
                <v-icon size="18" color="medium-emphasis" class="mr-1">
                  {{ isExpanded(provider.id) ? 'mdi-chevron-up' : 'mdi-chevron-down' }}
                </v-icon>
                <v-btn icon="mdi-pencil" variant="text" size="small" @click.stop="openEditDialog(provider)" />
                <v-btn icon="mdi-delete" variant="text" size="small" color="error" @click.stop="confirmDelete(provider)" />
              </template>
            </v-list-item>

            <v-expand-transition>
              <div v-show="isExpanded(provider.id)" class="model-list ml-8 mb-2">
                <v-divider class="mb-1" />
                <div class="text-caption text-medium-emphasis pa-2 font-weight-bold">MODELS</div>
                <div v-if="provider.models.length === 0" class="px-3 pb-2">
                  <span class="text-caption text-medium-emphasis">No models</span>
                </div>
                <v-list density="compact" nav class="px-1">
                  <v-list-item v-for="model in provider.models" :key="model.id" rounded="lg" class="mb-1">
                    <template #prepend>
                      <v-icon size="16" color="medium-emphasis">mdi-cube-outline</v-icon>
                    </template>
                    <v-list-item-title class="text-body-2">{{ model.name }}</v-list-item-title>
                    <v-list-item-subtitle class="text-caption text-medium-emphasis">{{ model.id }}</v-list-item-subtitle>
                    <template #append>
                      <v-btn icon="mdi-pencil" variant="text" size="x-small" @click.stop="openEditModelDialog(provider.id, model)" />
                      <v-btn icon="mdi-delete" variant="text" size="x-small" color="error" @click.stop="confirmDeleteModel(provider.id, model)" />
                    </template>
                  </v-list-item>
                </v-list>
                <div class="px-3 pb-2">
                  <v-btn variant="tonal" size="x-small" prepend-icon="mdi-plus" @click.stop="openAddModelDialog(provider.id)">
                    Add Model
                  </v-btn>
                </div>
              </div>
            </v-expand-transition>
          </template>

          <v-list-item v-if="providers.length === 0">
            <v-list-item-title class="text-medium-emphasis">No providers configured</v-list-item-title>
          </v-list-item>
        </v-list>

        <v-btn color="primary" prepend-icon="mdi-plus" class="mt-2" @click="openAddDialog">
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
          v-model="settings.settings.default_provider"
          label="Default Provider"
          :items="providerOptions"
          variant="outlined"
          class="mb-4"
          :disabled="!providerOptions.length"
        />
        <v-select
          v-model="settings.settings.default_model"
          label="Default Model"
          :items="modelOptions"
          variant="outlined"
          class="mb-4"
          :disabled="!modelOptions.length"
        />
      </v-card-text>
    </v-card>

    <div class="d-flex justify-end mt-4">
      <v-btn color="primary" size="large" :loading="updatingDefault" @click="handleUpdateDefaultModel">
        <v-icon start>mdi-content-save</v-icon> Update Default Model
      </v-btn>
    </div>

    <!-- Provider Dialog -->
    <v-dialog v-model="dialogOpen" max-width="500">
      <v-card>
        <v-card-title>{{ isEditing ? 'Edit Provider' : 'Add Provider' }}</v-card-title>
        <v-card-text>
          <v-row dense>
            <v-col cols="12" v-if="isEditing">
              <v-text-field v-model="form.name" label="Name" variant="outlined" />
            </v-col>
            <v-col cols="12" v-if="!isEditing">
              <v-autocomplete
                v-model="form.id"
                label="Provider"
                variant="outlined"
                :items="providerOptionItems"
                item-title="title"
                item-value="id"
                :loading="optionsLoading"
                :rules="[v => !!v || 'Required']"
                @update:model-value="onProviderSelected"
              >
                <template #item="{ props, item }">
                  <v-list-item v-bind="props" :subtitle="item.raw.description" :prepend-icon="'mdi-server'">
                    <template #title>
                      <div class="d-flex align-center">
                        {{ item.raw.title }}
                        <v-chip v-if="item.raw.category" size="x-small" variant="tonal" class="ml-2">
                          {{ item.raw.category }}
                        </v-chip>
                      </div>
                    </template>
                  </v-list-item>
                </template>
              </v-autocomplete>
            </v-col>
            <v-col cols="12">
              <v-text-field v-model="form.api_base" label="API Base URL" variant="outlined" placeholder="https://api.example.com/v1" />
            </v-col>
            <v-col cols="12">
              <v-text-field
                v-model="form.api_key"
                label="API Key"
                variant="outlined"
                :type="passwordVisible ? 'text' : 'password'"
                :append-inner-icon="passwordVisible ? 'mdi-eye-off' : 'mdi-eye'"
                @click:append-inner="passwordVisible = !passwordVisible"
              />
            </v-col>
            <v-col cols="12">
              <v-switch v-model="form.enabled" label="Enabled" color="success" density="compact" hide-details />
            </v-col>
          </v-row>
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="dialogOpen = false">Cancel</v-btn>
          <v-btn color="primary" :loading="savingProvider" @click="handleSaveProvider">
            {{ isEditing ? 'Update' : 'Add' }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Delete Provider Dialog -->
    <v-dialog v-model="deleteConfirmOpen" max-width="400">
      <v-card>
        <v-card-title>Delete Provider</v-card-title>
        <v-card-text>
          Are you sure you want to delete <strong>{{ deletingProvider?.name }}</strong>?
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="deleteConfirmOpen = false">Cancel</v-btn>
          <v-btn color="error" :loading="deletingProviderLoading" @click="handleDeleteProvider">Delete</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Model Dialog -->
    <v-dialog v-model="modelDialogOpen" max-width="500">
      <v-card>
        <v-card-title>{{ isEditingModel ? 'Edit Model' : 'Add Model' }}</v-card-title>
        <v-card-text>
          <v-row dense>
            <v-col cols="12" v-if="!isEditingModel">
              <v-text-field v-model="modelForm.id" label="Model ID" variant="outlined" placeholder="deepseek/deepseek-v4-pro" />
            </v-col>
            <v-col cols="12">
              <v-text-field v-model="modelForm.name" label="Name" variant="outlined" />
            </v-col>
          </v-row>
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="modelDialogOpen = false">Cancel</v-btn>
          <v-btn color="primary" :loading="savingModel" @click="handleSaveModel">
            {{ isEditingModel ? 'Update' : 'Add' }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Delete Model Dialog -->
    <v-dialog v-model="deleteModelConfirmOpen" max-width="400">
      <v-card>
        <v-card-title>Delete Model</v-card-title>
        <v-card-text>
          Are you sure you want to delete <strong>{{ deletingModel?.name }}</strong>?
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="deleteModelConfirmOpen = false">Cancel</v-btn>
          <v-btn color="error" :loading="deletingModelLoading" @click="handleDeleteModel">Delete</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useSettingsStore } from '@/stores/settings'
import { useAppStore } from '@/stores/app'
import { api, type ConfigProvider, type ProviderOption, type ProviderModel } from '@/api'

const settings = useSettingsStore()
const appStore = useAppStore()

const providers = ref<ConfigProvider[]>([])
const providersLoading = ref(false)
const providerOptionsData = ref<ProviderOption[]>([])
const optionsLoading = ref(false)

const expandedSet = ref<Set<string>>(new Set())
const updatingDefault = ref(false)

function isExpanded(id: string) {
  return expandedSet.value.has(id)
}

function toggleExpand(id: string) {
  const s = new Set(expandedSet.value)
  if (s.has(id)) {
    s.delete(id)
  } else {
    s.add(id)
  }
  expandedSet.value = s
}

const savingProvider = ref(false)
const deletingProviderLoading = ref(false)

const dialogOpen = ref(false)
const deleteConfirmOpen = ref(false)
const isEditing = ref(false)
const passwordVisible = ref(false)
const editingProviderId = ref<string | null>(null)
const deletingProvider = ref<ConfigProvider | null>(null)

const form = ref({
  id: '',
  name: '',
  api_base: '',
  api_key: '',
  enabled: true,
})

const modelDialogOpen = ref(false)
const deleteModelConfirmOpen = ref(false)
const isEditingModel = ref(false)
const savingModel = ref(false)
const deletingModelLoading = ref(false)
const editingModelProviderId = ref('')
const editingModelId = ref('')
const deletingModelProviderId = ref('')
const deletingModel = ref<ProviderModel | null>(null)

const modelForm = ref({
  id: '',
  name: '',
})

const providerOptionItems = computed(() => providerOptionsData.value)

const providerOptions = computed(() => {
  return providers.value.map(p => ({ title: p.name, value: p.id }))
})

const modelOptions = computed(() => {
  if (!settings.settings.default_provider) return []
  const provider = providers.value.find(p => p.id === settings.settings.default_provider)
  return (provider?.models || []).map(m => ({ title: m.name, value: m.id }))
})

async function fetchProviders() {
  providersLoading.value = true
  try {
    providers.value = await api.getConfigProviders()
  } catch (e) {
    console.error('Failed to fetch providers:', e)
    providers.value = []
  } finally {
    providersLoading.value = false
  }
}

async function fetchProviderOptions() {
  optionsLoading.value = true
  try {
    providerOptionsData.value = await api.getProviderOptions()
  } catch (e) {
    console.error('Failed to fetch provider options:', e)
    providerOptionsData.value = []
  } finally {
    optionsLoading.value = false
  }
}

async function handleUpdateDefaultModel() {
  if (!settings.settings.default_model) return
  updatingDefault.value = true
  try {
    await api.updateDefaultModel(settings.settings.default_model)
    appStore.showMessage('Default model updated!', 'success')
  } catch (e) {
    appStore.showMessage('Failed to update default model', 'error')
  } finally {
    updatingDefault.value = false
  }
}

function onProviderSelected(id: string) {
  const option = providerOptionsData.value.find(o => o.id === id)
  if (option) {
    form.value.name = option.title
  }
}

function openAddDialog() {
  isEditing.value = false
  editingProviderId.value = null
  form.value = { id: '', name: '', api_base: '', api_key: '', enabled: true }
  dialogOpen.value = true
}

function openEditDialog(provider: ConfigProvider) {
  isEditing.value = true
  editingProviderId.value = provider.id
  form.value = {
    id: provider.id,
    name: provider.name,
    api_base: provider.api_base,
    api_key: provider.api_key || '',
    enabled: provider.enabled,
  }
  dialogOpen.value = true
}

async function handleSaveProvider() {
  if (!form.value.id) return

  savingProvider.value = true
  try {
    if (isEditing.value && editingProviderId.value) {
      await api.updateConfigProvider(editingProviderId.value, {
        name: form.value.name,
        api_base: form.value.api_base,
        api_key: form.value.api_key,
        enabled: form.value.enabled,
      })
      appStore.showMessage('Provider updated!', 'success')
    } else {
      await api.createConfigProvider({
        id: form.value.id,
        name: form.value.name,
        api_base: form.value.api_base,
        api_key: form.value.api_key,
        enabled: form.value.enabled,
      })
      appStore.showMessage('Provider added!', 'success')
    }
    dialogOpen.value = false
    await fetchProviders()
  } catch (e) {
    appStore.showMessage('Failed to save provider', 'error')
  } finally {
    savingProvider.value = false
  }
}

function confirmDelete(provider: ConfigProvider) {
  deletingProvider.value = provider
  deleteConfirmOpen.value = true
}

async function handleDeleteProvider() {
  if (!deletingProvider.value) return
  deletingProviderLoading.value = true
  try {
    await api.deleteConfigProvider(deletingProvider.value.id)
    if (settings.settings.default_provider === deletingProvider.value.id) {
      settings.settings.default_provider = ''
      settings.settings.default_model = ''
    }
    appStore.showMessage('Provider deleted!', 'success')
    deleteConfirmOpen.value = false
    await fetchProviders()
  } catch (e) {
    appStore.showMessage('Failed to delete provider', 'error')
  } finally {
    deletingProviderLoading.value = false
  }
}

function openAddModelDialog(providerId: string) {
  isEditingModel.value = false
  editingModelProviderId.value = providerId
  editingModelId.value = ''
  modelForm.value = { id: '', name: '' }
  modelDialogOpen.value = true
}

function openEditModelDialog(providerId: string, model: ProviderModel) {
  isEditingModel.value = true
  editingModelProviderId.value = providerId
  editingModelId.value = model.id
  modelForm.value = { id: model.id, name: model.name }
  modelDialogOpen.value = true
}

async function handleSaveModel() {
  if (!modelForm.value.name) return

  savingModel.value = true
  try {
    if (isEditingModel.value) {
      await api.updateProviderModel(editingModelProviderId.value, editingModelId.value, {
        name: modelForm.value.name,
      })
      appStore.showMessage('Model updated!', 'success')
    } else {
      if (!modelForm.value.id) return
      await api.createProviderModel(editingModelProviderId.value, {
        id: modelForm.value.id,
        name: modelForm.value.name,
      })
      appStore.showMessage('Model added!', 'success')
    }
    modelDialogOpen.value = false
    await fetchProviders()
  } catch (e) {
    appStore.showMessage('Failed to save model', 'error')
  } finally {
    savingModel.value = false
  }
}

function confirmDeleteModel(providerId: string, model: ProviderModel) {
  deletingModelProviderId.value = providerId
  deletingModel.value = model
  deleteModelConfirmOpen.value = true
}

async function handleDeleteModel() {
  if (!deletingModel.value) return
  deletingModelLoading.value = true
  try {
    await api.deleteProviderModel(deletingModelProviderId.value, deletingModel.value.id)
    if (settings.settings.default_provider === deletingModelProviderId.value &&
        settings.settings.default_model === deletingModel.value.id) {
      settings.settings.default_model = ''
    }
    appStore.showMessage('Model deleted!', 'success')
    deleteModelConfirmOpen.value = false
    await fetchProviders()
  } catch (e) {
    appStore.showMessage('Failed to delete model', 'error')
  } finally {
    deletingModelLoading.value = false
  }
}

onMounted(() => {
  fetchProviders()
  fetchProviderOptions()
})
</script>
