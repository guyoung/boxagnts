<template>
  <v-card>
    <v-card-text class="pa-0">
      <v-toolbar density="compact" color="surface">
        <template #prepend>
          <v-icon color="medium-emphasis" class="mr-2">{{ fileTypeIcon }}</v-icon>
          <span class="text-body-2 font-weight-medium text-truncate" style="max-width: 400px">
            {{ fileName }}
          </span>
          <v-chip v-if="language" size="x-small" variant="tonal" class="ml-2">{{ language }}</v-chip>
          <v-chip v-if="dirty" size="x-small" color="warning" variant="tonal" class="ml-2">Unsaved</v-chip>
        </template>
        <v-spacer />
        <v-btn icon="mdi-content-save" variant="text" size="small" color="primary"
          @click="save" :loading="saving" title="Save (Ctrl+S)" />
        <v-btn icon="mdi-close" variant="text" size="small" color="medium-emphasis" @click="tryClose" title="Close" />
      </v-toolbar>
      <v-divider />
      <div v-if="loading" class="d-flex align-center justify-center pa-12">
        <v-progress-circular indeterminate size="32" width="3" color="primary" />
      </div>
      <div v-else class="editor-wrapper">
        <Codemirror
          v-model="code"
          :extensions="extensions"
          :disabled="false"
          :style="{ minHeight: 'calc(100vh - 280px)', maxHeight: 'calc(100vh - 200px)' }"
          @change="onChange"
        />
      </div>
    </v-card-text>

    <v-dialog v-model="unsavedDialog" max-width="400">
      <v-card>
        <v-card-title>Unsaved Changes</v-card-title>
        <v-card-text>
          <p>You have unsaved changes in <strong>{{ fileName }}</strong>.</p>
          <p class="text-caption text-medium-emphasis mt-2">Do you want to save before closing?</p>
        </v-card-text>
        <v-card-actions>
          <v-btn variant="text" @click="discardClose">Discard</v-btn>
          <v-spacer />
          <v-btn variant="text" @click="unsavedDialog = false">Cancel</v-btn>
          <v-btn color="primary" @click="saveAndClose" :loading="saving">Save</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </v-card>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onBeforeUnmount } from 'vue'
import { useFileStore } from '@/stores/files'
import { useAppStore } from '@/stores/app'
import { Codemirror } from 'vue-codemirror'
import { EditorView, keymap } from '@codemirror/view'
import { defaultKeymap, history, historyKeymap, indentWithTab } from '@codemirror/commands'
import { syntaxHighlighting, defaultHighlightStyle, bracketMatching, foldGutter, indentOnInput } from '@codemirror/language'
import { oneDark } from '@codemirror/theme-one-dark'
import { markdown } from '@codemirror/lang-markdown'
import { json } from '@codemirror/lang-json'
import { Compartment } from '@codemirror/state'
import fileApi from '@/api/fileApi'

const fileStore = useFileStore()
const appStore = useAppStore()

const loading = ref(false)
const saving = ref(false)
const code = ref('')
const dirty = ref(false)
const originalContent = ref('')
const unsavedDialog = ref(false)
const languageCompartment = new Compartment()

const fileName = computed(() => {
  const path = fileStore.selectedFile
  if (!path) return ''
  return path.split('/').pop() || path
})

const fileExt = computed(() => {
  const name = fileName.value
  const dot = name.lastIndexOf('.')
  return dot >= 0 ? name.slice(dot).toLowerCase() : ''
})

const language = computed(() => {
  const langMap: Record<string, string> = {
    '.ts': 'TypeScript', '.tsx': 'TSX', '.js': 'JavaScript', '.jsx': 'JSX',
    '.vue': 'Vue', '.css': 'CSS', '.scss': 'SCSS', '.html': 'HTML',
    '.json': 'JSON', '.md': 'Markdown', '.py': 'Python', '.rs': 'Rust',
    '.go': 'Go', '.java': 'Java', '.yml': 'YAML', '.yaml': 'YAML',
    '.toml': 'TOML', '.xml': 'XML', '.sql': 'SQL', '.sh': 'Shell',
    '.c': 'C', '.cpp': 'C++', '.h': 'C Header',
  }
  return langMap[fileExt.value] || ''
})

const fileTypeIcon = computed(() => {
  const iconMap: Record<string, string> = {
    '.ts': 'mdi-language-typescript', '.tsx': 'mdi-react',
    '.js': 'mdi-language-javascript', '.jsx': 'mdi-react',
    '.vue': 'mdi-vuejs', '.css': 'mdi-language-css3', '.scss': 'mdi-sass',
    '.html': 'mdi-language-html5', '.json': 'mdi-code-json',
    '.md': 'mdi-language-markdown', '.py': 'mdi-language-python',
    '.rs': 'mdi-language-rust', '.go': 'mdi-language-go', '.java': 'mdi-language-java',
    '.yml': 'mdi-cog', '.yaml': 'mdi-cog',
  }
  return iconMap[fileExt.value] || 'mdi-file-code-outline'
})

function getLanguageExtension() {
  const ext = fileExt.value
  if (ext === '.json') return json()
  if (ext === '.md') return markdown()
  return []
}

const extensions = computed(() => [
  EditorView.lineWrapping,
  keymap.of([...defaultKeymap, ...historyKeymap, indentWithTab]),
  history(),
  foldGutter(),
  syntaxHighlighting(defaultHighlightStyle, { fallback: true }),
  bracketMatching(),
  indentOnInput(),
  oneDark,
  languageCompartment.of(getLanguageExtension()),
  EditorView.updateListener.of((update) => {
    if (update.docChanged) {
      dirty.value = true
    }
  }),
])

function onChange(value: string) {
  if (value !== originalContent.value) {
    dirty.value = true
  }
}

async function loadFileContent() {
  if (!fileStore.selectedFile) return
  loading.value = true
  try {
    const result = await fileApi.downloadFile(fileStore.selectedFile)
    const text = await result.blob.text()
    code.value = text
    originalContent.value = text
    dirty.value = false
  } catch {
    code.value = ''
    originalContent.value = ''
  } finally {
    loading.value = false
  }
}

async function save() {
  if (!fileStore.selectedFile || !dirty.value) return
  saving.value = true
  try {
    await fileStore.saveFileContent(fileStore.selectedFile, code.value)
    originalContent.value = code.value
    dirty.value = false
    appStore.showMessage('File saved', 'success')
  } catch {
    appStore.showMessage('Failed to save file', 'error')
  } finally {
    saving.value = false
  }
}

function tryClose() {
  if (dirty.value) {
    unsavedDialog.value = true
    return
  }
  fileStore.clearSelectedFile()
}

function discardClose() {
  dirty.value = false
  unsavedDialog.value = false
  fileStore.clearSelectedFile()
}

async function saveAndClose() {
  if (!fileStore.selectedFile) return
  saving.value = true
  try {
    await fileStore.saveFileContent(fileStore.selectedFile, code.value)
    originalContent.value = code.value
    dirty.value = false
    appStore.showMessage('File saved', 'success')
    unsavedDialog.value = false
    fileStore.clearSelectedFile()
  } catch {
    appStore.showMessage('Failed to save file', 'error')
  } finally {
    saving.value = false
  }
}

function close() {
  fileStore.clearSelectedFile()
}

function handleKeydown(e: KeyboardEvent) {
  if ((e.ctrlKey || e.metaKey) && e.key === 's') {
    e.preventDefault()
    save()
  }
}

watch(() => fileStore.selectedFile, (newPath) => {
  if (newPath) {
    loadFileContent()
  }
})

onMounted(() => {
  if (fileStore.selectedFile) {
    loadFileContent()
  }
  window.addEventListener('keydown', handleKeydown)
})

onBeforeUnmount(() => {
  window.removeEventListener('keydown', handleKeydown)
})
</script>

<style scoped>
.editor-wrapper {
  overflow: auto;
}

.editor-wrapper :deep(.cm-editor) {
  height: 100%;
}

.editor-wrapper :deep(.cm-scroller) {
  overflow: auto;
  font-family: 'Consolas', 'Courier New', 'Fira Code', monospace;
  font-size: 13px;
  line-height: 1.6;
}
</style>
