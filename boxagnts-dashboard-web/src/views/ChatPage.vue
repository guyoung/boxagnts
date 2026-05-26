<template>
  <div class="chat-layout">
    <div ref="scrollContainer" class="messages-container">
      <div v-if="messages.length === 0 && !uiState.isRunning && availableModels.length === 0" class="empty-state">
        <v-icon size="80" color="warning" class="mb-4">mdi-alert-circle</v-icon>
        <p class="text-h6 text-medium-emphasis">No models available</p>
        <p class="text-body-2 text-medium-emphasis mt-1">Please configure a model provider first.</p>
        <v-btn color="primary" variant="tonal" class="mt-4" prepend-icon="mdi-robot" @click="goToModelSettings">
          Go to Model Settings
        </v-btn>
      </div>

      <div v-else-if="messages.length === 0 && !uiState.isRunning" class="empty-state">
        <v-icon size="80" color="medium-emphasis" class="mb-4">mdi-robot</v-icon>
        <p class="text-h6 text-medium-emphasis">Ask Boxagnts anything</p>
        <p class="text-body-2 text-medium-emphasis mt-1">Type your message below to start</p>
        <div class="d-flex gap-2 mt-4 flex-wrap justify-center">
          <v-chip v-for="suggestion in quickPrompts" :key="suggestion" variant="tonal" size="small"
            @click="prompt = suggestion; sendMessage()">
            {{ suggestion }}
          </v-chip>
        </div>
      </div>

      <template v-for="(item, idx) in messages" :key="item.uuid">
        <div v-if="item.kind === 'user'" :class="['message-row', 'justify-end']">
          <div class="message-bubble user">
            <div v-if="item.editing" class="edit-wrap">
              <v-textarea v-model="uiState.editText" variant="plain" rows="1" auto-grow hide-details class="edit-textarea"
                @keydown.enter.exact.prevent="confirmEdit(idx)" />
              <div class="d-flex justify-end mt-1">
                <v-btn icon="mdi-close" variant="text" size="x-small" color="medium-emphasis"
                  @click="item.editing = false" title="Cancel" />
                <v-btn icon="mdi-check" variant="text" size="x-small" color="medium-emphasis" @click="confirmEdit(idx)"
                  title="Submit" />
              </div>
            </div>
            <template v-else>
              <div class="message-text">{{ item.text }}</div>
              <div class="d-flex justify-end mt-1">
                <div class="d-flex">
                  <v-btn v-if="isLastUserMessage(idx) && !uiState.isRunning" icon="mdi-pencil" variant="text" size="x-small"
                    color="medium-emphasis" @click="editMessage(idx)" title="Edit" />
                  <v-btn v-if="isLastUserMessage(idx) && !uiState.isRunning" icon="mdi-refresh" variant="text" size="x-small"
                    color="medium-emphasis" @click="resubmitMessage(idx)" title="Resubmit" />
                  <v-btn v-if="!uiState.isRunning" icon="mdi-delete" variant="text" size="x-small" color="medium-emphasis"
                    @click="confirmDeleteMsg(idx)" title="Delete" />
                </div>
              </div>
            </template>
          </div>
        </div>

        <div v-else-if="item.kind === 'tool'" class="tool-standalone">
          <v-card variant="flat" color="surface" rounded="lg" class="tool-card">
            <div class="d-flex align-center pa-3 tool-header" @click="item.expanded = !item.expanded">
              <v-icon :color="item.toolType === 'tool_start' ? 'warning' : item.isError ? 'error' : 'success'" size="18"
                class="mr-2">
                {{ item.toolType === 'tool_start' ? 'mdi-code-braces' : item.isError ? 'mdi-alert-circle' :
                  'mdi-check-circle' }}
              </v-icon>
              <div class="flex-1">
                <div class="text-body-2 font-weight-medium">
                  {{ item.tool }}
                  <span v-if="item.toolType === 'tool_start'"> · running</span>
                  <span v-else-if="item.isError"> · failed</span>
                  <span v-else> · done</span>
                </div>
                <div v-if="item.toolType === 'tool_start' && item.inputParams"
                  class="text-caption text-medium-emphasis mt-1 line-clamp-1">
                  {{ item.inputParams }}
                </div>
                <div v-else-if="item.toolType === 'tool_end'" class="text-caption mt-1 line-clamp-1"
                  :class="item.isError ? 'text-error' : 'text-medium-emphasis'">
                  {{ item.content || (item.isError ? 'Execution failed' : 'Completed') }}
                </div>
              </div>
              <v-spacer />
              <v-icon size="16" color="medium-emphasis">
                {{ item.expanded ? 'mdi-chevron-up' : 'mdi-chevron-down' }}
              </v-icon>
            </div>
            <v-expand-transition>
              <div v-show="item.expanded">
                <v-divider />
                <div class="pa-3">
                  <div v-if="item.inputParams" class="mb-3">
                    <div class="d-flex align-center mb-1">
                      <span class="text-caption font-weight-bold text-medium-emphasis">Input Parameters</span>
                      <v-spacer />
                      <v-btn icon="mdi-content-copy" variant="text" size="x-small" color="medium-emphasis"
                        @click.stop="copyText(item.inputParams)" title="Copy" />
                    </div>
                    <pre class="tool-detail-text">{{ item.inputParams }}</pre>
                  </div>
                  <div v-if="item.toolType === 'tool_end'">
                    <div class="d-flex align-center mb-1">
                      <span class="text-caption font-weight-bold text-medium-emphasis">Result</span>
                      <v-spacer />
                      <v-btn icon="mdi-content-copy" variant="text" size="x-small" color="medium-emphasis"
                        @click.stop="copyText(item.content || '')" title="Copy" />
                    </div>
                    <pre class="tool-detail-text">{{ item.content || '(empty)' }}</pre>
                  </div>
                </div>
              </div>
            </v-expand-transition>
          </v-card>
        </div>

        <div v-else-if="item.kind === 'assistant'" class="message-row justify-start">
          <div class="message-bubble assistant">
            <div v-if="item.text" class="message-text markdown-body" v-html="renderMarkdown(item.uuid, item.text)" />
            <div v-else-if="item.isLoading" class="loading-indicator">
              <span class="dot" /><span class="dot" /><span class="dot" />
            </div>
            <div class="d-flex justify-end mt-1">
              <div class="d-flex">
                <v-btn v-if="item.text" icon="mdi-content-copy" variant="text" size="x-small" color="medium-emphasis"
                  @click="copyText(item.text)" title="Copy response" />
                <v-btn v-if="item.text && !item.isLoading && !uiState.isRunning" icon="mdi-delete" variant="text" size="x-small"
                  color="medium-emphasis" @click="confirmDeleteMsg(idx)" title="Delete" />
              </div>
            </div>
          </div>
        </div>
      </template>

      <div ref="messagesEnd" />
    </div>

    <v-card flat color="surface-variant" rounded="xl" class="input-card">
      <v-card-text class="pa-3">
        <div class="d-flex align-end gap-3">
          <v-textarea v-model="prompt" placeholder="Ask Boxagnts..." variant="plain" rows="1" auto-grow hide-details
            @keydown.enter.exact.prevent="sendMessage" @keydown.shift.enter="prompt += '\n'" :disabled="uiState.isRunning"
            class="chat-input" />
          <div class="d-flex flex-column gap-2">
            <v-btn icon="mdi-send" color="primary" variant="tonal" @click="sendMessage"
              :disabled="!prompt.trim() || uiState.isRunning" :loading="uiState.isRunning" size="small" />
            <v-btn v-if="uiState.isRunning" icon="mdi-stop" color="error" variant="tonal" size="small"
              @click="cancelExecution" />
          </div>
        </div>
        <div class="d-flex align-center mt-2">
          <v-icon size="small" class="mr-1" color="medium-emphasis">mdi-cube</v-icon>
          <v-select v-model="uiState.selectedModel" :items="availableModels" density="compact" variant="solo-filled"
            hide-details flat style="max-width: 200px" />
        </div>
      </v-card-text>
    </v-card>
  </div>

  <!-- Delete message confirmation dialog -->
  <v-dialog v-model="uiState.deleteMsgDialog" max-width="400">
    <v-card>
      <v-card-title>Delete Message</v-card-title>
      <v-card-text>
        <p>Delete this message?</p>
        <p v-if="uiState.deleteMsgIsUser" class="text-caption text-medium-emphasis mt-2">
          This will also delete the assistant's response below it.
        </p>
      </v-card-text>
      <v-card-actions>
        <v-spacer />
        <v-btn variant="text" @click="uiState.deleteMsgDialog = false">Cancel</v-btn>
        <v-btn color="error" @click="handleDeleteMsg" :loading="uiState.deletingMsg">Delete</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { ref, reactive, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import { type Project } from '@/api'
import { useAppStore } from '@/stores/app'
import { useMarkdownRender } from '@/composables/useMarkdownRender'
import { useChatScroll } from '@/composables/useChatScroll'
import { useChatSession } from '@/composables/useChatSession'
import { useChatMessages, type DisplayItem, type ChatUiState } from '@/composables/useChatMessages'

const router = useRouter()
const appStore = useAppStore()

const { renderMarkdown } = useMarkdownRender()

const quickPrompts = [
  'Write JavaScript code and run it', 'Develop website', 'Develop HTML5 game',
  'Run Linux Shell commands', 'Get today\'s weather',
]

function goToModelSettings() {
  router.push('/settings/model')
}

async function copyText(text: string) {
  try {
    await navigator.clipboard.writeText(text)
    appStore.showMessage('Copied to clipboard', 'success')
  } catch {
    appStore.showMessage('Failed to copy', 'error')
  }
}

const messages = ref<DisplayItem[]>([])
const prompt = ref('')
const { scrollContainer, messagesEnd, scrollToBottom } = useChatScroll()

const sessionId = ref<string | null>(null)
const currentProject = ref<Project | null>(null)
const availableModels = ref<string[]>([])

const uiState = reactive<ChatUiState>({
  isRunning: false,
  selectedModel: '',
  deleteMsgIdx: -1,
  deleteMsgDialog: false,
  deleteMsgIsUser: false,
  deletingMsg: false,
  editText: '',
})

const {
  sendMessage,
  resubmitMessage: doResubmit,
  confirmEdit,
  handleDeleteMsg,
  cleanupActiveStream,
} = useChatMessages({
  messages, prompt, sessionId, currentProject, uiState, scrollToBottom,
})

const { cancelExecution } = useChatSession({
  sessionId, currentProject, availableModels, messages, uiState, scrollToBottom, cleanupActiveStream,
})

function resubmitMessage(idx: number) {
  const item = messages.value[idx]
  if (item?.kind === 'user') {
    doResubmit(idx, item.text)
  }
}

function confirmDeleteMsg(idx: number) {
  if (uiState.isRunning) return
  uiState.deleteMsgIdx = idx
  const item = messages.value[idx]
  uiState.deleteMsgIsUser = item?.kind === 'user'
  uiState.deleteMsgDialog = true
}

function isLastUserMessage(idx: number): boolean {
  for (let i = idx + 1; i < messages.value.length; i++) {
    if (messages.value[i].kind === 'user') return false
  }
  return true
}

function editMessage(idx: number) {
  if (uiState.isRunning) return
  const item = messages.value[idx]
  if (item.kind !== 'user') return
  item.editing = true
  uiState.editText = item.text
  nextTick(() => {
    const input = document.querySelector('.edit-wrap textarea') as HTMLTextAreaElement | null
    input?.focus()
  })
}
</script>

<style scoped>
.chat-layout {
  display: flex;
  flex-direction: column;
  height: 100%;
  min-height: 0;
  overflow: hidden;
}

.messages-container {
  flex: 1;
  overflow-y: auto;
  padding: 8px 0;
  margin: 0 auto;
  max-width: 900px;
  width: 100%;
  min-height: calc(100vh - 300px);
  max-height: calc(100vh - 225px);
}

.empty-state {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  height: 100%;
  min-height: 300px;
}

.message-row {
  display: flex;
  margin-bottom: 16px;
  padding: 0 8px;
}

.message-bubble {
  max-width: 80%;
  padding: 12px 16px;
  border-radius: 18px;
  position: relative;
}

.message-bubble.user {
  background: rgb(var(--v-theme-primary));
  color: rgb(var(--v-theme-on-primary));
  border-bottom-right-radius: 6px;
}

.message-bubble.assistant {
  background: rgb(var(--v-theme-surface-variant));
  border-bottom-left-radius: 6px;
  min-width: 80px;
}

.message-text {
  font-size: 0.9375rem;
  line-height: 1.65;
}

.message-meta {
  opacity: 0.5;
}

.edit-wrap :deep(.v-field__input) {
  font-size: 0.9375rem;
  line-height: 1.65;
  color: rgb(var(--v-theme-on-primary)) !important;
}

.markdown-body :deep(p) {
  margin: 0 0 8px;
}

.markdown-body :deep(p:last-child) {
  margin-bottom: 0;
}

.markdown-body :deep(pre) {
  background: rgba(var(--v-theme-on-surface), 0.08);
  border-radius: 8px;
  padding: 12px 16px;
  overflow-x: auto;
  margin: 8px 0;
  font-size: 0.8125rem;
}

.markdown-body :deep(code) {
  font-family: 'Cascadia Code', 'Fira Code', Consolas, monospace;
  font-size: 0.875em;
}

.markdown-body :deep(pre code) {
  background: none;
  padding: 0;
  font-size: 0.8125rem;
}

.markdown-body :deep(:not(pre) > code) {
  background: rgba(var(--v-theme-on-surface), 0.1);
  padding: 2px 6px;
  border-radius: 4px;
}

.markdown-body :deep(ul),
.markdown-body :deep(ol) {
  padding-left: 20px;
  margin: 4px 0 8px;
}

.markdown-body :deep(li) {
  margin-bottom: 2px;
}

.markdown-body :deep(blockquote) {
  border-left: 3px solid rgba(var(--v-theme-on-surface), 0.2);
  padding-left: 12px;
  margin: 8px 0;
  opacity: 0.85;
}

.markdown-body :deep(h1),
.markdown-body :deep(h2),
.markdown-body :deep(h3),
.markdown-body :deep(h4) {
  margin: 12px 0 6px;
  font-weight: 600;
}

.markdown-body :deep(h1) {
  font-size: 1.3em;
}

.markdown-body :deep(h2) {
  font-size: 1.15em;
}

.markdown-body :deep(h3) {
  font-size: 1.05em;
}

.markdown-body :deep(table) {
  border-collapse: collapse;
  margin: 8px 0;
  width: 100%;
}

.markdown-body :deep(th),
.markdown-body :deep(td) {
  border: 1px solid rgba(var(--v-theme-on-surface), 0.12);
  padding: 6px 10px;
  text-align: left;
  font-size: 0.875rem;
}

.markdown-body :deep(th) {
  background: rgba(var(--v-theme-on-surface), 0.04);
}

.markdown-body :deep(strong) {
  font-weight: 600;
}

.markdown-body :deep(hr) {
  border: none;
  border-top: 1px solid rgba(var(--v-theme-on-surface), 0.12);
  margin: 12px 0;
}

.tool-standalone {
  margin-bottom: 16px;
  padding: 0 24px;
}

.tool-card {
  border: 1px solid rgba(var(--v-theme-on-surface), 0.08);
}

.tool-header {
  cursor: pointer;
  user-select: none;
}

.tool-header:hover {
  background: rgba(var(--v-theme-on-surface), 0.04);
}

.tool-detail-text {
  white-space: pre-wrap;
  word-break: break-all;
  font-family: 'Cascadia Code', 'Fira Code', monospace;
  font-size: 0.8125rem;
  line-height: 1.5;
  color: rgba(var(--v-theme-on-surface), 0.7);
  max-height: 600px;
  overflow-y: auto;
}

.loading-indicator {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 0;
  min-height: 24px;
}

.loading-indicator .dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: rgba(var(--v-theme-on-surface), 0.35);
  animation: pulse 1.4s infinite ease-in-out both;
}

.loading-indicator .dot:nth-child(1) {
  animation-delay: -0.32s;
}

.loading-indicator .dot:nth-child(2) {
  animation-delay: -0.16s;
}

.loading-indicator .dot:nth-child(3) {
  animation-delay: 0s;
}

@keyframes pulse {

  0%,
  80%,
  100% {
    transform: scale(0.6);
    opacity: 0.3;
  }

  40% {
    transform: scale(1);
    opacity: 1;
  }
}

.flex-1 {
  flex: 1;
  min-width: 0;
}

.line-clamp-1 {
  display: -webkit-box;
  -webkit-line-clamp: 1;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.input-card {
  flex-shrink: 0;
  max-width: 900px;
  width: 100%;
  margin: 0 auto;
}

.chat-input :deep(.v-field__input) {
  font-size: 0.9375rem;
}

.gap-2 {
  gap: 8px;
}

.gap-3 {
  gap: 12px;
}
</style>

<style>
.messages-container::-webkit-scrollbar {
  width: 6px;
}

.messages-container::-webkit-scrollbar-track {
  background: transparent;
}

.messages-container::-webkit-scrollbar-thumb {
  background: rgba(var(--v-theme-on-surface), 0.18);
  border-radius: 3px;
}

.messages-container::-webkit-scrollbar-thumb:hover {
  background: rgba(var(--v-theme-on-surface), 0.3);
}

.messages-container {
  scrollbar-width: thin;
  scrollbar-color: rgba(var(--v-theme-on-surface), 0.18) transparent;
}
</style>
