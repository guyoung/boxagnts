<template>
  <div>
    <div class="d-flex align-center mb-6">
      <v-icon size="32" color="primary" class="mr-3">mdi-cog</v-icon>
      <h1 class="text-h4 font-weight-bold">Settings</h1>
    </div>

    <v-tabs v-model="activeTab" color="primary" class="mb-6">
      <v-tab value="model" prepend-icon="mdi-robot" @click="navigateTo('model')">
        Model Settings
      </v-tab>
      <v-tab value="prompt" prepend-icon="mdi-text" @click="navigateTo('prompt')">
        System Prompt
      </v-tab>
      <v-tab value="security" prepend-icon="mdi-security" @click="navigateTo('security')">
        Security
      </v-tab>
    </v-tabs>

    <router-view />
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'

const router = useRouter()
const route = useRoute()

const activeTab = ref('model')

function getTabFromPath(path: string): string {
  if (path.includes('/settings/security')) return 'security'
  if (path.includes('/settings/prompt')) return 'prompt'
  return 'model'
}

watch(() => route.path, (val) => {
  activeTab.value = getTabFromPath(val)
}, { immediate: true })

function navigateTo(tab: string) {
  router.push(`/settings/${tab}`)
}
</script>
