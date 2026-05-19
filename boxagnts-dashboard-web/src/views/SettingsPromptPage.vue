<template>
  <div>
    <div class="d-flex align-center mb-2">
      <v-icon color="primary" class="mr-2">mdi-text</v-icon>
      <h2 class="text-h5 font-weight-bold">System Prompt</h2>
    </div>
    <p class="text-body-2 text-medium-emphasis mb-4">
      Configure the default system prompt that will be used for all AI interactions.
    </p>

    <v-card>
      <v-card-text>
        <v-textarea
          v-model="settingsStore.settings.system_prompt"
          variant="outlined"
          rows="10"
          placeholder="You are a helpful AI assistant..."
        />
      </v-card-text>
    </v-card>

    <div class="d-flex justify-end mt-4">
      <v-btn color="primary" size="large" @click="handleSave">
        <v-icon start>mdi-content-save</v-icon> Save
      </v-btn>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useSettingsStore } from '@/stores/settings'
import { useAppStore } from '@/stores/app'

const settingsStore = useSettingsStore()
const appStore = useAppStore()

function handleSave() {
  appStore.showMessage('System prompt saved locally!', 'success')
}
</script>
