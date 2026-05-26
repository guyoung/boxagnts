<template>
  <div class="home-page">
    <div class="main-tab-bar d-flex align-center ga-1 mb-4" style="min-height: 36px;">
      <v-btn
        :variant="mainTab === 'chat' ? 'tonal' : 'text'"
        :color="mainTab === 'chat' ? 'primary' : 'medium-emphasis'"
        icon="mdi-message-text"
        size="small"
        title="Chat"
        @click="mainTab = 'chat'"
      />
      <v-btn
        :variant="mainTab === 'files' ? 'tonal' : 'text'"
        :color="mainTab === 'files' ? 'primary' : 'medium-emphasis'"
        icon="mdi-folder-outline"
        size="small"
        title="Files"
        @click="mainTab = 'files'"
      />
    </div>

    <div v-show="mainTab === 'chat'" class="tab-content">
      <ChatPage />
    </div>

    <div v-show="mainTab === 'files'" class="tab-content">
      <div v-if="fileStore.openTabs.length === 0" class="empty-files-state">
        <v-icon size="64" color="medium-emphasis" class="mb-3">mdi-file-outline</v-icon>
        <p class="text-h6 text-medium-emphasis">No open files</p>
        <p class="text-body-2 text-medium-emphasis mt-1">Click a file in the file tree to open it here</p>
      </div>

      <div v-else class="file-view-container">
        <v-tabs
          v-model="activeTabIndex"
          density="compact"
          color="primary"
          class="file-tabs mb-2"
          show-arrows
        >
          <v-tab
            v-for="(tab, idx) in fileStore.openTabs"
            :key="tab.path"
            :value="idx"
            class="file-tab"
          >
            <v-icon size="14" class="mr-1" color="medium-emphasis">
              {{ fileIcon(tab.name) }}
            </v-icon>
            <span class="text-body-2 text-truncate file-tab-label">{{ tab.name }}</span>
            <span v-if="tab.dirty" class="ml-1 text-warning">●</span>
            <v-btn
              icon="mdi-close"
              variant="text"
              size="x-small"
              density="compact"
              color="medium-emphasis"
              class="ml-1"
              @click.stop="closeTab(idx)"
            />
          </v-tab>
        </v-tabs>

        <div class="file-content">
          <div
            v-for="tab in fileStore.openTabs"
            :key="tab.path"
            v-show="fileStore.openTabs.indexOf(tab) === activeTabIndex"
            class="file-content-inner"
          >
            <CodeEditor v-if="!fileStore.isImageFile(tab.path)" />
            <ImagePreview v-else />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import ChatPage from '@/views/ChatPage.vue'
import ChatPage2 from '@/views/ChatPage2.vue'
import CodeEditor from '@/components/CodeEditor.vue'
import ImagePreview from '@/components/ImagePreview.vue'
import { useFileStore } from '@/stores/files'

const fileStore = useFileStore()

const mainTab = ref('chat')
const activeTabIndex = ref(0)

onMounted(() => {
  if (fileStore.openTabs.length > 0) {
    mainTab.value = 'files'
  }
})

const fileTypeIcons: Record<string, string> = {
  '.ts': 'mdi-language-typescript', '.tsx': 'mdi-react',
  '.js': 'mdi-language-javascript', '.jsx': 'mdi-react',
  '.vue': 'mdi-vuejs', '.css': 'mdi-language-css3',
  '.scss': 'mdi-sass', '.html': 'mdi-language-html5',
  '.json': 'mdi-code-json', '.md': 'mdi-language-markdown',
  '.py': 'mdi-language-python', '.rs': 'mdi-language-rust',
  '.go': 'mdi-language-go', '.java': 'mdi-language-java',
  '.yml': 'mdi-cog', '.yaml': 'mdi-cog',
  '.png': 'mdi-file-image', '.jpg': 'mdi-file-image',
  '.jpeg': 'mdi-file-image', '.gif': 'mdi-file-image',
  '.svg': 'mdi-file-image', '.pdf': 'mdi-file-pdf-box',
  '.zip': 'mdi-folder-zip',
}

function fileIcon(name: string): string {
  const ext = name.lastIndexOf('.') >= 0 ? name.slice(name.lastIndexOf('.')).toLowerCase() : ''
  return fileTypeIcons[ext] || 'mdi-file-outline'
}

function closeTab(idx: number) {
  const tab = fileStore.openTabs[idx]
  if (!tab) return
  fileStore.closeOpenFile(tab.path)
  if (fileStore.activeTabPath) {
    fileStore.selectFile(fileStore.activeTabPath)
  } else {
    fileStore.selectFile(null)
  }
  if (activeTabIndex.value >= fileStore.openTabs.length) {
    activeTabIndex.value = Math.max(0, fileStore.openTabs.length - 1)
  }
}

watch(() => fileStore.activeTabPath, (newPath) => {
  if (newPath) {
    fileStore.selectFile(newPath)
    const idx = fileStore.openTabs.findIndex(t => t.path === newPath)
    if (idx >= 0) {
      activeTabIndex.value = idx
    }
  }
})

watch(() => fileStore.selectedFile, (newVal) => {
  if (newVal === null && fileStore.openTabs.length > 0) {
    const idx = fileStore.openTabs.findIndex(t => t.path === fileStore.activeTabPath)
    if (idx >= 0) {
      closeTab(idx)
    }
  }
})

watch(() => fileStore.openTabs.length, (len) => {
  if (len > 0) {
    mainTab.value = 'files'
    if (fileStore.activeTabPath) {
      const idx = fileStore.openTabs.findIndex(t => t.path === fileStore.activeTabPath)
      if (idx >= 0) activeTabIndex.value = idx
    }
  }
})
</script>

<style scoped>
.home-page {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.main-tab-bar {
  flex-shrink: 0;
}

.tab-content {
  flex: 1;
  min-height: 0;
  overflow: hidden;
}

.empty-files-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 64px 16px;
}

.file-tabs {
  flex-shrink: 0;
}

.file-tab {
  min-width: 120px;
  max-width: 200px;
}

.file-tab-label {
  max-width: 100px;
}

.file-content {
  flex: 1;
  min-height: 0;
  overflow: hidden;
}

.file-view-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  min-height: 0;
  overflow: hidden;
}

.file-content-inner {
  height: 100%;
  overflow: hidden;
}

/***
:deep(.chat-layout) {
  height: 100%;
}
***/
</style>
