<template>
  <v-card>
    <v-card-text class="pa-0">
      <v-toolbar density="compact" color="surface">
        <template #prepend>
          <v-icon color="medium-emphasis" class="mr-2">mdi-file-image</v-icon>
          <span class="text-body-2 font-weight-medium text-truncate" style="max-width: 400px">
            {{ fileName }}
          </span>
        </template>
        <v-spacer />
        <v-btn icon="mdi-download" variant="text" size="small" color="medium-emphasis"
          @click="download" title="Download" />
        <v-btn icon="mdi-close" variant="text" size="small" color="medium-emphasis" @click="close" title="Close" />
      </v-toolbar>
      <v-divider />
      <div v-if="loading" class="d-flex align-center justify-center pa-12">
        <v-progress-circular indeterminate size="32" width="3" color="primary" />
      </div>
      <div v-else-if="error" class="d-flex flex-column align-center justify-center py-12">
        <v-icon size="48" color="error">mdi-alert-circle</v-icon>
        <p class="text-medium-emphasis mt-2">{{ error }}</p>
      </div>
      <div v-else class="image-preview-container">
        <img :src="imageSrc" :alt="fileName" class="image-preview" />
      </div>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onBeforeUnmount } from 'vue'
import { useFileStore } from '@/stores/files'
import fileApi from '@/api/fileApi'

const fileStore = useFileStore()

const loading = ref(false)
const error = ref('')
const imageSrc = ref('')
let objectUrl: string | null = null

const fileName = computed(() => {
  const path = fileStore.selectedFile
  if (!path) return ''
  return path.split('/').pop() || path
})

async function loadImage() {
  if (!fileStore.selectedFile) return
  loading.value = true
  error.value = ''
  try {
    const result = await fileApi.downloadFile(fileStore.selectedFile)
    if (objectUrl) {
      URL.revokeObjectURL(objectUrl)
    }
    objectUrl = URL.createObjectURL(result.blob)
    imageSrc.value = objectUrl
  } catch (e) {
    error.value = 'Failed to load image'
    imageSrc.value = ''
  } finally {
    loading.value = false
  }
}

function download() {
  if (!fileStore.selectedFile) return
  fileApi.downloadFile(fileStore.selectedFile).then(result => {
    fileApi.saveBlob(result.blob, result.fileName)
  }).catch(() => {
    error.value = 'Failed to download image'
  })
}

function close() {
  fileStore.clearSelectedFile()
}

function cleanup() {
  if (objectUrl) {
    URL.revokeObjectURL(objectUrl)
    objectUrl = null
  }
  imageSrc.value = ''
}

watch(() => fileStore.selectedFile, (newPath) => {
  if (newPath) {
    loadImage()
  } else {
    cleanup()
  }
})

onMounted(() => {
  if (fileStore.selectedFile) {
    loadImage()
  }
})

onBeforeUnmount(() => {
  cleanup()
})
</script>

<style scoped>
.image-preview-container {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: calc(100vh - 280px);
  max-height: calc(100vh - 200px);
  overflow: auto;
  background:
    repeating-conic-gradient(rgba(var(--v-theme-on-surface), 0.04) 0% 25%, transparent 0% 50%) 50% / 20px 20px;
  padding: 16px;
}

.image-preview {
  max-width: 100%;
  max-height: calc(100vh - 320px);
  object-fit: contain;
  border-radius: 4px;
}
</style>
