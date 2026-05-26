<template>
  <div class="tree-node">
    <v-menu
      location="start"
      :close-on-content-click="true"
      offset="4"
    >
      <template #activator="{ props: menuProps }">
        <v-list-item
          :active="active || fileActive"
          density="compact"
          rounded="lg"
          class="mb-0 tree-item"
          :class="{ 'tree-item--cut': isCut }"
          :style="{ paddingLeft: (depth * 16 + 4) + 'px' }"
          v-bind="menuProps"
          @click="onClick"
          @contextmenu.prevent
        >
          <template #prepend>
            <v-progress-circular
              v-if="node.loading"
              indeterminate
              size="14"
              width="2"
              color="primary"
            />
            <v-icon v-else size="16" :color="node.is_dir ? 'warning' : 'medium-emphasis'">
              {{ node.is_dir ? (node.expanded ? 'mdi-folder-open' : 'mdi-folder') : fileIcon(node.name) }}
            </v-icon>
          </template>
          <v-list-item-title class="text-body-2 text-truncate">
            {{ node.name }}
          </v-list-item-title>
          <template #append v-if="node.is_dir">
            <v-icon size="14" color="medium-emphasis">
              {{ node.expanded ? 'mdi-chevron-down' : 'mdi-chevron-right' }}
            </v-icon>
          </template>
        </v-list-item>
      </template>

      <v-list density="compact" min-width="160" rounded="lg" class="context-menu">
        <v-list-item
          prepend-icon="mdi-content-copy"
          title="Copy"
          density="compact"
          @click="handleCopy"
        />
        <v-list-item
          prepend-icon="mdi-content-cut"
          title="Cut"
          density="compact"
          @click="handleCut"
        />
        <v-list-item
          v-if="fileStore.clipboard && node.is_dir"
          prepend-icon="mdi-content-paste"
          title="Paste"
          density="compact"
          :subtitle="'Paste \'' + fileStore.clipboard.name + '\' here'"
          @click="handlePaste"
        />
        <v-divider class="my-1" />
        <v-list-item
          v-if="!node.is_dir"
          prepend-icon="mdi-download"
          title="Download"
          density="compact"
          @click="handleDownload"
        />
        <v-list-item
          prepend-icon="mdi-pencil"
          title="Rename"
          density="compact"
          @click="$emit('rename', node.path, node.name)"
        />
        <v-divider class="my-1" />
        <v-list-item
          prepend-icon="mdi-delete"
          title="Delete"
          density="compact"
          class="text-error"
          @click="$emit('delete', node.path, node.name, node.is_dir)"
        />
      </v-list>
    </v-menu>

    <v-expand-transition v-if="node.is_dir">
      <div v-show="node.expanded">
        <FileTreeItem
          v-for="child in node.children"
          :key="child.path"
          :node="child"
          :depth="depth + 1"
          :current-path="currentPath"
          :selected-file-path="selectedFilePath"
          @navigate="(p: string) => $emit('navigate', p)"
          @select-file="(p: string) => $emit('selectFile', p)"
          @rename="(p: string, n: string) => $emit('rename', p, n)"
          @delete="(p: string, n: string, d: boolean) => $emit('delete', p, n, d)"
        />
        <div
          v-if="node.loaded && node.children.length === 0"
          class="text-caption text-medium-emphasis px-4 py-1"
          :style="{ paddingLeft: ((depth + 1) * 16 + 28) + 'px' }"
        >
          Empty
        </div>
      </div>
    </v-expand-transition>
  </div>

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
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import type { TreeNode } from '@/stores/files'
import { useFileStore } from '@/stores/files'
import { useAppStore } from '@/stores/app'

const props = defineProps<{
  node: TreeNode
  depth: number
  currentPath: string
  selectedFilePath: string | null
}>()

const emit = defineEmits<{
  navigate: [path: string]
  selectFile: [path: string]
  rename: [path: string, name: string]
  delete: [path: string, name: string, isDir: boolean]
}>()

const fileStore = useFileStore()
const appStore = useAppStore()

const active = computed(() => props.currentPath === props.node.path)
const fileActive = computed(() => !props.node.is_dir && props.selectedFilePath === props.node.path)

const isCut = computed(() =>
  fileStore.clipboard?.mode === 'cut' && fileStore.clipboard?.path === props.node.path
)

const showPasteRenameDialog = ref(false)
const pasteRenameName = ref('')
const pasteRenameLoading = ref(false)

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

function fileIcon(name: string): string {
  const ext = name.lastIndexOf('.') >= 0 ? name.slice(name.lastIndexOf('.')).toLowerCase() : ''
  return fileTypeIcons[ext] || 'mdi-file-outline'
}

function onClick() {
  if (props.node.is_dir) {
    fileStore.onTreeItemClick(props.node)
    emit('navigate', props.node.path)
  } else {
    fileStore.selectFile(props.node.path)
    emit('selectFile', props.node.path)
  }
}

function handleCopy() {
  fileStore.setClipboard({
    path: props.node.path,
    name: props.node.name,
    is_dir: props.node.is_dir,
    mode: 'copy',
  })
  appStore.showMessage('Copied to clipboard', 'success')
}

function handleCut() {
  fileStore.setClipboard({
    path: props.node.path,
    name: props.node.name,
    is_dir: props.node.is_dir,
    mode: 'cut',
  })
  appStore.showMessage('Cut to clipboard', 'success')
}

async function handlePaste() {
  if (!fileStore.clipboard) return
  try {
    await fileStore.pasteFile(props.node.path)
    appStore.showMessage('Pasted successfully', 'success')
  } catch (e: unknown) {
    appStore.showMessage((e as Error).message || 'Failed to paste', 'error')
    pasteRenameName.value = fileStore.clipboard?.name || 'file'
    showPasteRenameDialog.value = true
  }
}

async function handlePasteRename() {
  const newName = pasteRenameName.value.trim()
  if (!newName) return
  pasteRenameLoading.value = true
  try {
    await fileStore.pasteFile(props.node.path, newName)
    appStore.showMessage('Pasted successfully', 'success')
    showPasteRenameDialog.value = false
  } catch (e: unknown) {
    appStore.showMessage((e as Error).message || 'Failed to paste', 'error')
  } finally {
    pasteRenameLoading.value = false
  }
}

function handleDownload() {
  fileStore.downloadItem(props.node.path).catch(() => {
    appStore.showMessage('Failed to download', 'error')
  })
}
</script>

<style scoped>
.tree-item {
  transition: background-color 0.1s ease;
  border-radius: 6px;
}

.tree-item:hover {
  background: rgba(var(--v-theme-on-surface), 0.04);
}

.tree-item--cut {
  opacity: 0.45;
}

.context-menu {
  border: 1px solid rgba(var(--v-theme-on-surface), 0.06);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
}
</style>
