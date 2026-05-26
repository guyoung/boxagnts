<template>
  <v-navigation-drawer
    v-bind="drawerProps"
    location="right"
    color="surface"
    :width="280"
    :rail-width="48"
    class="right-sidebar"
  >
    <div
      class="project-bar px-3 py-1 d-flex align-center"
    >
      <template v-if="appStore.currentProject && !appStore.rightSidebarCollapsed">
        <v-icon size="16" color="primary" class="mr-2 flex-shrink-0">mdi-folder</v-icon>
        <span class="text-body-2 text-truncate project-name">{{ appStore.currentProject.id }}</span>
        <v-spacer />
      </template>
      <template v-else-if="appStore.rightSidebarCollapsed">
        <v-icon
          v-if="appStore.currentProject"
          size="18"
          color="primary"
          class="mx-auto"
        >mdi-folder</v-icon>
      </template>
      <v-btn
        :icon="appStore.rightSidebarCollapsed ? 'mdi-chevron-left' : 'mdi-chevron-right'"
        variant="text"
        size="small"
        density="compact"
        color="medium-emphasis"
        class="collapse-btn"
        @click="appStore.toggleRightSidebar()"
        :title="appStore.rightSidebarCollapsed ? 'Expand' : 'Collapse'"
      />
    </div>

    <div v-if="!appStore.rightSidebarCollapsed" ref="sidebarContentRef" class="sidebar-content">
      <div class="toolbar px-3 py-1 d-flex align-center ga-1">
        <span class="text-caption text-medium-emphasis font-weight-bold">FILES</span>
        <v-spacer />
        <v-btn
          icon="mdi-file-plus"
          variant="text"
          size="x-small"
          color="medium-emphasis"
          class="toolbar-btn"
          title="New File"
          @click="showNewFileDialog = true"
        />
        <v-btn
          icon="mdi-folder-plus"
          variant="text"
          size="x-small"
          color="medium-emphasis"
          class="toolbar-btn"
          title="New Folder"
          @click="showMkdirDialog = true"
        />
        <v-btn
          icon="mdi-refresh"
          variant="text"
          size="x-small"
          color="medium-emphasis"
          class="toolbar-btn"
          title="Refresh"
          @click="fileStore.refreshTree()"
          :loading="fileStore.treeLoading"
        />
        <v-btn
          :icon="fileStore.treeAllExpanded ? 'mdi-arrow-collapse-all' : 'mdi-arrow-expand-all'"
          variant="text"
          size="x-small"
          color="medium-emphasis"
          class="toolbar-btn"
          :title="fileStore.treeAllExpanded ? 'Collapse All' : 'Expand All'"
          @click="fileStore.toggleExpandAll()"
        />
      </div>

      <div
        v-if="fileStore.clipboard"
        class="clipboard-bar px-3 py-1 d-flex align-center"
      >
        <v-icon size="14" class="mr-1 clipboard-icon">
          {{ fileStore.clipboard.mode === 'cut' ? 'mdi-content-cut' : 'mdi-content-copy' }}
        </v-icon>
        <span class="text-caption text-truncate clipboard-text">{{ fileStore.clipboard.name }}</span>
        <v-spacer />
        <v-btn
          icon="mdi-close"
          variant="text"
          size="x-small"
          color="medium-emphasis"
          density="compact"
          @click="fileStore.clearClipboard()"
        />
      </div>

      <div
        ref="fileTreeWrapperRef"
        class="file-tree-wrapper"
        @contextmenu.prevent="onTreeContextMenu"
      >
        <div v-if="fileStore.treeLoading" class="text-center pa-4">
          <v-progress-circular indeterminate size="20" width="2" color="primary" />
        </div>

        <v-list v-else density="compact" nav class="px-1">
          <FileTreeItem
            v-for="node in fileStore.treeRoots"
            :key="node.path"
            :node="node"
            :depth="0"
            :current-path="currentPath"
            :selected-file-path="selectedFilePath"
            @navigate="handleNavigate"
            @select-file="handleSelectFile"
            @rename="openRename"
            @delete="confirmDelete"
          />

          <div v-if="fileStore.treeRoots.length === 0" class="text-center pa-6">
            <v-icon size="40" color="medium-emphasis" class="mb-2">mdi-folder-open-outline</v-icon>
            <p class="text-caption text-medium-emphasis">No files</p>
          </div>
        </v-list>
      </div>

      <div
        v-if="rootMenu"
        ref="rootMenuRef"
        class="root-context-menu-wrapper"
        :style="{ left: rootMenuX + 'px', top: rootMenuY + 'px' }"
      >
        <v-list density="compact" min-width="150" rounded="lg" class="root-context-menu">
          <v-list-item
            prepend-icon="mdi-file-plus"
            title="New File"
            density="compact"
            @click="rootMenu = false; showNewFileDialog = true"
          />
          <v-list-item
            prepend-icon="mdi-folder-plus"
            title="New Folder"
            density="compact"
            @click="rootMenu = false; showMkdirDialog = true"
          />
          <v-divider v-if="fileStore.clipboard" class="my-1" />
          <v-list-item
            v-if="fileStore.clipboard"
            prepend-icon="mdi-content-paste"
            title="Paste"
            density="compact"
            :subtitle="'Paste \'' + fileStore.clipboard.name + '\' here'"
            @click="onRootMenuPaste"
          />
        </v-list>
      </div>

      <input
        ref="fileInputRef"
        type="file"
        multiple
        style="display: none"
        @change="onFilesSelected"
      />
    </div>

    <v-dialog v-model="showNewFileDialog" max-width="400">
      <v-card rounded="lg">
        <v-card-title class="text-body-1">New File</v-card-title>
        <v-card-text>
          <v-text-field
            v-model="newFileName"
            label="File Name"
            variant="outlined"
            density="compact"
            autofocus
            :error-messages="newFileError ? [newFileError] : []"
            @update:model-value="newFileError = ''"
            @keydown.enter="handleNewFile"
          />
        </v-card-text>
        <v-card-actions class="px-4 pb-4">
          <v-spacer />
          <v-btn variant="text" size="small" @click="showNewFileDialog = false; newFileError = ''">Cancel</v-btn>
          <v-btn color="primary" variant="tonal" size="small" @click="handleNewFile" :loading="newFileLoading">Create</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-dialog v-model="showMkdirDialog" max-width="400">
      <v-card rounded="lg">
        <v-card-title class="text-body-1">New Folder</v-card-title>
        <v-card-text>
          <v-text-field
            v-model="mkdirName"
            label="Folder Name"
            variant="outlined"
            density="compact"
            autofocus
            :error-messages="mkdirError ? [mkdirError] : []"
            @update:model-value="mkdirError = ''"
            @keydown.enter="handleMkdir"
          />
        </v-card-text>
        <v-card-actions class="px-4 pb-4">
          <v-spacer />
          <v-btn variant="text" size="small" @click="showMkdirDialog = false; mkdirError = ''">Cancel</v-btn>
          <v-btn color="primary" variant="tonal" size="small" @click="handleMkdir" :loading="mkdirLoading">Create</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-dialog v-model="renameDialog" max-width="400">
      <v-card rounded="lg">
        <v-card-title class="text-body-1">Rename</v-card-title>
        <v-card-text>
          <v-text-field
            v-model="renameName"
            label="New Name"
            variant="outlined"
            density="compact"
            autofocus
            @keydown.enter="handleRename"
          />
        </v-card-text>
        <v-card-actions class="px-4 pb-4">
          <v-spacer />
          <v-btn variant="text" size="small" @click="renameDialog = false">Cancel</v-btn>
          <v-btn color="primary" variant="tonal" size="small" @click="handleRename" :loading="renameLoading">Save</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-dialog v-model="deleteDialog" max-width="400">
      <v-card rounded="lg">
        <v-card-title class="text-body-1">Delete</v-card-title>
        <v-card-text>
          <p class="mb-2">
            Are you sure you want to delete
            <strong>{{ deleteTargetName }}</strong>?
          </p>
          <p class="text-caption text-error">This action cannot be undone.</p>
        </v-card-text>
        <v-card-actions class="px-4 pb-4">
          <v-spacer />
          <v-btn variant="text" size="small" @click="deleteDialog = false">Cancel</v-btn>
          <v-btn color="error" variant="tonal" size="small" @click="handleDelete" :loading="deleteLoading">Delete</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-dialog v-model="showPasteRenameDialog" max-width="400" persistent @keydown.enter="handlePasteRename">
      <v-card rounded="lg">
        <v-card-title class="text-body-1">Name Conflict</v-card-title>
        <v-card-text>
          <p class="text-caption text-medium-emphasis mb-2">
            A file or folder with the same name already exists in the target location.
          </p>
          <v-text-field
            v-model="pasteRenameName"
            label="New Name"
            variant="outlined"
            density="compact"
            autofocus
          />
        </v-card-text>
        <v-card-actions class="px-4 pb-4">
          <v-spacer />
          <v-btn variant="text" size="small" @click="showPasteRenameDialog = false">Cancel</v-btn>
          <v-btn color="primary" variant="tonal" size="small" @click="handlePasteRename" :loading="pasteRenameLoading">Paste</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

  </v-navigation-drawer>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useWindowSize, onClickOutside } from '@vueuse/core'
import { useFileStore } from '@/stores/files'
import { useAppStore } from '@/stores/app'
import FileTreeItem from '@/components/FileTreeItem.vue'
const fileStore = useFileStore()
const appStore = useAppStore()
const route = useRoute()
const router = useRouter()

const { width: windowWidth } = useWindowSize()

const MIN_THREE_COL_WIDTH = 920

const isFloating = computed(() => windowWidth.value < MIN_THREE_COL_WIDTH)

const drawerProps = computed(() => {
  if (isFloating.value) {
    return {
      temporary: true,
      modelValue: !appStore.rightSidebarCollapsed,
      'onUpdate:modelValue': (val: boolean) => {
        if (val !== !appStore.rightSidebarCollapsed) {
          appStore.toggleRightSidebar()
        }
      },
    }
  }
  return {
    permanent: true,
    rail: appStore.rightSidebarCollapsed,
  }
})

const fileInputRef = ref<HTMLInputElement | null>(null)
const fileTreeWrapperRef = ref<HTMLElement | null>(null)
const sidebarContentRef = ref<HTMLElement | null>(null)
const rootMenuRef = ref<HTMLElement | null>(null)

const showNewFileDialog = ref(false)
const newFileName = ref('')
const newFileLoading = ref(false)
const newFileError = ref('')

const showMkdirDialog = ref(false)
const mkdirName = ref('')
const mkdirLoading = ref(false)
const mkdirError = ref('')

const renameDialog = ref(false)
const renamePath = ref('')
const renameName = ref('')
const renameLoading = ref(false)

const deleteDialog = ref(false)
const deletePath = ref('')
const deleteTargetName = ref('')
const deleteLoading = ref(false)

const showPasteRenameDialog = ref(false)
const pasteRenameName = ref('')
const pasteRenameTargetDir = ref('')
const pasteRenameLoading = ref(false)

const rootMenu = ref(false)
const rootMenuX = ref(0)
const rootMenuY = ref(0)

const dialogContextPath = ref<string | null>(null)

onClickOutside(rootMenuRef, () => {
  rootMenu.value = false
})

const selectedFilePath = computed(() => fileStore.selectedFile)

const currentPath = computed(() => (route.query.path as string) || '')

function handleNavigate(folderPath: string) {
  fileStore.setCurrentPath(folderPath)
  router.push({ query: { path: folderPath } })
}

function onTreeContextMenu(e: MouseEvent) {
  const target = e.target as HTMLElement
  if (target.closest('.tree-node')) return
  const container = sidebarContentRef.value
  if (!container) return
  const rect = container.getBoundingClientRect()
  rootMenuX.value = e.clientX - rect.left
  rootMenuY.value = e.clientY - rect.top
  dialogContextPath.value = ''
  rootMenu.value = true
}

async function handleRootPaste(targetDir = '') {
  if (!fileStore.clipboard) return
  try {
    await fileStore.pasteFile(targetDir)
    appStore.showMessage('Pasted successfully', 'success')
  } catch (e: unknown) {
    appStore.showMessage((e as Error).message || 'Failed to paste', 'error')
    pasteRenameName.value = fileStore.clipboard?.name || 'file'
    pasteRenameTargetDir.value = targetDir
    showPasteRenameDialog.value = true
  }
}

async function handlePasteRename() {
  const newName = pasteRenameName.value.trim()
  if (!newName) return
  pasteRenameLoading.value = true
  try {
    await fileStore.pasteFile(pasteRenameTargetDir.value, newName)
    appStore.showMessage('Pasted successfully', 'success')
    showPasteRenameDialog.value = false
  } catch (e: unknown) {
    appStore.showMessage((e as Error).message || 'Failed to paste', 'error')
  } finally {
    pasteRenameLoading.value = false
  }
}

async function onRootMenuPaste() {
  rootMenu.value = false
  await handleRootPaste()
}

function handleSelectFile(filePath: string) {
  fileStore.openFile(filePath)
  const parentPath = filePath.substring(0, filePath.lastIndexOf('/'))
  fileStore.setCurrentPath(parentPath)
  router.push({ path: '/', query: parentPath ? { path: parentPath } : {} })
}

async function handleNewFile() {
  const name = newFileName.value.trim()
  if (!name) return
  newFileLoading.value = true
  try {
    const parentPath = dialogContextPath.value !== null ? dialogContextPath.value : currentPath.value
    const filePath = await fileStore.createFile(parentPath, name)
    appStore.showMessage('File created', 'success')
    showNewFileDialog.value = false
    newFileName.value = ''
    dialogContextPath.value = null
    fileStore.openFile(filePath)
  } catch (e: unknown) {
    const errMsg = (e as Error).message || 'Failed to create file'
    newFileError.value = errMsg
    appStore.showMessage(errMsg, 'error')
  } finally {
    newFileLoading.value = false
  }
}

async function handleMkdir() {
  const name = mkdirName.value.trim()
  if (!name) return
  mkdirLoading.value = true
  try {
    const parentPath = dialogContextPath.value !== null ? dialogContextPath.value : currentPath.value
    await fileStore.createDirectory(parentPath, name)
    appStore.showMessage('Folder created', 'success')
    showMkdirDialog.value = false
    mkdirName.value = ''
    const newFolderPath = parentPath ? parentPath + '/' + name : name
    dialogContextPath.value = null
    handleNavigate(newFolderPath)
  } catch (e: unknown) {
    const errMsg = (e as Error).message || 'Failed to create folder'
    mkdirError.value = errMsg
    appStore.showMessage(errMsg, 'error')
  } finally {
    mkdirLoading.value = false
  }
}

function openRename(path: string, name: string) {
  renamePath.value = path
  renameName.value = name
  renameDialog.value = true
}

async function handleRename() {
  if (!renamePath.value || !renameName.value.trim()) return
  renameLoading.value = true
  try {
    await fileStore.renameItem(renamePath.value, renameName.value.trim())
    appStore.showMessage('Renamed successfully', 'success')
    renameDialog.value = false
    renamePath.value = ''
  } catch {
    appStore.showMessage('Failed to rename', 'error')
  } finally {
    renameLoading.value = false
  }
}

function confirmDelete(path: string, name: string, _isDir: boolean) {
  deletePath.value = path
  deleteTargetName.value = name
  deleteDialog.value = true
}

async function handleDelete() {
  if (!deletePath.value) return
  deleteLoading.value = true
  try {
    const deletedPath = deletePath.value
    const currentRoutePath = currentPath.value
    await fileStore.deleteItem(deletedPath)
    if (currentRoutePath === deletedPath || currentRoutePath.startsWith(deletedPath + '/')) {
      const parentPath = deletedPath.substring(0, deletedPath.lastIndexOf('/'))
      router.push({ query: { path: parentPath } })
    }
    appStore.showMessage('Deleted successfully', 'success')
    deleteDialog.value = false
    deletePath.value = ''
  } catch {
    appStore.showMessage('Failed to delete', 'error')
  } finally {
    deleteLoading.value = false
  }
}

function onFilesSelected(e: Event) {
  const input = e.target as HTMLInputElement
  const files = input.files
  if (!files || !files.length) return
  fileStore.uploadFiles('', Array.from(files)).then(() => {
    appStore.showMessage('Files uploaded', 'success')
  }).catch(() => {
    appStore.showMessage('Failed to upload files', 'error')
  }).finally(() => {
    input.value = ''
  })
}

onMounted(() => {
  fileStore.fetchTreeRoot()
  fileStore.setupFileWatcher()
})

onUnmounted(() => {
  fileStore.teardownFileWatcher()
})
</script>

<style scoped>
.right-sidebar {
  border-left: 1px solid rgba(var(--v-theme-on-surface), 0.06);
}

.right-sidebar :deep(.v-navigation-drawer__content) {
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.sidebar-content {
  display: flex;
  flex-direction: column;
  flex: 1;
  min-height: 0;
  position: relative;
}

.project-bar {
  min-height: 36px;
  display: flex;
  align-items: center;
  background: rgba(var(--v-theme-surface-variant), 0.3);
  border-bottom: 1px solid rgba(var(--v-theme-on-surface), 0.08);
  user-select: none;
}

.project-name {
  font-size: 21px;
  font-weight: 500;
  letter-spacing: -0.3px;
}

.collapse-btn {
  opacity: 0.5;
  transition: opacity 0.15s ease;
}

.collapse-btn:hover {
  opacity: 1;
}

.toolbar {
  min-height: 36px;
  user-select: none;
  background: rgba(var(--v-theme-surface-variant), 0.15);
  border-bottom: 1px solid rgba(var(--v-theme-on-surface), 0.06);
}

.toolbar-btn {
  opacity: 0.6;
  transition: opacity 0.15s ease, color 0.15s ease;
}

.toolbar-btn:hover {
  opacity: 1;
  color: rgb(var(--v-theme-primary));
}

.clipboard-bar {
  min-height: 28px;
  user-select: none;
  background: rgba(var(--v-theme-primary), 0.08);
  border-bottom: 1px solid rgba(var(--v-theme-primary), 0.12);
}

.clipboard-icon {
  color: rgb(var(--v-theme-primary));
}

.clipboard-text {
  color: rgb(var(--v-theme-primary));
}

.file-tree-wrapper {
  flex: 1;
  overflow-y: auto;
  min-height: 0;
  position: relative;
}

.file-tree-wrapper::-webkit-scrollbar {
  width: 6px;
}

.file-tree-wrapper::-webkit-scrollbar-track {
  background: transparent;
}

.file-tree-wrapper::-webkit-scrollbar-thumb {
  background: rgba(var(--v-theme-on-surface), 0.12);
  border-radius: 3px;
}

.file-tree-wrapper::-webkit-scrollbar-thumb:hover {
  background: rgba(var(--v-theme-on-surface), 0.2);
}

.root-context-menu-wrapper {
  position: absolute;
  z-index: 100;
}

.root-context-menu {
  border: 1px solid rgba(var(--v-theme-on-surface), 0.06);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
}
</style>
