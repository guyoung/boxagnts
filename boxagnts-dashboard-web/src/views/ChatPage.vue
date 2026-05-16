<template>
  <div class="chat-layout">
    <div class="d-flex align-center mb-4" v-if="currentProject">
      <v-icon color="primary" class="mr-2">mdi-folder</v-icon>
      <span class="text-body-2 text-medium-emphasis">{{ currentProject.id }}</span>
    </div>

    <div ref="scrollContainer" class="messages-container mb-4">
      <div v-if="messages.length === 0 && !isRunning" class="empty-state">
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

      <template v-for="(item, idx) in messages" :key="idx">
        <div v-if="item.kind === 'user'" :class="['message-row', 'justify-end']">
          <div class="message-bubble user">
            <div v-if="item.editing" class="edit-wrap">
              <v-textarea v-model="editText" variant="plain" rows="1" auto-grow hide-details class="edit-textarea"
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
                  <v-btn v-if="isLastUserMessage(idx) && !isRunning" icon="mdi-pencil" variant="text" size="x-small"
                    color="medium-emphasis" @click="editMessage(idx)" title="Edit" />
                  <v-btn v-if="isLastUserMessage(idx) && !isRunning" icon="mdi-refresh" variant="text" size="x-small"
                    color="medium-emphasis" @click="resubmitMessage(idx)" title="Resubmit" />
                  <v-btn v-if="!isRunning" icon="mdi-delete" variant="text" size="x-small" color="medium-emphasis"
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
                    <div class="text-caption font-weight-bold text-medium-emphasis mb-1">Input Parameters</div>
                    <pre class="tool-detail-text">{{ item.inputParams }}</pre>
                  </div>
                  <div v-if="item.toolType === 'tool_end'">
                    <div class="text-caption font-weight-bold text-medium-emphasis mb-1">Result</div>
                    <pre class="tool-detail-text">{{ item.content || '(empty)' }}</pre>
                  </div>
                </div>
              </div>
            </v-expand-transition>
          </v-card>
        </div>

        <div v-else-if="item.kind === 'assistant'" class="message-row justify-start">
          <div class="message-bubble assistant">
            <div v-if="item.text" class="message-text markdown-body" v-html="renderMarkdown(item.text)" />
            <div v-else-if="item.isLoading" class="loading-indicator">
              <span class="dot" /><span class="dot" /><span class="dot" />
            </div>
            <div class="d-flex justify-end mt-1">
              <div class="d-flex">
                <v-btn v-if="item.text" icon="mdi-content-copy" variant="text" size="x-small" color="medium-emphasis"
                  @click="copyText(item.text)" title="Copy response" />
                <v-btn v-if="item.text && !item.isLoading && !isRunning" icon="mdi-delete" variant="text" size="x-small"
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
            @keydown.enter.exact.prevent="sendMessage" @keydown.shift.enter="prompt += '\n'" :disabled="isRunning"
            class="chat-input" />
          <div class="d-flex flex-column gap-2">
            <v-btn icon="mdi-send" color="primary" variant="tonal" @click="sendMessage"
              :disabled="!prompt.trim() || isRunning" :loading="isRunning" size="small" />
            <v-btn v-if="isRunning" icon="mdi-stop" color="error" variant="tonal" size="small"
              @click="cancelExecution" />
          </div>
        </div>
        <div class="d-flex align-center mt-2">
          <v-icon size="small" class="mr-1" color="medium-emphasis">mdi-cube</v-icon>
          <v-select v-model="selectedModel" :items="availableModels" density="compact" variant="solo-filled"
            hide-details flat style="max-width: 200px" />
        </div>
      </v-card-text>
    </v-card>
  </div>

  <!-- Delete message confirmation dialog -->
  <v-dialog v-model="deleteMsgDialog" max-width="400">
    <v-card>
      <v-card-title>Delete Message</v-card-title>
      <v-card-text>
        <p>Delete this message?</p>
        <p v-if="deleteMsgIsUser" class="text-caption text-medium-emphasis mt-2">
          This will also delete the assistant's response below it.
        </p>
      </v-card-text>
      <v-card-actions>
        <v-spacer />
        <v-btn variant="text" @click="deleteMsgDialog = false">Cancel</v-btn>
        <v-btn color="error" @click="handleDeleteMsg" :loading="deletingMsg">Delete</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { ref, onMounted, nextTick, watch } from 'vue'
import { api, type Project, type SessionMessage, type ContentBlock } from '@/api'
import { useAppStore } from '@/stores/app'
import { useSessionStore } from '@/stores/sessions'
import { marked } from 'marked'
import DOMPurify from 'dompurify'


const availableModels = [
  'minimax/MiniMax-M2.7', 'deepseek/deepseek-v4-pro', 'deepseek/deepseek-v4-flash'
]
const quickPrompts = [
  'Explain this codebase', 'Find bugs and suggest fixes', 'Refactor for better readability',
  'Add unit tests', 'Write documentation',
]

const appStore = useAppStore()
const sessionStore = useSessionStore()
const currentProject = ref<Project | null>(null)
const prompt = ref('')
const messages = ref<DisplayItem[]>([])
const isRunning = ref(false)
const sessionId = ref<string | null>(null)
const selectedModel = ref<string>(availableModels[0])
const messagesEnd = ref<HTMLElement | null>(null)
const scrollContainer = ref<HTMLElement | null>(null)

let activeUserIdx = -1
let activeAsstIdx = -1
let pendingText = ''
let toolIdx = -1
let deleteMsgIdx = -1

interface ToolItem {
  kind: 'tool'; tool: string; toolType: 'tool_start' | 'tool_end'
  content: string; inputParams: string; isError: boolean; expanded: boolean; timestamp: string; uuid: string
}
interface AssistantItem {
  kind: 'assistant'; text: string; timestamp: string; isLoading: boolean; uuid: string
}
interface UserItem {
  kind: 'user'; text: string; timestamp: string; uuid: string; editing: boolean
}
type DisplayItem = UserItem | ToolItem | AssistantItem


let activeCleanup: (() => void) | null = null

function renderMarkdown(text: string): string {
  try { return DOMPurify.sanitize(marked.parse(text, { async: false }) as string || '') } catch { return escapeHtml(text) }
}
function escapeHtml(str: string): string { return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;') }
function formatTime(ts: string) { return new Date(ts).toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' }) }

async function copyText(text: string) {
  try { await navigator.clipboard.writeText(text); appStore.showMessage('Copied to clipboard', 'success') } catch { appStore.showMessage('Failed to copy', 'error') }
}

function scrollToBottom() {
  nextTick(() => {
    if (scrollContainer.value) scrollContainer.value.scrollTop = scrollContainer.value.scrollHeight
    messagesEnd.value?.scrollIntoView({ behavior: 'smooth' })
  })
}
watch(() => messages.value.length, () => scrollToBottom())
watch(() => { const last = messages.value[messages.value.length - 1]; if (last?.kind === 'assistant') return (last as AssistantItem).text; return null }, () => scrollToBottom())

// --- Watch sessionStore for selection changes ---
watch(() => sessionStore.currentSessionId, (newId) => {
  
  activeCleanup?.()
  activeCleanup = null
  isRunning.value = false
  if (newId) {
    sessionId.value = newId
    if (activeUserIdx === -1) {
      console.log("loadSessionHistory", activeUserIdx) 
      loadSessionHistory(newId)
    }
    
  } else {
    sessionId.value = null
    messages.value = []
    activeUserIdx = -1
    activeAsstIdx = -1
    pendingText = ''
    toolIdx = -1
  }
}, { immediate: true })

async function loadSessionHistory(sid: string) {
  messages.value = []
  try {
    const result = await api.loadSessionHistory(sid)
    if (!result) return

    // result is { messages: [...], ... } or a raw array
    const msgs: SessionMessage[] = Array.isArray(result)
      ? result
      : result.messages || []

    const items: DisplayItem[] = []
    for (const msg of msgs) {
      if (!msg) continue

      if (msg.role === 'user') {
        if (typeof msg.content === 'string') {
          if (msg.content.trim()) {
            items.push({
              kind: 'user',
              text: msg.content,
              timestamp: '',
              uuid: msg.uuid || '',
              editing: false
            })
          }
        } else if (Array.isArray(msg.content)) {
          for (const block of msg.content) {
            if (block.type === 'tool_result') {
              items.push({
                kind: 'tool',
                tool: block.tool_use_id || 'tool',
                toolType: 'tool_end',
                content: block.content || '',
                inputParams: '',
                isError: !!block.is_error,
                expanded: false,
                timestamp: '',
                uuid: msg.uuid || '',
              })
            }
          }
        }
      } else if (msg.role === 'assistant') {
        if (typeof msg.content === 'string') {
          if (msg.content.trim()) {
            items.push({
              kind: 'assistant',
              text: msg.content,
              timestamp: '',
              isLoading: false,
              uuid: msg.uuid || ''
            })
          }
        } else if (Array.isArray(msg.content)) {
          for (const block of msg.content) {
            if (block.type === 'text') {
              items.push({
                kind: 'assistant',
                text: block.text,
                timestamp: '',
                isLoading: false,
                uuid: msg.uuid || ''
              })
            } else if (block.type === 'tool_use') {
              items.push({
                kind: 'tool',
                tool: block.name,
                toolType: 'tool_start',
                content: '',
                inputParams: JSON.stringify(block.input || {}, null, 2),
                isError: false,
                expanded: false,
                timestamp: '',
                uuid: msg.uuid || '',
              })
            }
          }
        }
      }
    }
    messages.value = items;
    scrollToBottom();
  } catch {
    appStore.showMessage('Failed to load session history', 'error')
  }
}

function parseContentObject(content: any) {
  if (!content) return null
  if (typeof content === 'string') return content.trim() ? {
    type: 'text' as const,
    text: content,
    uuid: ''
  } : null
  if (typeof content !== 'object') return null
  if (content.type === 'text_delta' && typeof content.text === 'string') return {
    type: 'text' as const,
    text: content.text,
    uuid: content.uuid
  }
  if (content.type === 'tool_start' && content.tool) return {
    type: 'tool_start' as const,
    tool: content.tool,
    inputJson: content.input_json,
    uuid: content.uuid
  }
  if (content.type === 'tool_end' && content.tool) return {
    type: 'tool_end' as const,
    tool: content.tool,
    isError: !!content.is_error,
    uuid: content.uuid
  }

  return null
}

function flushText() {
  if (!pendingText.trim()) {
    toolIdx = -1;
    return
  }
  if (activeAsstIdx < 0) return
  const m = messages.value[activeAsstIdx] as AssistantItem
  if (m.text) m.text += pendingText; else { m.text = pendingText; m.isLoading = false }
  pendingText = '';
  toolIdx = -1
}

function formatToolInput(inputJson: string): string {
  try { return JSON.stringify(JSON.parse(inputJson), null, 2) } catch { return inputJson }
}

function handleOutputEvent(content: any) {
  const result = parseContentObject(content)
  if (!result) return
  if (result.type === 'tool_start') {
    flushText();
    const item: ToolItem = {
      kind: 'tool',
      tool: result.tool!,
      toolType: 'tool_start',
      content: '',
      inputParams: result.inputJson ? formatToolInput(result.inputJson) : '',
      isError: false,
      expanded: true,
      timestamp: new Date().toISOString(),
      uuid: result.uuid
    }
    messages.value.splice(activeAsstIdx, 0, item);
    activeAsstIdx++;
    toolIdx = activeAsstIdx - 1
  } else if (result.type === 'tool_end') {
    if (toolIdx >= 0) {
      const t = messages.value[toolIdx] as ToolItem;
      t.toolType = 'tool_end';
      t.isError = result.isError ?? false;
      t.expanded = false
    }
    toolIdx = -1;
  } else if (result.type === 'text') {
    if (!messages.value[activeAsstIdx].uuid) messages.value[activeAsstIdx].uuid = result.uuid;
    pendingText += result.text;
  }
  scrollToBottom();
}

function sendMessage() {
  const text = prompt.value.trim()
  if (!text || isRunning.value || !currentProject.value) return
  const now = new Date().toISOString()
  messages.value.push({
    kind: 'user',
    text,
    timestamp: now,
    uuid: '',
    editing: false
  })
  activeUserIdx = messages.value.length - 1
  activeAsstIdx = messages.value.length
  messages.value.push({
    kind: 'assistant',
    text: '',
    timestamp: now,
    isLoading: true,
    uuid: ''
  })
  pendingText = '';
  toolIdx = -1;
  prompt.value = '';
  isRunning.value = true;
  scrollToBottom()

  activeCleanup?.()
  activeCleanup = null

  function onOutput(e: Event) {
    const detail = (e as CustomEvent).detail as { content: any; session_id?: string }
    if (detail.session_id && !sessionId.value) {
      sessionId.value = detail.session_id

      setTimeout(() => {        
        sessionStore.fetchSessions()
        sessionStore.selectSession(sessionId.value)
      }, 1000)

    }
    handleOutputEvent(detail.content)
  }
  function onComplete(e: Event) {
    cleanup()
    flushText()
    const detail = (e as CustomEvent).detail as { result: any }
    if (detail.result && detail.result.user_message_uuid) {
      const m1 = messages.value[activeUserIdx] as AssistantItem
      if (!m1.uuid) m1.uuid = detail.result.user_message_uuid
    }

    const m2 = messages.value[activeAsstIdx] as AssistantItem
    if (!m2.text) m2.text = '_(no response)_'
    m2.isLoading = false
    isRunning.value = false
    scrollToBottom()

  }
  function onError(e: Event) {
    cleanup()
    flushText()
    const m = messages.value[activeAsstIdx] as AssistantItem
    m.text = `**Error:** ${(e as CustomEvent).detail || 'Unknown error'}`
    m.isLoading = false
    isRunning.value = false
    scrollToBottom()
  }
  function cleanup() {
    window.removeEventListener('chat-output', onOutput)
    window.removeEventListener('chat-complete', onComplete)
    window.removeEventListener('chat-error', onError)
    activeCleanup = null
  }
  activeCleanup = cleanup
  window.addEventListener('chat-output', onOutput)
  window.addEventListener('chat-complete', onComplete)
  window.addEventListener('chat-error', onError)

  api.chatExecute(currentProject.value!.path, text, selectedModel.value, sessionId.value).catch(() => {
    cleanup()
    const m = messages.value[activeAsstIdx] as AssistantItem
    if (!m.text) m.text = '_(connection failed)_'
    m.isLoading = false
    isRunning.value = false
  })
}

function cancelExecution() {
  activeCleanup?.()
  activeCleanup = null
  isRunning.value = false
  const m = messages.value[activeAsstIdx] as AssistantItem | undefined
  if (m?.kind === 'assistant') {
    m.isLoading = false
    if (!m.text) m.text = '_(cancelled)_'
  }
  if (sessionId.value) {
    api.chatExecuteCancel(sessionId.value).catch(() => { })
  }
}

const deleteMsgDialog = ref(false)
const deleteMsgIsUser = ref(false)
const deletingMsg = ref(false)
const editText = ref('')

function confirmDeleteMsg(idx: number) {
  if (isRunning.value) return
  deleteMsgIdx = idx
  const item = messages.value[idx]
  deleteMsgIsUser.value = item?.kind === 'user'
  deleteMsgDialog.value = true
}

function isLastUserMessage(idx: number): boolean {
  for (let i = idx + 1; i < messages.value.length; i++) {
    if (messages.value[i].kind === 'user') return false
  }
  return true
}

function editMessage(idx: number) {
  if (isRunning.value) return
  const item = messages.value[idx]
  if (item.kind !== 'user') return
  item.editing = true
  editText.value = item.text
  nextTick(() => {
    const input = document.querySelector('.edit-wrap textarea') as HTMLTextAreaElement | null
    input?.focus()
  })
}

async function confirmEdit(idx: number) {
  const item = messages.value[idx]
  if (item.kind !== 'user' || isRunning.value || !currentProject.value) return
  const newText = editText.value.trim()
  if (!newText) return


  const uuids: string[] = []

  // Remove this user message and all following non-user messages
  while (idx + 1 < messages.value.length && messages.value[idx + 1]?.kind !== 'user') {
    uuids.push(messages.value[idx + 1].uuid)
    messages.value.splice(idx + 1, 1);
  }
  uuids.push(messages.value[idx].uuid)
  messages.value.splice(idx, 1);
  editText.value = '';
  try {
    if (sessionId.value) {
      await api.deleteSessionMessages(sessionId.value, uuids)
    }
  } catch {

  }

  // Re-send
  const now = new Date().toISOString()
  messages.value.push({ kind: 'user', text: newText, timestamp: now, uuid: '', editing: false })
  activeUserIdx =  messages.value.length-1
  activeAsstIdx = messages.value.length
  messages.value.push({ kind: 'assistant', text: '', timestamp: now, isLoading: true, uuid: '' })
  pendingText = '';
  toolIdx = -1;
  isRunning.value = true;
  scrollToBottom()

  activeCleanup?.()
  activeCleanup = null

  function onOutput(e: Event) {
    const detail = (e as CustomEvent).detail as { content: any; session_id?: string }
    handleOutputEvent(detail.content)
  }
  function onComplete(e: Event) {
    cleanup()
    flushText()
    const m = messages.value[activeAsstIdx] as AssistantItem
    if (!m.text) m.text = '_(no response)_'
    m.isLoading = false
    isRunning.value = false
    scrollToBottom()
  }
  function onError(e: Event) {
    cleanup()
    flushText()
    const m = messages.value[activeAsstIdx] as AssistantItem
    m.text = `**Error:** ${(e as CustomEvent).detail || 'Unknown error'}`
    m.isLoading = false
    isRunning.value = false
    scrollToBottom()
  }
  function cleanup() {
    window.removeEventListener('chat-output', onOutput)
    window.removeEventListener('chat-complete', onComplete)
    window.removeEventListener('chat-error', onError)
    activeCleanup = null
  }
  activeCleanup = cleanup
  window.addEventListener('chat-output', onOutput)
  window.addEventListener('chat-complete', onComplete)
  window.addEventListener('chat-error', onError)

  api.chatExecute(currentProject.value!.path, newText, selectedModel.value, sessionId.value).catch(() => {
    cleanup()
    const m = messages.value[activeAsstIdx] as AssistantItem
    if (!m.text) m.text = '_(connection failed)_'
    m.isLoading = false
    isRunning.value = false
  })
}

async function resubmitMessage(idx: number) {
  if (isRunning.value || !currentProject.value) return
  const item = messages.value[idx]
  if (item.kind !== 'user') return

  const uuids: string[] = []

  // Remove this user message and all following non-user messages
  while (idx + 1 < messages.value.length && messages.value[idx + 1]?.kind !== 'user') {
    uuids.push(messages.value[idx + 1].uuid)
    messages.value.splice(idx + 1, 1)
  }
  const text = item.text
  uuids.push(messages.value[idx].uuid)
  messages.value.splice(idx, 1)
  try {
    if (sessionId.value) {
      await api.deleteSessionMessages(sessionId.value, uuids)
    }
  } catch {

  }

  // Re-send the message
  const now = new Date().toISOString()
  messages.value.push({ kind: 'user', text, timestamp: now, uuid: '', editing: false })
  activeUserIdx =  messages.value.length-1
  activeAsstIdx = messages.value.length
  messages.value.push({ kind: 'assistant', text: '', timestamp: now, isLoading: true, uuid: '' })
  pendingText = '';
  toolIdx = -1;
  isRunning.value = true;
  scrollToBottom()

  activeCleanup?.()
  activeCleanup = null

  function onOutput(e: Event) {
    const detail = (e as CustomEvent).detail as { content: any; session_id?: string }
    handleOutputEvent(detail.content)
  }
  function onComplete() {
    cleanup()
    flushText()
    const m = messages.value[activeAsstIdx] as AssistantItem
    if (!m.text) m.text = '_(no response)_'
    m.isLoading = false
    isRunning.value = false
    scrollToBottom()

  }
  function onError(e: Event) {
    cleanup()
    flushText()
    const m = messages.value[activeAsstIdx] as AssistantItem
    m.text = `**Error:** ${(e as CustomEvent).detail || 'Unknown error'}`
    m.isLoading = false
    isRunning.value = false
    scrollToBottom()
  }
  function cleanup() {
    window.removeEventListener('chat-output', onOutput)
    window.removeEventListener('chat-complete', onComplete)
    window.removeEventListener('chat-error', onError)
    activeCleanup = null
  }
  activeCleanup = cleanup
  window.addEventListener('chat-output', onOutput)
  window.addEventListener('chat-complete', onComplete)
  window.addEventListener('chat-error', onError)

  api.chatExecute(currentProject.value!.path, text, selectedModel.value, sessionId.value).catch(() => {
    cleanup()
    const m = messages.value[activeAsstIdx] as AssistantItem
    if (!m.text) m.text = '_(connection failed)_'
    m.isLoading = false
    isRunning.value = false
  })
}

async function handleDeleteMsg() {
  if (deleteMsgIdx < 0 || deleteMsgIdx >= messages.value.length) return
  const item = messages.value[deleteMsgIdx]
  deletingMsg.value = true

  // Collect UUIDs to delete: this item + following items if it's a user message
  const uuids: string[] = []
  uuids.push(item.uuid)

  if (item.kind === 'user') {
    // Also delete the assistant/tool items that follow until next user
    for (let i = deleteMsgIdx + 1; i < messages.value.length; i++) {
      const next = messages.value[i]
      if (next.kind === 'user') break
      if (next.uuid) uuids.push(next.uuid)
    }
  }

  try {
    if (sessionId.value) {
      await api.deleteSessionMessages(sessionId.value, uuids)
    }
    // Remove from display
    if (item.kind === 'user') {
      while (deleteMsgIdx + 1 < messages.value.length && messages.value[deleteMsgIdx + 1]?.kind !== 'user') {
        messages.value.splice(deleteMsgIdx + 1, 1)
      }
    }
    messages.value.splice(deleteMsgIdx, 1)
    appStore.showMessage('Message deleted', 'success')
  } catch {
    appStore.showMessage('Failed to delete message', 'error')
  } finally {
    deletingMsg.value = false
    deleteMsgDialog.value = false
    deleteMsgIdx = -1
  }
}

onMounted(async () => {
  try {
    currentProject.value = await api.getCurrentProject()
  } catch {
    appStore.showMessage('Could not detect current project', 'warning')
  }
  sessionStore.fetchSessions()
})
</script>

<style scoped>
.chat-layout {
  display: flex;
  flex-direction: column;
  height: calc(100vh - 48px);
}

.messages-container {
  flex: 1;
  overflow-y: auto;
  padding: 8px 0;
  min-height: 0;
  max-width: 900px;
  width: 100%;
  margin: 0 auto;
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
  max-height: 300px;
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
  position: sticky;
  bottom: 0;
  z-index: 10;
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

.messages-container::-webkit-scrollbar {
  width: 8px;
}

.messages-container::-webkit-scrollbar-thumb {
  background: rgba(var(--v-theme-on-surface), 0.15);
  border-radius: 4px;
}
</style>
