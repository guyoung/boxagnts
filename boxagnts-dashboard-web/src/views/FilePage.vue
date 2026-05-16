<template>
  <div>
    <div class="d-flex align-center justify-space-between mb-6">
      <div class="d-flex align-center">
        <v-icon size="32" color="primary" class="mr-3">mdi-folder-outline</v-icon>
        <h1 class="text-h4 font-weight-bold">Files</h1>
      </div>
      <div class="d-flex align-center gap-3">
        <v-btn
          v-if="selectedPaths.size > 0"
          color="error"
          variant="tonal"
          prepend-icon="mdi-delete"
          @click="batchDeleteDialog = true"
        >
          Delete ({{ selectedPaths.size }})
        </v-btn>
        <v-menu location="bottom end" :close-on-content-click="false">
          <template #activator="{ props: menuProps }">
            <v-btn
              color="primary"
              prepend-icon="mdi-plus"
              v-bind="menuProps"
            >
              New
            </v-btn>
          </template>
          <v-list density="compact" min-width="160">
            <v-list-item prepend-icon="mdi-file-plus" title="New File" @click="showNewFileDialog = true" />
            <v-list-item prepend-icon="mdi-folder-plus" title="New Folder" @click="showMkdirDialog = true" />
            <v-divider />
            <v-list-item prepend-icon="mdi-upload" title="Upload" @click="triggerUpload" />
          </v-list>
        </v-menu>
        <input
          ref="fileInputRef"
          type="file"
          multiple
          style="display: none"
          @change="onFilesSelected"
        />
      </div>
    </div>

    <v-card class="mb-4" variant="flat" color="surface-variant">
      <v-card-text class="pa-2">
        <v-breadcrumbs density="compact" class="py-0" divider="&gt;">
          <v-breadcrumbs-item
            v-for="(part, idx) in breadcrumbs"
            :key="idx"
            :disabled="idx === breadcrumbs.length - 1"
            @click="idx < breadcrumbs.length - 1 && navigateBreadcrumb(idx)"
          >
            <v-icon v-if="idx === 0" size="16" class="mr-1">mdi-home</v-icon>
            {{ part.label }}
          </v-breadcrumbs-item>
        </v-breadcrumbs>
      </v-card-text>
    </v-card>

    <CodeEditor v-if="fileStore.selectedFile && !isImageFile" />
    <ImagePreview v-else-if="fileStore.selectedFile && isImageFile" />

    <v-card v-else>
      <v-card-text class="pa-0">
        <div v-if="fileStore.loading" class="text-center pa-8">
          <v-progress-circular indeterminate size="32" width="3" color="primary" />
        </div>

        <v-list v-else lines="one" density="comfortable">
          <v-list-item
            v-for="item in fileStore.items"
            :key="item.path"
            rounded="lg"
            class="mb-1"
            :active="isSelected(item.path)"
            @click="handleFileClick(item)"
          >
            <template #prepend>
              <v-checkbox
                :model-value="isSelected(item.path)"
                density="compact"
                hide-details
                color="primary"
                class="mr-1"
                @click.stop
                @update:model-value="(v: boolean | null) => toggleSelect(item.path, !!v)"
              />
              <v-icon :color="item.is_dir ? 'warning' : 'medium-emphasis'">
                {{ item.is_dir ? 'mdi-folder' : fileIcon(item.name) }}
              </v-icon>
            </template>

            <v-list-item-title class="text-body-2">
              {{ item.name }}
            </v-list-item-title>

            <v-list-item-subtitle v-if="!item.is_dir">
              <span class="text-caption">{{ formatSize(item.size) }}</span>
              <span v-if="item.modified" class="text-caption ml-2">{{ item.modified }}</span>
            </v-list-item-subtitle>

            <template #append>
              <v-menu location="bottom end" :close-on-content-click="true">
                <template #activator="{ props: menuProps }">
                  <v-btn
                    icon="mdi-dots-vertical"
                    variant="text"
                    size="x-small"
                    color="medium-emphasis"
                    v-bind="menuProps"
                    @click.stop
                  />
                </template>
                <v-list density="compact" min-width="140">
                  <v-list-item
                    prepend-icon="mdi-pencil"
                    title="Rename"
                    @click.stop="openRename(item)"
                  />
                  <v-list-item
                    v-if="!item.is_dir"
                    prepend-icon="mdi-download"
                    title="Download"
                    @click.stop="handleDownload(item)"
                  />
                  <v-list-item
                    prepend-icon="mdi-delete"
                    title="Delete"
                    @click.stop="confirmDelete(item)"
                  />
                </v-list>
              </v-menu>
            </template>
          </v-list-item>

          <div v-if="fileStore.items.length === 0" class="text-center pa-8">
            <v-icon size="48" color="medium-emphasis">mdi-folder-open-outline</v-icon>
            <p class="text-medium-emphasis mt-2">This directory is empty</p>
          </div>
        </v-list>
      </v-card-text>
    </v-card>

    <!-- New File Dialog -->
    <v-dialog v-model="showNewFileDialog" max-width="400">
      <v-card>
        <v-card-title>New File</v-card-title>
        <v-card-text>
          <v-text-field
            v-model="newFileName"
            label="File Name"
            variant="outlined"
            autofocus
            @keydown.enter="handleNewFile"
          />
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="showNewFileDialog = false">Cancel</v-btn>
          <v-btn color="primary" @click="handleNewFile" :loading="newFileLoading">Create</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Create Directory Dialog -->
    <v-dialog v-model="showMkdirDialog" max-width="400">
      <v-card>
        <v-card-title>New Folder</v-card-title>
        <v-card-text>
          <v-text-field
            v-model="mkdirName"
            label="Folder Name"
            variant="outlined"
            autofocus
            @keydown.enter="handleMkdir"
          />
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="showMkdirDialog = false">Cancel</v-btn>
          <v-btn color="primary" @click="handleMkdir" :loading="mkdirLoading">Create</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Rename Dialog -->
    <v-dialog v-model="renameDialog" max-width="400">
      <v-card>
        <v-card-title>Rename</v-card-title>
        <v-card-text>
          <v-text-field
            v-model="renameName"
            label="New Name"
            variant="outlined"
            autofocus
            @keydown.enter="handleRename"
          />
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="renameDialog = false">Cancel</v-btn>
          <v-btn color="primary" @click="handleRename" :loading="renameLoading">Save</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Delete Confirmation Dialog -->
    <v-dialog v-model="deleteDialog" max-width="400">
      <v-card>
        <v-card-title>Delete</v-card-title>
        <v-card-text>
          <p>
            Are you sure you want to delete
            <strong>{{ deleteTarget?.name }}</strong
            >?
          </p>
          <p class="text-caption text-error mt-2">This action cannot be undone.</p>
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="deleteDialog = false">Cancel</v-btn>
          <v-btn color="error" @click="handleDelete" :loading="deleteLoading">Delete</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Batch Delete Dialog -->
    <v-dialog v-model="batchDeleteDialog" max-width="400">
      <v-card>
        <v-card-title>Delete Selected</v-card-title>
        <v-card-text>
          <p>
            Are you sure you want to delete
            <strong>{{ selectedPaths.size }} item{{ selectedPaths.size > 1 ? 's' : '' }}</strong>?
          </p>
          <p class="text-caption text-error mt-2">This action cannot be undone.</p>
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="batchDeleteDialog = false">Cancel</v-btn>
          <v-btn color="error" @click="handleBatchDelete" :loading="batchDeleteLoading">Delete All</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useFileStore } from '@/stores/files'
import { useAppStore } from '@/stores/app'
import CodeEditor from '@/components/CodeEditor.vue'
import ImagePreview from '@/components/ImagePreview.vue'
import type { FileItem } from '@/api/fileApi'

const fileStore = useFileStore()
const appStore = useAppStore()
const route = useRoute()
const router = useRouter()

const fileInputRef = ref<HTMLInputElement | null>(null)

const showMkdirDialog = ref(false)
const mkdirName = ref('')
const mkdirLoading = ref(false)

const showNewFileDialog = ref(false)
const newFileName = ref('')
const newFileLoading = ref(false)

const renameDialog = ref(false)
const renameTarget = ref<FileItem | null>(null)
const renameName = ref('')
const renameLoading = ref(false)

const deleteDialog = ref(false)
const deleteTarget = ref<FileItem | null>(null)
const deleteLoading = ref(false)

const selectedPaths = ref(new Set<string>())
const batchDeleteDialog = ref(false)
const batchDeleteLoading = ref(false)

const fileTypeIcons: Record<string, string> = {
  '.ts': 'mdi-language-typescript',
  '.tsx': 'mdi-react',
  '.js': 'mdi-language-javascript',
  '.jsx': 'mdi-react',
  '.vue': 'mdi-vuejs',
  '.css': 'mdi-language-css3',
  '.scss': 'mdi-sass',
  '.html': 'mdi-language-html5',
  '.json': 'mdi-code-json',
  '.md': 'mdi-language-markdown',
  '.py': 'mdi-language-python',
  '.rs': 'mdi-language-rust',
  '.go': 'mdi-language-go',
  '.java': 'mdi-language-java',
  '.yml': 'mdi-cog',
  '.yaml': 'mdi-cog',
  '.toml': 'mdi-cog',
  '.lock': 'mdi-lock',
  '.gitignore': 'mdi-git',
  '.png': 'mdi-file-image',
  '.jpg': 'mdi-file-image',
  '.jpeg': 'mdi-file-image',
  '.gif': 'mdi-file-image',
  '.svg': 'mdi-file-image',
  '.pdf': 'mdi-file-pdf-box',
  '.zip': 'mdi-folder-zip',
  '.tar': 'mdi-folder-zip',
  '.gz': 'mdi-folder-zip',
}

const currentPath = computed(() => (route.query.path as string) || '')

const selectedFileExt = computed(() => {
  const path = fileStore.selectedFile
  if (!path) return ''
  const dot = path.lastIndexOf('.')
  return dot >= 0 ? path.slice(dot).toLowerCase() : ''
})

const isImageFile = computed(() => {
  const imageExts = ['.png', '.jpg', '.jpeg', '.gif', '.svg', '.webp', '.bmp', '.ico']
  return imageExts.includes(selectedFileExt.value)
})

function handleFileClick(item: FileItem) {
  if (item.is_dir) {
    fileStore.clearSelectedFile()
    navigateInto(item.path)
  } else {
    fileStore.selectFile(item.path)
  }
}

watch(currentPath, (p) => {
  fileStore.setCurrentPath(p)
  fileStore.fetchCurrentItems()
  selectedPaths.value = new Set()
}, { immediate: true })

const breadcrumbs = computed(() => {
  const parts: { label: string; path: string }[] = [{ label: 'root', path: '' }]
  if (!currentPath.value) return parts
  const segs = currentPath.value.split('/').filter(Boolean)
  let accumulated = ''
  for (const seg of segs) {
    accumulated += '/' + seg
    parts.push({ label: seg, path: accumulated })
  }
  return parts
})

function navigateBreadcrumb(idx: number) {
  const target = breadcrumbs.value[idx]
  router.push({ query: target.path ? { path: target.path } : {} })
}

function navigateInto(itemPath: string) {
  router.push({ query: { path: itemPath } })
}

function fileIcon(name: string): string {
  const ext = name.lastIndexOf('.') >= 0 ? name.slice(name.lastIndexOf('.')).toLowerCase() : ''
  return fileTypeIcons[ext] || 'mdi-file-outline'
}

function formatSize(bytes: number): string {
  if (bytes === 0) return '0 B'
  const units = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(1024))
  return (bytes / Math.pow(1024, i)).toFixed(i > 0 ? 1 : 0) + ' ' + units[i]
}

function triggerUpload() {
  fileInputRef.value?.click()
}

async function onFilesSelected(e: Event) {
  const input = e.target as HTMLInputElement
  const files = input.files
  if (!files || !files.length) return
  try {
    await fileStore.uploadFiles(Array.from(files))
    appStore.showMessage('Files uploaded', 'success')
  } catch {
    appStore.showMessage('Failed to upload files', 'error')
  } finally {
    input.value = ''
  }
}

async function handleNewFile() {
  const name = newFileName.value.trim()
  if (!name) return
  newFileLoading.value = true
  try {
    const filePath = currentPath.value ? currentPath.value + '/' + name : '/' + name
    await fileStore.saveFileContent(filePath, '')
    appStore.showMessage('File created', 'success')
    showNewFileDialog.value = false
    newFileName.value = ''
    fileStore.selectFile(filePath)
  } catch {
    appStore.showMessage('Failed to create file', 'error')
  } finally {
    newFileLoading.value = false
  }
}

async function handleMkdir() {
  const name = mkdirName.value.trim()
  if (!name) return
  mkdirLoading.value = true
  try {
    await fileStore.createDirectory(name)
    appStore.showMessage('Folder created', 'success')
    showMkdirDialog.value = false
    mkdirName.value = ''
  } catch {
    appStore.showMessage('Failed to create folder', 'error')
  } finally {
    mkdirLoading.value = false
  }
}

function openRename(item: FileItem) {
  renameTarget.value = item
  renameName.value = item.name
  renameDialog.value = true
}

async function handleRename() {
  if (!renameTarget.value || !renameName.value.trim()) return
  renameLoading.value = true
  try {
    await fileStore.renameItem(renameTarget.value.path, renameName.value.trim())
    appStore.showMessage('Renamed successfully', 'success')
    renameDialog.value = false
    renameTarget.value = null
  } catch {
    appStore.showMessage('Failed to rename', 'error')
  } finally {
    renameLoading.value = false
  }
}

function confirmDelete(item: FileItem) {
  deleteTarget.value = item
  deleteDialog.value = true
}

async function handleDelete() {
  if (!deleteTarget.value) return
  deleteLoading.value = true
  try {
    await fileStore.deleteItem(deleteTarget.value.path)
    appStore.showMessage('Deleted successfully', 'success')
    deleteDialog.value = false
    deleteTarget.value = null
  } catch {
    appStore.showMessage('Failed to delete', 'error')
  } finally {
    deleteLoading.value = false
  }
}

async function handleDownload(item: FileItem) {
  try {
    await fileStore.downloadItem(item.path)
    appStore.showMessage('Download started', 'success')
  } catch {
    appStore.showMessage('Failed to download', 'error')
  }
}

function isSelected(path: string): boolean {
  return selectedPaths.value.has(path)
}

function toggleSelect(path: string, selected: boolean) {
  if (selected) {
    selectedPaths.value.add(path)
  } else {
    selectedPaths.value.delete(path)
  }
  selectedPaths.value = new Set(selectedPaths.value)
}

async function handleBatchDelete() {
  batchDeleteLoading.value = true
  let failed = 0
  const paths = [...selectedPaths.value]
  for (const p of paths) {
    try {
      await fileStore.deleteItem(p)
    } catch {
      failed++
    }
  }
  selectedPaths.value = new Set()
  batchDeleteDialog.value = false
  batchDeleteLoading.value = false
  if (failed) {
    appStore.showMessage(`${failed} item${failed > 1 ? 's' : ''} failed to delete`, 'error')
  } else {
    appStore.showMessage(`${paths.length} item${paths.length > 1 ? 's' : ''} deleted`, 'success')
  }
}

onMounted(() => {
  fileStore.refreshTree()
})
</script>
