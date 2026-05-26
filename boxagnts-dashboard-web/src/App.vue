<template>
  <v-app>
    <AppSidebar />
    <v-main>
      <v-container fluid class="pa-6">
        <router-view v-slot="{ Component, route }">
            <component :is="Component" :key="route.path" />
        </router-view>
      </v-container>
    </v-main>
    <RightSidebar />
  </v-app>
</template>

<script setup lang="ts">
import { watch } from 'vue'
import { useTheme } from 'vuetify'
import AppSidebar from '@/components/AppSidebar.vue'
import RightSidebar from '@/components/RightSidebar.vue'
import { useAppStore } from '@/stores/app'
import '@mdi/font/css/materialdesignicons.css'


const appStore = useAppStore()
const theme = useTheme()

watch(() => appStore.theme, (val) => {
  theme.global.name.value = val
}, { immediate: true })
</script>

<style>
html,
body {
  overflow: hidden;
  height: 100%;
}

* {
  scrollbar-width: thin;
  scrollbar-color: rgba(var(--v-theme-on-surface), 0.15) transparent;
}

.v-application {
  height: 100vh;
}

.v-main {
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.v-container--fluid {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.v-container--fluid > * {
  flex: 1;
  min-height: 0;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.15s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: transparent;
  border-radius: 4px;
}

::-webkit-scrollbar-thumb {
  background: rgba(var(--v-theme-on-surface), 0.15);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: rgba(var(--v-theme-on-surface), 0.25);
}
</style>
