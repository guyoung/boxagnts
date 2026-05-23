import { type Ref } from 'vue'
import { api, type Project, type SessionMessage } from '@/api'
import { useAppStore } from '@/stores/app'
import { useSessionStore } from '@/stores/sessions'

interface BaseMessage {
  uuid: string
  kind: 'user' | 'assistant' | 'tool'
  timestamp: string
}

export interface UserItem extends BaseMessage {
  kind: 'user'
  text: string
  editing: boolean
}

export interface ToolItem extends BaseMessage {
  kind: 'tool'
  tool: string
  toolType: 'tool_start' | 'tool_end'
  content: string
  inputParams: string
  isError: boolean
  expanded: boolean
}

export interface AssistantItem extends BaseMessage {
  kind: 'assistant'
  text: string
  isLoading: boolean
}

export type DisplayItem = UserItem | ToolItem | AssistantItem

function generateUuid(): string {
  return crypto.randomUUID()
}

export async function loadSessionHistory(sid: string): Promise<DisplayItem[]> {
  const result = await api.loadSessionHistory(sid)
  if (!result) return []

  const msgs: SessionMessage[] = Array.isArray(result) ? result : result.messages || []

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
            uuid: msg.uuid || generateUuid(),
            editing: false,
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
              uuid: `${msg.uuid || generateUuid()}_tool_${block.tool_use_id || ''}`,
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
            uuid: msg.uuid || generateUuid(),
          })
        }
      } else if (Array.isArray(msg.content)) {
        let toolBlockIdx = 0
        for (const block of msg.content) {
          if (block.type === 'text') {
            items.push({
              kind: 'assistant',
              text: block.text,
              timestamp: '',
              isLoading: false,
              uuid: msg.uuid || generateUuid(),
            })
          } else if (block.type === 'tool_use') {
            const toolUuid = block.id || generateUuid()
            items.push({
              kind: 'tool',
              tool: block.name,
              toolType: 'tool_start',
              content: '',
              inputParams: JSON.stringify(block.input || {}, null, 2),
              isError: false,
              expanded: false,
              timestamp: '',
              uuid: toolUuid,
            })
            toolBlockIdx++
          }
        }
      }
    }
  }
  return items
}

function parseContentObject(content: any) {
  if (!content) return null
  if (typeof content === 'string') {
    return content.trim()
      ? { type: 'text' as const, text: content, uuid: '' }
      : null
  }
  if (typeof content !== 'object') return null
  if (content.type === 'text_delta' && typeof content.text === 'string') {
    return { type: 'text' as const, text: content.text, uuid: content.uuid }
  }
  if (content.type === 'tool_start' && content.tool) {
    return { type: 'tool_start' as const, tool: content.tool, inputJson: content.input_json, uuid: content.uuid }
  }
  if (content.type === 'tool_end' && content.tool) {
    return { type: 'tool_end' as const, tool: content.tool, isError: !!content.is_error, uuid: content.uuid }
  }
  return null
}

function formatToolInput(inputJson: string): string {
  try { return JSON.stringify(JSON.parse(inputJson), null, 2) } catch { return inputJson }
}

export interface ChatUiState {
  isRunning: boolean
  selectedModel: string
  deleteMsgIdx: number
  deleteMsgDialog: boolean
  deleteMsgIsUser: boolean
  deletingMsg: boolean
  editText: string
}

interface StreamState {
  activeUserIdx: number
  activeAsstIdx: number
  pendingText: string
  toolIdx: number
  activeCleanup: (() => void) | null
}

export interface UseChatMessagesOptions {
  messages: Ref<DisplayItem[]>
  prompt: Ref<string>
  sessionId: Ref<string | null>
  currentProject: Ref<Project | null>
  uiState: ChatUiState
  scrollToBottom: () => void
}

export function useChatMessages(options: UseChatMessagesOptions) {
  const { messages, prompt, sessionId, currentProject, uiState, scrollToBottom } = options

  const appStore = useAppStore()
  const sessionStore = useSessionStore()

  const stream: StreamState = {
    activeUserIdx: -1,
    activeAsstIdx: -1,
    pendingText: '',
    toolIdx: -1,
    activeCleanup: null,
  }

  function flushText(createNewSlot = false) {
    if (!stream.pendingText.trim()) {
      stream.toolIdx = -1
      return
    }
    if (stream.activeAsstIdx < 0) return
    const m = messages.value[stream.activeAsstIdx] as AssistantItem
    if (m.text) {
      m.text += stream.pendingText
    } else {
      m.text = stream.pendingText
      m.isLoading = false
    }
    stream.pendingText = ''
    stream.toolIdx = -1
    if (createNewSlot) {
      messages.value.push({
        kind: 'assistant',
        text: '',
        timestamp: new Date().toISOString(),
        isLoading: true,
        uuid: generateUuid(),
      })
      stream.activeAsstIdx = messages.value.length - 1
    }
  }

  function handleOutputEvent(content: any) {
    const result = parseContentObject(content)
    if (!result) return
    if (result.type === 'tool_start') {
      flushText(true)
      const item: ToolItem = {
        kind: 'tool',
        tool: result.tool!,
        toolType: 'tool_start',
        content: '',
        inputParams: result.inputJson ? formatToolInput(result.inputJson) : '',
        isError: false,
        expanded: true,
        timestamp: new Date().toISOString(),
        uuid: result.uuid,
      }
      messages.value.splice(stream.activeAsstIdx, 0, item)
      stream.activeAsstIdx++
      stream.toolIdx = stream.activeAsstIdx - 1
    } else if (result.type === 'tool_end') {
      if (stream.toolIdx >= 0) {
        const t = messages.value[stream.toolIdx] as ToolItem
        t.toolType = 'tool_end'
        t.isError = result.isError ?? false
        t.expanded = false
      }
      stream.toolIdx = -1
    } else if (result.type === 'text') {
      if (result.uuid) {
        messages.value[stream.activeAsstIdx].uuid = result.uuid
      }
      stream.pendingText += result.text
    }
    scrollToBottom()
  }

  function startChatStream(text: string, isNewSession: boolean) {
    uiState.isRunning = true
    scrollToBottom()

    stream.activeCleanup?.()
    stream.activeCleanup = null

    let cleanupCalled = false
    let sessionRegistered = false

    function ensureSessionRefreshed(sid: string) {
      if (sessionRegistered) return
      sessionRegistered = true
      if (!sessionId.value) {
        sessionId.value = sid
        sessionStore.selectSession(sid)
      }
      sessionStore.fetchSessions()
    }

    function onSession(e: Event) {
      if (cleanupCalled) return
      const detail = (e as CustomEvent).detail as { session_id: string }
      if (isNewSession && detail.session_id) {
        ensureSessionRefreshed(detail.session_id)
      }
    }

    function onOutput(e: Event) {
      if (cleanupCalled) return
      const detail = (e as CustomEvent).detail as { content: any; session_id?: string }
      if (isNewSession && detail.session_id) {
        ensureSessionRefreshed(detail.session_id)
      }
      handleOutputEvent(detail.content)
    }

    function onComplete(e: Event) {
      if (cleanupCalled) return
      cleanup()
      flushText()
      const detail = (e as CustomEvent).detail as { result?: { user_message_uuid?: string; session_id?: string } }
      if (detail?.result?.user_message_uuid) {
        messages.value[stream.activeUserIdx].uuid = detail.result.user_message_uuid
      }
      if (isNewSession && detail?.result?.session_id) {
        ensureSessionRefreshed(detail.result.session_id)
      }
      const m = messages.value[stream.activeAsstIdx] as AssistantItem
      if (!m.text) m.text = '_(no response)_'
      m.isLoading = false
      uiState.isRunning = false
      scrollToBottom()
      if (isNewSession) {
        setTimeout(() => sessionStore.fetchSessions(), 500)
      }
    }

    function onError(e: Event) {
      if (cleanupCalled) return
      cleanup()
      flushText()
      const m = messages.value[stream.activeAsstIdx] as AssistantItem
      const errDetail = (e as CustomEvent).detail
      const errMsg = typeof errDetail === 'string' ? errDetail : 'Unknown error'
      m.text = `**Error:** ${errMsg}`
      m.isLoading = false
      uiState.isRunning = false
      scrollToBottom()
      if (isNewSession) {
        setTimeout(() => sessionStore.fetchSessions(), 500)
      }
    }

    function cleanup() {
      if (cleanupCalled) return
      cleanupCalled = true
      window.removeEventListener('chat-session', onSession)
      window.removeEventListener('chat-output', onOutput)
      window.removeEventListener('chat-complete', onComplete)
      window.removeEventListener('chat-error', onError)
      stream.activeCleanup = null
    }

    stream.activeCleanup = cleanup
    window.addEventListener('chat-session', onSession)
    window.addEventListener('chat-output', onOutput)
    window.addEventListener('chat-complete', onComplete)
    window.addEventListener('chat-error', onError)

    api.chatExecute(currentProject.value!.path, text, uiState.selectedModel, sessionId.value).catch((err: unknown) => {
      cleanup()
      flushText()
      const m = messages.value[stream.activeAsstIdx] as AssistantItem
      if (!m.text) {
        if (err instanceof TypeError && (err.message.includes('WebSocket') || err.message.includes('fetch'))) {
          m.text = '_(network error: unable to connect)_'
        } else if (err instanceof Error) {
          m.text = `_(connection failed: ${err.message})_`
        } else {
          m.text = '_(connection failed)_'
        }
      }
      m.isLoading = false
      uiState.isRunning = false
      scrollToBottom()
    })
  }

  function cleanupActiveStream() {
    stream.activeCleanup?.()
    stream.activeCleanup = null
    uiState.isRunning = false
    const m = messages.value[stream.activeAsstIdx] as AssistantItem | undefined
    if (m?.kind === 'assistant') {
      m.isLoading = false
      if (!m.text) m.text = '_(cancelled)_'
    }
  }

  function sendMessage() {
    const text = prompt.value.trim()
    if (!text || uiState.isRunning || !currentProject.value) return
    const now = new Date().toISOString()
    messages.value.push({
      kind: 'user',
      text,
      timestamp: now,
      uuid: generateUuid(),
      editing: false,
    })
    stream.activeUserIdx = messages.value.length - 1
    stream.activeAsstIdx = messages.value.length
    messages.value.push({
      kind: 'assistant',
      text: '',
      timestamp: now,
      isLoading: true,
      uuid: generateUuid(),
    })
    stream.pendingText = ''
    stream.toolIdx = -1
    prompt.value = ''
    startChatStream(text, true)
  }

  async function resubmitMessage(idx: number, text: string) {
    if (uiState.isRunning || !currentProject.value) return

    const uuids: string[] = []
    while (idx + 1 < messages.value.length && messages.value[idx + 1]?.kind !== 'user') {
      uuids.push(messages.value[idx + 1].uuid)
      messages.value.splice(idx + 1, 1)
    }
    uuids.push(messages.value[idx].uuid)
    messages.value.splice(idx, 1)
    try {
      if (sessionId.value) {
        await api.deleteSessionMessages(sessionId.value, uuids)
      }
    } catch { /* server may not support deletion */ }

    const now = new Date().toISOString()
    messages.value.push({ kind: 'user', text, timestamp: now, uuid: generateUuid(), editing: false })
    stream.activeUserIdx = messages.value.length - 1
    stream.activeAsstIdx = messages.value.length
    messages.value.push({ kind: 'assistant', text: '', timestamp: now, isLoading: true, uuid: generateUuid() })
    stream.pendingText = ''
    stream.toolIdx = -1
    startChatStream(text, false)
  }

  async function confirmEdit(idx: number) {
    const item = messages.value[idx]
    if (item.kind !== 'user' || uiState.isRunning || !currentProject.value) return
    const newText = uiState.editText.trim()
    if (!newText) return
    await resubmitMessage(idx, newText)
    uiState.editText = ''
  }

  async function handleDeleteMsg() {
    if (uiState.deleteMsgIdx < 0 || uiState.deleteMsgIdx >= messages.value.length) return
    const item = messages.value[uiState.deleteMsgIdx]
    uiState.deletingMsg = true

    const uuids: string[] = []
    uuids.push(item.uuid)

    if (item.kind === 'user') {
      for (let i = uiState.deleteMsgIdx + 1; i < messages.value.length; i++) {
        const next = messages.value[i]
        if (next.kind === 'user') break
        if (next.uuid) uuids.push(next.uuid)
      }
    }

    try {
      if (sessionId.value) {
        await api.deleteSessionMessages(sessionId.value, uuids)
      }
      if (item.kind === 'user') {
        while (
          uiState.deleteMsgIdx + 1 < messages.value.length &&
          messages.value[uiState.deleteMsgIdx + 1]?.kind !== 'user'
        ) {
          messages.value.splice(uiState.deleteMsgIdx + 1, 1)
        }
      }
      messages.value.splice(uiState.deleteMsgIdx, 1)
      appStore.showMessage('Message deleted', 'success')
    } catch {
      appStore.showMessage('Failed to delete message', 'error')
    } finally {
      uiState.deletingMsg = false
      uiState.deleteMsgDialog = false
      uiState.deleteMsgIdx = -1
    }
  }

  return {
    sendMessage,
    resubmitMessage,
    confirmEdit,
    handleDeleteMsg,
    cleanupActiveStream,
  }
}
