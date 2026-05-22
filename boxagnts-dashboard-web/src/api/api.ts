import { apiCall } from './apiAdapter'

export interface Project {
  id: string
  path: string
  sessions: string[]
  created_at: number
  most_recent_session?: number
}

export interface Session {
  id: string
  title?: string,
  todo_data?: any
  created_at: number
  first_message?: string
  message_uuid?: string
  message_timestamp?: string
}

export interface SessionMessage {
  role: 'user' | 'assistant'
  content: string | ContentBlock[]
  uuid: string
}

export type ContentBlock =
  | { type: 'text'; text: string }
  | { type: 'tool_use'; id: string; name: string; input: Record<string, any> }
  | { type: 'tool_result'; tool_use_id: string; content: string; is_error: boolean }

export interface UsageStats {
  total_cost: number
  total_tokens: number
  total_input_tokens: number
  total_output_tokens: number
  total_cache_creation_tokens: number
  total_cache_read_tokens: number
  total_sessions: number
  by_model: ModelUsage[]
  by_date: DailyUsage[]
  by_project: ProjectUsage[]
}

export interface ModelUsage {
  model: string
  total_cost: number
  total_tokens: number
  input_tokens: number
  output_tokens: number
  cache_creation_tokens: number
  cache_read_tokens: number
  session_count: number
}

export interface DailyUsage {
  date: string
  total_cost: number
  total_tokens: number
  models_used: string[]
}

export interface ProjectUsage {
  project_path: string
  project_name: string
  total_cost: number
  total_tokens: number
  session_count: number
  last_used: string
}

export interface UsageEntry {
  project: string
  timestamp: string
  model: string
  input_tokens: number
  output_tokens: number
  cache_write_tokens: number
  cache_read_tokens: number
  cost: number
}

export interface MCPServer {
  name: string
  transport: string
  command?: string
  args: string[]
  env: Record<string, string>
  url?: string
  scope: string
  is_active: boolean
  status: ServerStatus
}

export interface ServerStatus {
  running: boolean
  error?: string
  last_checked?: number
}

export interface ImportResult {
  imported_count: number
  failed_count: number
  servers: ImportServerResult[]
}

export interface ImportServerResult {
  name: string
  success: boolean
  error?: string
}

export interface ClaudeVersionStatus {
  is_installed: boolean
  version?: string
  output: string
}

export interface ClaudeInstallation {
  path: string
  version?: string
  source: string
  installation_type: string
}

export interface ProviderModel {
  id: string
  name: string
  context_window: number
  max_tokens: number
  temperature: number
}

export interface ConfigProvider {
  id: string
  name: string
  api_base: string
  api_key?: string
  enabled: boolean
  models: ProviderModel[]
  options: Record<string, any>
}

export interface ProviderOption {
  badge: string | null
  category: string
  description: string
  id: string
  title: string
}

export interface Settings {
  system_prompt?: string
  default_model?: string
  default_provider?: string
  allowed_outbound_hosts?: string[]
}

export interface Site {
  id: string
  name: string
  title: string
  description: string
  path: string
  entry_point: string | null
  component: string
  enabled: boolean
  enable_auth: boolean | null
  auth_user: string | null
  auth_pass: string | null
}

export interface FolderItem {
  name: string
  path: string
}

export interface FolderListResponse {
  code: number
  data: {
    folders: FolderItem[]
    path: string
  }
  message: string
}

export interface CronJob {
  id: string
  name: string
  description: string
  cron: string
  enabled: boolean
  timeout: number | null
  prompt: string | null
  model: string | null
  last_run_at: string | null
  last_run_success: boolean | null
}

export interface CronLog {
  id: string
  job_id: string
  job_name: string
  executed_at: string
  success: boolean
  message: string
}

export interface Agent {
  id: string
  name: string
  desc: string
  model: string
  system_prompt: string
  tools: string
  enabled: boolean
}

export interface Skill {
  id: string
  name: string
  description: string
  type: string
  config: string
  enabled: boolean
}

export interface Tool {
  id: string
  name: string
  description: string
  type: string
  config: string
  enabled: boolean
}

function safeCall<T>(fn: () => Promise<T>, fallback: T): () => Promise<T> {
  return async () => {
    try { return await fn() } catch { return fallback }
  }
}

export const api = {

  async chatExecute(projectPath: string, prompt: string, model: string, session_id: string | null): Promise<void> {
    return apiCall('chat_execute', { projectPath, prompt, model, session_id })
  },
  async chatExecuteCancel(session_id: string): Promise<void> {
    return apiCall('chat_execute_cancel', { session_id })
  },


  async getCurrentProject(): Promise<Project> {
    return apiCall<Project>('get_current_project')
  },

  async getSessions(): Promise<Session[]> {
    return apiCall<Session[]>('get_sessions')
  },

  async loadSessionHistory(sessionId: string): Promise<{ messages: SessionMessage[] }> {
    return apiCall('load_session_history', { sessionId })
  },

  async deleteSession(sessionId: string): Promise<any> {
    return apiCall<any>('delete_session', { sessionId })
  },

  async updateSessionTitle(sessionId: string, title: string): Promise<any> {
    return apiCall('update_session_title', { sessionId }, 'POST', { title })
  },

  async deleteSessionMessages(sessionId: string, message_uuids: string[]): Promise<any> {
    return apiCall('delete_session_messages', { sessionId }, 'POST', { messages: message_uuids })
  },

  async clearSessionMessages(sessionId: string): Promise<any> {
    return apiCall<any>('clear_session_messages', { sessionId })
  },

  // File
  




  // --- Usage APIs (mock) ---
  getUsageStats: safeCall(() => apiCall<UsageStats>('get_usage_stats'), {
    total_cost: 0, total_tokens: 0, total_input_tokens: 0, total_output_tokens: 0,
    total_cache_creation_tokens: 0, total_cache_read_tokens: 0, total_sessions: 0,
    by_model: [], by_date: [], by_project: [],
  }),
  async getUsageByDateRange(start: string, end: string): Promise<UsageStats> {
    try { return await apiCall<UsageStats>('get_usage_by_date_range', { startDate: start, endDate: end }) }
    catch {
      return {
        total_cost: 0, total_tokens: 0, total_input_tokens: 0, total_output_tokens: 0,
        total_cache_creation_tokens: 0, total_cache_read_tokens: 0, total_sessions: 0,
        by_model: [], by_date: [], by_project: []
      }
    }
  },
  getSessionStats: safeCall(() => apiCall<ProjectUsage[]>('get_session_stats'), []),
  async getUsageDetails(limit?: number): Promise<UsageEntry[]> {
    try { return await apiCall<UsageEntry[]>('get_usage_details', { limit }) } catch { return [] }
  },

  // --- MCP APIs (mock) ---
  mcpList: safeCall(() => apiCall<MCPServer[]>('mcp_list'), []),
  mcpAdd: (name: string, transport: string, command?: string, args?: string[], env?: Record<string, string>, url?: string, scope?: string) =>
    apiCall('mcp_add', { name, transport, command, args, env, url, scope }),
  mcpRemove: (name: string) => apiCall<string>('mcp_remove', { name }),
  mcpTestConnection: (name: string) => apiCall<string>('mcp_test_connection', { name }),
  mcpAddFromClaudeDesktop: (scope?: string) => apiCall<ImportResult>('mcp_add_from_claude_desktop', { scope }),

  // --- Update Default Model API ---
  async updateDefaultModel(id: string): Promise<void> {
    return apiCall('update_default_model', {}, 'POST', { id })
  },

  // --- Get Available Models API ---
  async getModels(): Promise<string[]> {
    return apiCall<string[]>('get_models')
  },

  // --- Allowed Outbound Hosts APIs ---
  async getAllowedOutboundHosts(): Promise<string[]> {
    const data = await apiCall<{ allowed_outbound_hosts?: string[] }>('get_allowed_outbound_hosts')
    return data.allowed_outbound_hosts || []
  },
  async updateAllowedOutboundHosts(hosts: string[]): Promise<void> {
    return apiCall('update_allowed_outbound_hosts', {}, 'POST', { allowed_outbound_hosts: hosts })
  },

  // --- Config Providers APIs ---
  async getConfigProviders(): Promise<ConfigProvider[]> {
    return apiCall<ConfigProvider[]>('get_config_providers')
  },
  async getConfigProvider(id: string): Promise<ConfigProvider | null> {
    return apiCall<ConfigProvider | null>('get_config_provider', { id })
  },
  async createConfigProvider(data: { id: string; name: string; api_base: string; api_key: string; enabled: boolean }): Promise<ConfigProvider> {
    return apiCall<ConfigProvider>('create_config_provider', {}, 'POST', data)
  },
  async updateConfigProvider(id: string, data: { name: string; api_base: string; api_key: string; enabled: boolean }): Promise<ConfigProvider> {
    return apiCall<ConfigProvider>('update_config_provider', { id }, 'POST', data)
  },
  async deleteConfigProvider(id: string): Promise<void> {
    return apiCall('delete_config_provider', { id }, 'POST')
  },
  async getProviderOptions(): Promise<ProviderOption[]> {
    return apiCall<ProviderOption[]>('get_provider_options')
  },

  // --- Provider Model APIs ---
  async createProviderModel(providerId: string, data: { id: string; name: string }): Promise<ProviderModel> {
    return apiCall<ProviderModel>('create_provider_model', { providerId }, 'POST', data)
  },
  async updateProviderModel(providerId: string, modelId: string, data: { name: string }): Promise<ProviderModel> {
    return apiCall<ProviderModel>('update_provider_model', { providerId, modelId }, 'POST', data)
  },
  async deleteProviderModel(providerId: string, modelId: string): Promise<void> {
    return apiCall('delete_provider_model', { providerId, modelId }, 'POST')
  },

  // --- Agents.md APIs ---
  async getAgentsMd(): Promise<string> {
    const result = await apiCall<{ content?: string }>('get_agents_md')
    return result.content || ''
  },
  async updateAgentsMd(content: string): Promise<void> {
    return apiCall('update_agents_md', {}, 'POST', { content })
  },

  // --- File/Folder APIs ---
  async getRootSubFolders(): Promise<FolderItem[]> {
    await delay(200)
    try {
      const res = await apiCall<FolderListResponse>('get_root_sub_folders')
      if (res && res.data && Array.isArray(res.data.folders)) {
        return res.data.folders
      }
      return []
    } catch {
      return []
    }
  },

  // --- Sites APIs ---
  async getSites(): Promise<Site[]> {
    await delay(200)
    try {
      return await apiCall<Site[]>('get_sites')
    } catch {
      const raw = localStorage.getItem('boxagnts_sites')
      return raw ? JSON.parse(raw) : []
    }
  },
  async getSite(id: string): Promise<Site | null> {
    await delay(100)
    try {
      return await apiCall<Site | null>('get_site', { id })
    } catch {
      const sites = await this.getSites()
      return sites.find(s => s.id === id) || null
    }
  },
  async createSite(data: Omit<Site, 'id'>): Promise<Site> {
    await delay(300)
    try {
      return await apiCall<Site>('create_site', {}, 'POST', data)
    } catch {
      const sites = await this.getSites()
      const site: Site = {
        ...data,
        id: 'site_' + Date.now() + '_' + Math.random().toString(36).slice(2, 8),
      }
      sites.push(site)
      localStorage.setItem('boxagnts_sites', JSON.stringify(sites))
      return site
    }
  },
  async updateSite(id: string, data: Partial<Omit<Site, 'id'>>): Promise<Site> {
    await delay(300)
    try {
      return await apiCall<Site>('update_site', { id }, 'POST', data)
    } catch {
      const sites = await this.getSites()
      const idx = sites.findIndex(s => s.id === id)
      if (idx < 0) throw new Error('Site not found')
      sites[idx] = { ...sites[idx], ...data }
      localStorage.setItem('boxagnts_sites', JSON.stringify(sites))
      return sites[idx]
    }
  },
  async deleteSite(id: string): Promise<void> {
    await delay(200)
    try {
      return await apiCall('delete_site', { id }, 'POST')
    } catch {
      const sites = await this.getSites()
      const filtered = sites.filter(s => s.id !== id)
      localStorage.setItem('boxagnts_sites', JSON.stringify(filtered))
    }
  },

  // --- Crons APIs ---
  async getCrons(): Promise<CronJob[]> {
    await delay(200)
    try {
      return await apiCall<CronJob[]>('get_crons')
    } catch {
      return []
    }
  },
  async getCron(id: string): Promise<CronJob | null> {
    await delay(100)
    try {
      return await apiCall<CronJob | null>('get_cron', { id })
    } catch {
      return null
    }
  },
  async createCron(data: Omit<CronJob, 'id' | 'last_run_at' | 'last_run_success'>): Promise<CronJob> {
    await delay(300)
    return apiCall<CronJob>('create_cron', {}, 'POST', data)
  },
  async updateCron(id: string, data: Partial<Omit<CronJob, 'id'>>): Promise<CronJob> {
    await delay(300)
    return apiCall<CronJob>('update_cron', { id }, 'POST', data)
  },
  async deleteCron(id: string): Promise<void> {
    await delay(200)
    return apiCall('delete_cron', { id }, 'POST')
  },

  // --- Cron Logs APIs ---
  async getCronLogs(jobId: string): Promise<CronLog[]> {
    await delay(150)
    try {
      return await apiCall<CronLog[]>('get_cron_logs', { jobId })
    } catch {
      return []
    }
  },

  // --- Agents APIs (mock with localStorage) ---
  async getAgents(): Promise<Agent[]> {
    await delay(200)
    const raw = localStorage.getItem('boxagnts_agents')
    return raw ? JSON.parse(raw) : []
  },
  async createAgent(data: Omit<Agent, 'id'>): Promise<Agent> {
    await delay(300)
    const agents = await this.getAgents()
    const now = Date.now()
    const agent: Agent = {
      ...data,
      id: 'agent_' + now + '_' + Math.random().toString(36).slice(2, 8),
    }
    agents.push(agent)
    localStorage.setItem('boxagnts_agents', JSON.stringify(agents))
    return agent
  },
  async updateAgent(id: string, data: Partial<Omit<Agent, 'id'>>): Promise<Agent> {
    await delay(300)
    const agents = await this.getAgents()
    const idx = agents.findIndex(a => a.id === id)
    if (idx < 0) throw new Error('Agent not found')
    agents[idx] = { ...agents[idx], ...data }
    localStorage.setItem('boxagnts_agents', JSON.stringify(agents))
    return agents[idx]
  },
  async deleteAgent(id: string): Promise<void> {
    await delay(200)
    const agents = await this.getAgents()
    const filtered = agents.filter(a => a.id !== id)
    localStorage.setItem('boxagnts_agents', JSON.stringify(filtered))
  },

  // --- Skills APIs ---
  async getSkills(): Promise<Skill[]> {
    await delay(200)
    try {
      return await apiCall<Skill[]>('get_skills')
    } catch {
      const raw = localStorage.getItem('boxagnts_skills')
      return raw ? JSON.parse(raw) : []
    }
  },
  async createSkill(data: Omit<Skill, 'id'>): Promise<Skill> {
    await delay(300)
    try {
      return await apiCall<Skill>('create_skill', {}, 'POST', data)
    } catch {
      const skills = await this.getSkills()
      const now = Date.now()
      const skill: Skill = {
        ...data,
        id: 'skill_' + now + '_' + Math.random().toString(36).slice(2, 8),
      }
      skills.push(skill)
      localStorage.setItem('boxagnts_skills', JSON.stringify(skills))
      return skill
    }
  },
  async updateSkill(id: string, data: Partial<Omit<Skill, 'id'>>): Promise<Skill> {
    await delay(300)
    try {
      return await apiCall<Skill>('update_skill', { id }, 'PUT', data)
    } catch {
      const skills = await this.getSkills()
      const idx = skills.findIndex(s => s.id === id)
      if (idx < 0) throw new Error('Skill not found')
      skills[idx] = { ...skills[idx], ...data }
      localStorage.setItem('boxagnts_skills', JSON.stringify(skills))
      return skills[idx]
    }
  },
  async deleteSkill(id: string): Promise<void> {
    await delay(200)
    try {
      return await apiCall('delete_skill', { id }, 'DELETE')
    } catch {
      const skills = await this.getSkills()
      const filtered = skills.filter(s => s.id !== id)
      localStorage.setItem('boxagnts_skills', JSON.stringify(filtered))
    }
  },

  // --- Tools APIs ---
  async getTools(): Promise<Tool[]> {
    await delay(200)
    try {
      return await apiCall<Tool[]>('get_tools')
    } catch {
      const raw = localStorage.getItem('boxagnts_tools')
      return raw ? JSON.parse(raw) : []
    }
  },
  async createTool(data: Omit<Tool, 'id'>): Promise<Tool> {
    await delay(300)
    try {
      return await apiCall<Tool>('create_tool', {}, 'POST', data)
    } catch {
      const tools = await this.getTools()
      const now = Date.now()
      const tool: Tool = {
        ...data,
        id: 'tool_' + now + '_' + Math.random().toString(36).slice(2, 8),
      }
      tools.push(tool)
      localStorage.setItem('boxagnts_tools', JSON.stringify(tools))
      return tool
    }
  },
  async updateTool(id: string, data: Partial<Omit<Tool, 'id'>>): Promise<Tool> {
    await delay(300)
    try {
      return await apiCall<Tool>('update_tool', { id }, 'PUT', data)
    } catch {
      const tools = await this.getTools()
      const idx = tools.findIndex(t => t.id === id)
      if (idx < 0) throw new Error('Tool not found')
      tools[idx] = { ...tools[idx], ...data }
      localStorage.setItem('boxagnts_tools', JSON.stringify(tools))
      return tools[idx]
    }
  },
  async deleteTool(id: string): Promise<void> {
    await delay(200)
    try {
      return await apiCall('delete_tool', { id }, 'DELETE')
    } catch {
      const tools = await this.getTools()
      const filtered = tools.filter(t => t.id !== id)
      localStorage.setItem('boxagnts_tools', JSON.stringify(filtered))
    }
  },
}

function delay(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms))
}
