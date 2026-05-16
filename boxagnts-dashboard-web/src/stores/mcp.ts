import { defineStore } from 'pinia'
import { ref } from 'vue'
import { api, type MCPServer, type ImportResult } from '@/api'

export const useMcpStore = defineStore('mcp', () => {
  const servers = ref<MCPServer[]>([])
  const loading = ref(false)
  const testingServer = ref<string | null>(null)

  async function fetchServers() {
    loading.value = true
    try {
      servers.value = await api.mcpList()
    } catch (e) {
      console.error('Failed to fetch MCP servers:', e)
      servers.value = []
    } finally {
      loading.value = false
    }
  }

  async function addServer(
    name: string,
    transport: string,
    command?: string,
    args: string[] = [],
    env: Record<string, string> = {},
    url?: string,
    scope: string = 'local'
  ) {
    const result = await api.mcpAdd(name, transport, command, args, env, url, scope)
    await fetchServers()
    return result
  }

  async function removeServer(name: string) {
    await api.mcpRemove(name)
    await fetchServers()
  }

  async function testConnection(name: string) {
    testingServer.value = name
    try {
      const result = await api.mcpTestConnection(name)
      return result
    } finally {
      testingServer.value = null
    }
  }

  async function importFromClaudeDesktop(scope: string = 'local'): Promise<ImportResult> {
    const result = await api.mcpAddFromClaudeDesktop(scope)
    await fetchServers()
    return result
  }

  function isServerConnected(server: MCPServer): boolean {
    return server.status?.running ?? false
  }

  return {
    servers,
    loading,
    testingServer,
    fetchServers,
    addServer,
    removeServer,
    testConnection,
    importFromClaudeDesktop,
    isServerConnected,
  }
})
