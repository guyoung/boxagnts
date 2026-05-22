<template>
  <div>
    <div class="d-flex align-center mb-2">
      <v-icon color="primary" class="mr-2">mdi-security</v-icon>
      <h2 class="text-h5 font-weight-bold">Security Settings</h2>
    </div>
    <p class="text-body-2 text-medium-emphasis mb-4">
      Manage allowed outbound hosts for network access control.
    </p>

    <v-card>
      <v-card-title class="d-flex align-center">
        <v-icon start>mdi-server-network</v-icon>
        Allowed Outbound Hosts
      </v-card-title>
      <v-card-text>
        <v-list density="compact" class="mb-4">
          <v-list-item v-for="(host, idx) in settings.settings.allowed_outbound_hosts" :key="idx" rounded="lg">
            <template #prepend>
              <v-icon>mdi-server</v-icon>
            </template>
            <v-list-item-title>{{ host }}</v-list-item-title>
            <template #append>
              <v-btn icon="mdi-delete" variant="text" color="error" @click="deleteOutboundHost(idx)" />
            </template>
          </v-list-item>
          <v-list-item v-if="!settings.settings.allowed_outbound_hosts || settings.settings.allowed_outbound_hosts.length === 0">
            <v-list-item-title class="text-medium-emphasis">No outbound hosts configured</v-list-item-title>
          </v-list-item>
        </v-list>

        <v-row dense align="center">
          <v-col cols="10">
            <v-text-field
              v-model="newOutboundHost"
              label="Add Host"
              variant="outlined"
              placeholder="https://api.example.com"
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
      <v-btn color="primary" size="large" @click="handleSave" :loading="saving">
        <v-icon start>mdi-content-save</v-icon> Save
      </v-btn>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useSettingsStore } from '@/stores/settings'
import { useAppStore } from '@/stores/app'
import { api } from '@/api'

const settings = useSettingsStore()
const appStore = useAppStore()
const newOutboundHost = ref('')
const saving = ref(false)
const loading = ref(false)

onMounted(async () => {
  loading.value = true
  try {
    const hosts = await api.getAllowedOutboundHosts()
    settings.settings.allowed_outbound_hosts = hosts
  } catch {
    appStore.showMessage('Failed to load allowed outbound hosts', 'error')
  } finally {
    loading.value = false
  }
})

async function handleSave() {
  saving.value = true
  try {
    await api.updateAllowedOutboundHosts(settings.settings.allowed_outbound_hosts || [])
    appStore.showMessage('Security settings saved!', 'success')
  } catch {
    appStore.showMessage('Failed to save security settings', 'error')
  } finally {
    saving.value = false
  }
}

function addOutboundHost() {
  const host = newOutboundHost.value.trim()
  if (!host) return
  if (!settings.settings.allowed_outbound_hosts) {
    settings.settings.allowed_outbound_hosts = []
  }
  if (!settings.settings.allowed_outbound_hosts.includes(host)) {
    settings.settings.allowed_outbound_hosts.push(host)
  }
  newOutboundHost.value = ''
}

function deleteOutboundHost(idx: number) {
  if (!settings.settings.allowed_outbound_hosts) return
  settings.settings.allowed_outbound_hosts.splice(idx, 1)
}
</script>
