<template>
  <div>
    <div class="d-flex align-center mb-2">
      <v-icon color="primary" class="mr-2">mdi-text</v-icon>
      <h2 class="text-h5 font-weight-bold">AGENTS.md</h2>
    </div>
    <p class="text-body-2 text-medium-emphasis mb-4">
      Configure the default AGENTS.md instructions that will be used for all AI interactions.
    </p>

    <v-card>
      <v-card-text>
        <v-textarea
          v-model="content"
          variant="outlined"
          rows="10"
          placeholder="Enter AGENTS.md content..."
          :loading="loading"
        />
      </v-card-text>
    </v-card>

    <div class="d-flex justify-end mt-4">
      <v-btn color="primary" size="large" :loading="saving" @click="handleSave">
        <v-icon start>mdi-content-save</v-icon> Save
      </v-btn>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { api } from '@/api'
import { useAppStore } from '@/stores/app'

const appStore = useAppStore()
const content = ref('')
const loading = ref(false)
const saving = ref(false)

onMounted(async () => {
  loading.value = true
  try {
    content.value = await api.getAgentsMd()
  } catch {
    appStore.showMessage('Failed to load AGENTS.md', 'error')
  } finally {
    loading.value = false
  }
})

async function handleSave() {
  saving.value = true
  try {
    await api.updateAgentsMd(content.value)
    appStore.showMessage('AGENTS.md saved successfully!', 'success')
  } catch {
    appStore.showMessage('Failed to save AGENTS.md', 'error')
  } finally {
    saving.value = false
  }
}
</script>
