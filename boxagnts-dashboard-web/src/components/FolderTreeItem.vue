<template>
  <template v-for="node in nodes" :key="node.path">
    <template v-if="node.children.length > 0">
      <v-list-item
        :title="node.name"
        prepend-icon="mdi-folder"
        :active="currentPath === node.path"
        :color="currentPath === node.path ? 'primary' : undefined"
        density="compact"
        @click="$emit('select', node.path)"
      >
        <template #append>
          <v-icon
            size="20"
            class="folder-expand-icon"
            @click.stop="toggleExpand(node.path)"
          >
            {{ isExpanded(node.path) ? 'mdi-chevron-up' : 'mdi-chevron-down' }}
          </v-icon>
        </template>
      </v-list-item>
      <v-expand-transition>
        <v-list v-show="isExpanded(node.path)" density="compact" class="pl-4">
          <FolderTreeItem
            :nodes="node.children"
            :current-path="currentPath"
            :expanded-paths="expandedPaths"
            @select="(p: string) => $emit('select', p)"
            @toggle="(p: string) => $emit('toggle', p)"
          />
        </v-list>
      </v-expand-transition>
    </template>
    <v-list-item
      v-else
      :title="node.name"
      prepend-icon="mdi-folder-outline"
      :active="currentPath === node.path"
      :color="currentPath === node.path ? 'primary' : undefined"
      density="compact"
      @click="$emit('select', node.path)"
    />
  </template>
</template>

<script setup lang="ts">
const props = defineProps<{
  nodes: { name: string; path: string; children: { name: string; path: string; children: any[] }[] }[]
  currentPath: string
  expandedPaths: Set<string>
}>()

const emit = defineEmits<{
  select: [path: string]
  toggle: [path: string]
}>()

function isExpanded(path: string): boolean {
  return props.expandedPaths.has(path)
}

function toggleExpand(path: string) {
  emit('toggle', path)
}
</script>

<style scoped>
.folder-expand-icon {
  cursor: pointer;
  border-radius: 4px;
  padding: 2px;
  transition: background-color 0.15s;
}
.folder-expand-icon:hover {
  background-color: rgba(var(--v-theme-on-surface), 0.08);
}
</style>
