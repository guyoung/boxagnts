<template>
  <div>
    <div class="d-flex align-center justify-space-between mb-6">
      <div class="d-flex align-center">
        <v-icon size="32" color="primary" class="mr-3">mdi-server-network</v-icon>
        <h1 class="text-h4 font-weight-bold">MCP Servers</h1>
      </div>
      <div class="d-flex gap-3">
        <v-btn variant="tonal" prepend-icon="mdi-cloud-download" @click="importFromDesktop" :loading="importing">
          Import from Claude Desktop
        </v-btn>
        <v-btn color="primary" prepend-icon="mdi-plus" @click="showAddDialog = true">
          Add Server
        </v-btn>
      </div>
    </div>

    <v-row>
      <v-col cols="12" md="6" v-for="server in mcpStore.servers" :key="server.name">
        <v-card class="fill-height">
          <v-card-item>
            <template #prepend>
              <v-icon :color="mcpStore.isServerConnected(server) ? 'success' : 'warning'">
                {{ mcpStore.isServerConnected(server) ? 'mdi-server' : 'mdi-server-off' }}
              </v-icon>
            </template>
            <v-card-title>{{ server.name }}</v-card-title>
            <v-card-subtitle>
              {{ server.command ? `${server.command} ${server.args.join(' ')}` : server.url || 'No endpoint' }}
            </v-card-subtitle>
          </v-card-item>
          <v-card-text>
            <div class="d-flex align-center gap-4">
              <v-chip :color="mcpStore.isServerConnected(server) ? 'success' : 'warning'" size="small" variant="tonal">
                {{ mcpStore.isServerConnected(server) ? 'running' : 'stopped' }}
              </v-chip>
              <v-chip size="small" variant="outlined">
                {{ server.transport }}
              </v-chip>
              <v-chip v-if="server.scope" size="small" variant="outlined">
                {{ server.scope }}
              </v-chip>
            </div>
            <div v-if="server.env && Object.keys(server.env).length" class="mt-3">
              <div class="text-caption text-medium-emphasis mb-1">Environment:</div>
              <div v-for="(val, key) in server.env" :key="key" class="text-caption">
                <code>{{ key }}={{ val }}</code>
              </div>
            </div>
            <div v-if="server.status?.error" class="mt-2">
              <div class="text-caption text-error">{{ server.status.error }}</div>
            </div>
          </v-card-text>
          <v-card-actions>
            <v-btn
              variant="tonal"
              size="small"
              @click="testConnection(server.name)"
              :loading="mcpStore.testingServer === server.name"
            >
              <v-icon start>mdi-connection</v-icon> Test
            </v-btn>
            <v-spacer />
            <v-btn
              icon="mdi-delete"
              variant="text"
              size="small"
              color="error"
              @click="confirmRemove(server)"
            />
          </v-card-actions>
        </v-card>
      </v-col>
    </v-row>

    <v-skeleton-loader v-if="mcpStore.loading" type="card@4" />

    <div v-if="!mcpStore.loading && mcpStore.servers.length === 0" class="text-center py-12">
      <v-icon size="64" color="medium-emphasis">mdi-server-off</v-icon>
      <p class="text-medium-emphasis mt-4">No MCP servers configured</p>
      <v-btn color="primary" class="mt-4" @click="showAddDialog = true">Add Server</v-btn>
    </div>

    <v-dialog v-model="showAddDialog" max-width="600">
      <v-card>
        <v-card-title>Add MCP Server</v-card-title>
        <v-card-text>
          <v-text-field v-model="addForm.name" label="Server Name" variant="outlined" class="mb-3" />
          <v-select
            v-model="addForm.transport"
            label="Transport"
            :items="['stdio', 'sse']"
            variant="outlined"
            class="mb-3"
          />
          <v-text-field v-model="addForm.command" label="Command" variant="outlined" class="mb-3" />
          <v-text-field v-model="addForm.argsText" label="Arguments (comma separated)" variant="outlined" class="mb-3" />
          <v-text-field v-model="addForm.url" label="URL (for SSE)" variant="outlined" class="mb-3" />
          <v-select
            v-model="addForm.scope"
            label="Scope"
            :items="['local', 'project', 'user']"
            variant="outlined"
            class="mb-3"
          />
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="showAddDialog = false">Cancel</v-btn>
          <v-btn color="primary" @click="addServer">Add</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-dialog v-model="removeDialog" max-width="400">
      <v-card>
        <v-card-title>Remove Server</v-card-title>
        <v-card-text>
          Remove <strong>{{ removeTarget?.name }}</strong>?
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="removeDialog = false">Cancel</v-btn>
          <v-btn color="error" @click="handleRemove">Remove</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useMcpStore } from '@/stores/mcp'
import { useAppStore } from '@/stores/app'
import type { MCPServer } from '@/types'

const mcpStore = useMcpStore()
const appStore = useAppStore()
const showAddDialog = ref(false)
const removeDialog = ref(false)
const removeTarget = ref<MCPServer | null>(null)
const importing = ref(false)

const addForm = ref({
  name: '',
  transport: 'stdio',
  command: '',
  argsText: '',
  url: '',
  scope: 'local',
})

function confirmRemove(server: MCPServer) {
  removeTarget.value = server
  removeDialog.value = true
}

async function handleRemove() {
  if (!removeTarget.value) return
  try {
    await mcpStore.removeServer(removeTarget.value.name)
    appStore.showMessage('Server removed', 'success')
  } catch {
    appStore.showMessage('Failed to remove server', 'error')
  }
  removeDialog.value = false
}

async function addServer() {
  try {
    await mcpStore.addServer(
      addForm.value.name,
      addForm.value.transport,
      addForm.value.command || undefined,
      addForm.value.argsText ? addForm.value.argsText.split(',').map(s => s.trim()) : [],
      {},
      addForm.value.url || undefined,
      addForm.value.scope
    )
    appStore.showMessage('Server added', 'success')
    showAddDialog.value = false
    addForm.value = { name: '', transport: 'stdio', command: '', argsText: '', url: '', scope: 'local' }
  } catch {
    appStore.showMessage('Failed to add server', 'error')
  }
}

async function testConnection(name: string) {
  try {
    const result = await mcpStore.testConnection(name)
    appStore.showMessage(
      result || 'Connection test complete',
      'success'
    )
  } catch {
    appStore.showMessage('Connection failed', 'error')
  }
}

async function importFromDesktop() {
  importing.value = true
  try {
    const result = await mcpStore.importFromClaudeDesktop()
    appStore.showMessage(`Imported ${result.imported_count} server(s)`, 'success')
  } catch {
    appStore.showMessage('Failed to import from Claude Desktop', 'error')
  } finally {
    importing.value = false
  }
}

onMounted(() => {
  console.log("McpPage onMounted")
  mcpStore.fetchServers()
})
</script>

<style scoped>
.gap-3 { gap: 12px; }
.gap-4 { gap: 16px; }
</style>
