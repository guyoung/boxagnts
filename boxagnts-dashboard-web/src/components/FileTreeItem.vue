<template>
  <div class="tree-node">
    <v-list-item
      :active="active || fileActive"
      density="compact"
      rounded="lg"
      class="mb-0 tree-item"
      :style="{ paddingLeft: (depth * 16 + 4) + 'px' }"
      @click="onClick"
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
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { TreeNode } from '@/stores/files'
import { useFileStore } from '@/stores/files'

const props = defineProps<{
  node: TreeNode
  depth: number
  currentPath: string
  selectedFilePath: string | null
}>()

const emit = defineEmits<{
  navigate: [path: string]
  selectFile: [path: string]
}>()

const fileStore = useFileStore()

const active = computed(() => props.currentPath === props.node.path)
const fileActive = computed(() => !props.node.is_dir && props.selectedFilePath === props.node.path)

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
</script>
