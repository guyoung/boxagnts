<template>
  <v-navigation-drawer :rail="appStore.sidebarCollapsed" permanent color="surface" :width="appStore.sidebarWidth"
    class="sidebar">
    <div class="sidebar-header pa-4 d-flex align-center">
      <v-icon size="28" color="primary" class="mr-3">mdi-console-line</v-icon>
      <div v-if="!appStore.sidebarCollapsed" class="text-h6 font-weight-bold text-primary">
        Boxagnts
      </div>
    </div>

    <div class="px-1 mt-1 text-center" v-if="appStore.sidebarCollapsed">
      <v-btn icon="mdi-chat-plus" variant="text" size="small" color="primary" @click="newSession" />
    </div>

    <div class="px-2 mt-1 mr-1" v-if="!appStore.sidebarCollapsed">
      <v-btn color="primary" variant="flat" block size="small" prepend-icon="mdi-plus" @click="newSession"
        class="mx-2 mr-4">
        New Session
      </v-btn>
    </div>

    <v-divider class="mx-4 mt-3 mb-1" />

    <!-- Sessions &amp; Files panels -->
    <div class="panel-area" v-if="!appStore.sidebarCollapsed">
      <!-- Sessions list -->
      <div class="panel-section" :class="expandedPanel === 'sessions' ? 'panel-expanded' : 'panel-collapsed'">
        <div class="d-flex align-center px-3 pt-2 pb-1">
          <span class="text-caption font-weight-bold text-medium-emphasis">SESSIONS</span>
          <v-spacer />
          <v-btn icon="mdi-refresh" variant="text" size="x-small" color="medium-emphasis"
            @click="sessionStore.fetchSessions()" :loading="sessionStore.loading" />
          <v-btn :icon="expandedPanel === 'sessions' ? 'mdi-chevron-up' : 'mdi-chevron-down'" variant="text"
            size="x-small" color="medium-emphasis"
            @click="expandedPanel = expandedPanel === 'sessions' ? null : 'sessions'" />
        </div>

        <v-expand-transition>
          <div v-show="expandedPanel === 'sessions'" class="session-list-wrapper">
            <div v-if="sessionStore.loading" class="text-center pa-4">
              <v-progress-circular indeterminate size="20" width="2" color="primary" />
            </div>

            <v-list v-else density="compact" nav class="px-1">
              <v-list-item v-for="s in sessionStore.sessions" :key="s.id"
                :active="sessionStore.currentSessionId === s.id" rounded="lg" class="mb-1 session-item"
                @click="selectSession(s.id)">
                <template #prepend>
                  <v-icon size="16" color="medium-emphasis">mdi-message-text</v-icon>
                </template>
                <v-list-item-title class="text-body-2">
                  {{ s.title || sessionStore.sessionLabel(s) }}
                </v-list-item-title>
                <template #append>
                  <v-menu location="bottom end" :close-on-content-click="true">
                    <template #activator="{ props: menuProps }">
                      <v-btn icon="mdi-dots-vertical" variant="text" size="x-small" color="medium-emphasis"
                        v-bind="menuProps" @click.stop />
                    </template>
                    <v-list density="compact" min-width="150">
                      <v-list-item prepend-icon="mdi-pencil" title="Rename" @click.stop="openRename(s)" />
                      <v-list-item prepend-icon="mdi-delete-sweep" title="Clear Message"
                        @click.stop="confirmClear(s)" />
                      <v-list-item prepend-icon="mdi-delete" title="Delete" @click.stop="confirmDelete(s)" />
                    </v-list>
                  </v-menu>
                </template>
              </v-list-item>

              <div v-if="sessionStore.sessions.length === 0" class="text-center pa-4">
                <p class="text-caption text-medium-emphasis">No sessions yet</p>
              </div>
            </v-list>
          </div>
        </v-expand-transition>
      </div>

      <!-- Files tree -->
      <div class="files-panel panel-section" :class="expandedPanel === 'files' ? 'panel-expanded' : 'panel-collapsed'">
        <v-divider class="mx-4 mb-2" />
        <div class="d-flex align-center px-3 pt-1 pb-1">
          <span class="text-caption font-weight-bold text-medium-emphasis">FILES</span>
          <v-spacer />
          <v-btn icon="mdi-arrow-expand-all" variant="text" size="x-small" color="medium-emphasis"
            @click="fileStore.expandAll()" title="Expand All" />
          <v-btn icon="mdi-arrow-collapse-all" variant="text" size="x-small" color="medium-emphasis"
            @click="fileStore.collapseAll()" title="Collapse All" />
          <v-btn icon="mdi-refresh" variant="text" size="x-small" color="medium-emphasis"
            @click="fileStore.refreshTree()" :loading="fileStore.treeLoading" />
          <v-btn :icon="expandedPanel === 'files' ? 'mdi-chevron-up' : 'mdi-chevron-down'" variant="text" size="x-small"
            color="medium-emphasis" @click="expandedPanel = expandedPanel === 'files' ? null : 'files'" />
        </div>

        <v-expand-transition>
          <div v-show="expandedPanel === 'files'" class="file-tree-wrapper">
            <div v-if="fileStore.treeLoading" class="text-center pa-4">
              <v-progress-circular indeterminate size="20" width="2" color="primary" />
            </div>

            <v-list v-else density="compact" nav class="px-1">
              <FileTreeItem v-for="node in fileStore.treeRoots" :key="node.path" :node="node" :depth="0"
                :current-path="currentFilePath" :selected-file-path="selectedFilePath" @navigate="goToFilePath"
                @select-file="handleSelectFile" />

              <div v-if="fileStore.treeRoots.length === 0" class="text-center pa-4">
                <p class="text-caption text-medium-emphasis">No files</p>
              </div>
            </v-list>
          </div>
        </v-expand-transition>
      </div>
    </div>

    <div v-if="!appStore.sidebarCollapsed" class="resize-handle" @mousedown.prevent="startResize" />

    <template #append>
      <v-divider class="mx-4" />

      <div v-if="appStore.sidebarCollapsed" class="pa-2 d-flex flex-column align-center">
        <v-btn icon="mdi-chart-bar" variant="text" size="small" color="medium-emphasis" @click="navigateTo('/usage')"
          title="Usage Analytics" />
        <v-btn icon="mdi-server-network" variant="text" size="small" color="medium-emphasis" @click="navigateTo('/mcp')"
          title="MCP Servers" />
        <v-btn icon="mdi-web" variant="text" size="small" color="medium-emphasis" @click="navigateTo('/sites')"
          title="Sites" />
        <v-btn icon="mdi-clock-outline" variant="text" size="small" color="medium-emphasis"
          @click="navigateTo('/crons')" title="Crons" />
        <v-btn icon="mdi-robot" variant="text" size="small" color="medium-emphasis" @click="navigateTo('/agents')"
          title="Agents" />
        <v-btn icon="mdi-star" variant="text" size="small" color="medium-emphasis" @click="navigateTo('/skills')"
          title="Skills" />
        <v-btn icon="mdi-hammer-wrench" variant="text" size="small" color="medium-emphasis" @click="navigateTo('/tools')"
          title="Tools" />
        <v-btn icon="mdi-cog" variant="text" size="small" color="medium-emphasis" @click="navigateTo('/settings')"
          title="Settings" />
      </div>

      <template v-if="!appStore.sidebarCollapsed">
        <v-menu location="top end" :close-on-content-click="true">
          <template #activator="{ props: menuProps }">
            <div class="d-flex justify-center py-2">
              <v-btn icon="mdi-dots-grid" variant="text" size="small" color="medium-emphasis" v-bind="menuProps" />
            </div>
          </template>
          <v-list density="compact" min-width="180">
            <v-list-item prepend-icon="mdi-web" title="Sites" @click="navigateTo('/sites')" />
            <v-list-item prepend-icon="mdi-clock-outline" title="Crons" @click="navigateTo('/crons')" />
            <v-divider />
<!--            <v-list-item prepend-icon="mdi-robot" title="Agents" @click="navigateTo('/agents')" />-->
            <v-list-item prepend-icon="mdi-star" title="Skills" @click="navigateTo('/skills')" />
            <v-list-item prepend-icon="mdi-hammer-wrench" title="Tools" @click="navigateTo('/tools')" />
<!--            <v-list-item prepend-icon="mdi-server-network" title="MCP Servers" @click="navigateTo('/mcp')" />-->
            <v-divider />
<!--            <v-list-item prepend-icon="mdi-chart-bar" title="Usage Analytics" @click="navigateTo('/usage')" />-->
            <v-list-item prepend-icon="mdi-cog" title="Settings" @click="navigateTo('/settings')" />
          </v-list>
        </v-menu>
      </template>

      <div class="pa-2">
        <v-btn :icon="appStore.sidebarCollapsed ? 'mdi-chevron-right' : 'mdi-chevron-left'" variant="text" size="small"
          block @click="appStore.toggleSidebar()" />
        <v-btn :icon="appStore.isDark ? 'mdi-weather-sunny' : 'mdi-weather-night'" variant="text" size="small" block
          @click="appStore.toggleTheme()" />
      </div>
    </template>
  </v-navigation-drawer>

  <!-- Delete confirmation dialog -->
  <v-dialog v-model="deleteDialog" max-width="400">
    <v-card>
      <v-card-title>Delete Session</v-card-title>
      <v-card-text>
        <p>Are you sure you want to delete this session?</p>
        <p class="text-body-2 text-medium-emphasis mt-2">
          {{ deleteTarget ? (deleteTarget.title || sessionStore.sessionLabel(deleteTarget)) : '' }}
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

  <!-- Rename dialog -->
  <v-dialog v-model="renameDialog" max-width="400">
    <v-card>
      <v-card-title>Rename Session</v-card-title>
      <v-card-text>
        <v-text-field v-model="renameTitle" label="Session Title" variant="outlined" autofocus
          @keydown.enter="handleRename" />
      </v-card-text>
      <v-card-actions>
        <v-spacer />
        <v-btn variant="text" @click="renameDialog = false">Cancel</v-btn>
        <v-btn color="primary" @click="handleRename" :loading="renaming">Save</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>

  <!-- Clear messages dialog -->
  <v-dialog v-model="clearDialog" max-width="400">
    <v-card>
      <v-card-title>Clear All Messages</v-card-title>
      <v-card-text>
        <p>Are you sure you want to clear all messages from this session?</p>
        <p class="text-body-2 text-medium-emphasis mt-2">
          {{ clearTarget ? (clearTarget.title || sessionStore.sessionLabel(clearTarget)) : '' }}
        </p>
        <p class="text-caption text-error mt-2">This action cannot be undone.</p>
      </v-card-text>
      <v-card-actions>
        <v-spacer />
        <v-btn variant="text" @click="clearDialog = false">Cancel</v-btn>
        <v-btn color="error" @click="handleClear" :loading="clearing">Clear</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { ref, onMounted, nextTick, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useAppStore } from '@/stores/app'
import { useSessionStore } from '@/stores/sessions'
import { useFileStore } from '@/stores/files'
import FileTreeItem from '@/components/FileTreeItem.vue'
import { api, type Session } from '@/api'

const appStore = useAppStore()
const sessionStore = useSessionStore()
const fileStore = useFileStore()
const route = useRoute()
const router = useRouter()

const deleteDialog = ref(false)
const deleteTarget = ref<Session | null>(null)
const deleting = ref(false)
const renameDialog = ref(false)
const renameTarget = ref<Session | null>(null)
const renameTitle = ref('')
const renaming = ref(false)
const clearDialog = ref(false)
const clearTarget = ref<Session | null>(null)
const clearing = ref(false)
const expandedPanel = ref<'sessions' | 'files' | null>('sessions')

function startResize(e: MouseEvent) {
  const startX = e.clientX
  const startWidth = appStore.sidebarWidth

  function onMove(ev: MouseEvent) {
    const delta = ev.clientX - startX
    appStore.setSidebarWidth(startWidth + delta)
  }
  function onUp() {
    document.removeEventListener('mousemove', onMove)
    document.removeEventListener('mouseup', onUp)
  }
  document.addEventListener('mousemove', onMove)
  document.addEventListener('mouseup', onUp)
}

function newSession() {
  sessionStore.selectSession(null)
  router.push('/')
}

function selectSession(id: string) {
  sessionStore.selectSession(id)
  router.push('/')
}

function navigateTo(path: string) {
  sessionStore.selectSession(null)
  router.push(path)
}

function confirmDelete(s: Session) {
  deleteTarget.value = s
  deleteDialog.value = true
}

async function handleDelete() {
  if (!deleteTarget.value) return
  deleting.value = true
  try {
    await sessionStore.deleteSession(deleteTarget.value.id)
    appStore.showMessage('Session deleted', 'success')
  } catch {
    appStore.showMessage('Failed to delete session', 'error')
  } finally {
    deleting.value = false
    deleteDialog.value = false
    deleteTarget.value = null
  }
}

function openRename(s: Session) {
  renameTarget.value = s
  renameTitle.value = s.title || sessionStore.sessionLabel(s)
  renameDialog.value = true
}

async function handleRename() {
  if (!renameTarget.value || !renameTitle.value.trim()) return
  renaming.value = true
  try {
    await api.updateSessionTitle(renameTarget.value.id, renameTitle.value.trim())
    renameTarget.value.title = renameTitle.value.trim()
    appStore.showMessage('Session renamed', 'success')
  } catch {
    appStore.showMessage('Failed to rename session', 'error')
  } finally {
    renaming.value = false
    renameDialog.value = false
    renameTarget.value = null
  }
}

function confirmClear(s: Session) {
  clearTarget.value = s
  clearDialog.value = true
}

async function handleClear() {
  if (!clearTarget.value) return
  clearing.value = true
  try {
    await api.clearSessionMessages(clearTarget.value.id)
    appStore.showMessage('All messages cleared', 'success')
    if (clearTarget.value.id === sessionStore.currentSessionId) {
      const sid = clearTarget.value.id
      sessionStore.selectSession(null)
      await nextTick()
      sessionStore.selectSession(sid)
    }
  } catch {
    appStore.showMessage('Failed to clear messages', 'error')
  } finally {
    clearing.value = false
    clearDialog.value = false
    clearTarget.value = null
  }
}

onMounted(() => {
  sessionStore.fetchSessions()
  fileStore.fetchTree()
})

const currentFilePath = computed(() => (route.query.path as string) || '')

const selectedFilePath = computed(() => fileStore.selectedFile)

function goToFilePath(path: string) {
  fileStore.clearSelectedFile()
  router.push({ path: '/files', query: path ? { path } : undefined })
}

function handleSelectFile(filePath: string) {
  const parentDir = filePath.substring(0, filePath.lastIndexOf('/'))
  router.push({ path: '/files', query: { path: parentDir } })
}
</script>

<style scoped>
.sidebar {
  border-right: 1px solid rgba(var(--v-theme-on-surface), 0.08);
}

.sidebar :deep(.v-navigation-drawer__content) {
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.panel-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
}

.panel-section {
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
}

.panel-section :deep(.v-expand-transition) {
  flex: 1;
  min-height: 0;
  overflow: hidden;
}

.panel-expanded {
  flex: 1;
  flex-shrink: 1;
}

.panel-collapsed {
  flex: 0 0 auto;
  overflow: hidden;
}

.files-panel {
  margin-top: 8px;
}

.session-list-wrapper,
.file-tree-wrapper {
  overflow-y: auto;
  height: 100%;
}

.session-list-wrapper::-webkit-scrollbar,
.file-tree-wrapper::-webkit-scrollbar {
  width: 6px;
}

.session-list-wrapper::-webkit-scrollbar-track,
.file-tree-wrapper::-webkit-scrollbar-track {
  background: rgba(var(--v-theme-on-surface), 0.05);
  border-radius: 4px;
}

.session-list-wrapper::-webkit-scrollbar-thumb,
.file-tree-wrapper::-webkit-scrollbar-thumb {
  background: rgba(var(--v-theme-on-surface), 0.12);
  border-radius: 4px;
}

.session-item :deep(.v-list-item-title) {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.session-item :deep(.v-list-item__prepend) {
  margin-inline-end: 2px;
}

.file-tree-wrapper :deep(.v-list-item__prepend) {
  margin-inline-end: 2px;
}

.resize-handle {
  position: absolute;
  top: 0;
  right: 0;
  bottom: 0;
  width: 4px;
  cursor: col-resize;
  z-index: 10;
}

.resize-handle:hover {
  background: rgba(var(--v-theme-primary), 0.4);
}

:deep(.v-list-item__prepend .v-list-item__spacer) {
  width: 8px !important;
}
</style>
