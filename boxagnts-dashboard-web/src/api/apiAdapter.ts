/**
 * API Adapter - Compatibility layer for Web environments
 */


/**
 * Response wrapper for REST API calls
 */
interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}

function resolvePlaceholders(endpoint: string, params: Record<string, any>): string {
  let result = endpoint
  Object.keys(params).forEach(key => {
    const placeholders = [
      `{${key}}`,
      `{${key.charAt(0).toLowerCase() + key.slice(1)}}`,
      `{${key.charAt(0).toUpperCase() + key.slice(1)}}`
    ]
    placeholders.forEach(ph => {
      if (result.includes(ph)) {
        result = result.replace(ph, encodeURIComponent(String(params[key])))
      }
    })
  })
  return result
}

function buildQueryParams(url: URL, endpoint: string, params: Record<string, any>) {
  Object.keys(params).forEach(key => {
    const usedAsPlaceholder =
      endpoint.includes(`{${key}}`) ||
      endpoint.includes(`{${key.charAt(0).toLowerCase() + key.slice(1)}}`) ||
      endpoint.includes(`{${key.charAt(0).toUpperCase() + key.slice(1)}}`)
    if (!usedAsPlaceholder && params[key] !== undefined && params[key] !== null) {
      url.searchParams.append(key, String(params[key]))
    }
  })
}

/**
 * Make a REST API call to our web server.
 * Supports GET, POST, PUT, DELETE methods.
 */
async function restApiCall<T>(
  endpoint: string,
  params?: Record<string, any>,
  method: string = 'GET',
  body?: any
): Promise<T> {
  const processedEndpoint = params ? resolvePlaceholders(endpoint, params) : endpoint
  const url = new URL(processedEndpoint, window.location.origin)

  const fetchOptions: RequestInit = {
    method,
    headers: { 'Content-Type': 'application/json' },
  }

  if (method === 'GET') {
    if (params) buildQueryParams(url, endpoint, params)
  } else if (body !== undefined) {
    fetchOptions.body = JSON.stringify(body)
  }

  try {
    const response = await fetch(url.toString(), fetchOptions)

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`)
    }

    const contentType = response.headers.get('content-type') || ''
    if (contentType.includes('application/json')) {
      const json = await response.json()
      if (json && typeof json === 'object' && 'success' in json) {
        if (!json.success) {
          throw new Error((json as ApiResponse<T>).error || 'API call failed')
        }
        return (json as ApiResponse<T>).data as T
      }
      return json as T
    }

    return (await response.json()) as T
  } catch (error) {
    console.error(`REST API call failed for ${method} ${endpoint}:`, error)
    throw error
  }
}



/**
 * Map command names to REST API endpoints.
 */
function mapCommandToEndpoint(command: string, _params?: any): string {
  const commandToEndpoint: Record<string, string> = {
    // Project
    'get_current_project': '/dashboard/api/project',

    // Session
    'get_sessions': '/dashboard/api/sessions',
    'load_session_history': '/dashboard/api/sessions/{sessionId}',
    'delete_session': '/dashboard/api/delete_session/{sessionId}',
    'clear_session_messages': '/dashboard/api/clear_session_messages/{sessionId}',
    'update_session_title': '/dashboard/api/update_session_title/{sessionId}',
    'delete_session_messages': '/dashboard/api/delete_session_messages/{sessionId}',

    // Usage
    'get_usage_stats': '/dashboard/api/usage/stats',
    'get_usage_by_date_range': '/dashboard/api/usage/by_date',
    'get_session_stats': '/dashboard/api/usage/session_stats',
    'get_usage_details': '/dashboard/api/usage/details',

    // MCP
    'mcp_list': '/dashboard/api/mcp/servers',
    'mcp_add': '/dashboard/api/mcp/servers',
    'mcp_remove': '/dashboard/api/mcp/servers/{name}',
    'mcp_test_connection': '/dashboard/api/mcp/servers/{name}/test',
    'mcp_add_from_claude_desktop': '/dashboard/api/mcp/import',

    // Config
    'update_default_model': '/dashboard/api/config/update_default_model',
    'get_models': '/dashboard/api/config/get_models',
    'get_allowed_outbound_hosts': '/dashboard/api/config/get_allowed_outbound_hosts',
    'update_allowed_outbound_hosts': '/dashboard/api/config/update_allowed_outbound_hosts',

    // Config Providers
    'get_config_providers': '/dashboard/api/config/providers',
    'get_config_provider': '/dashboard/api/config/providers/{id}',
    'create_config_provider': '/dashboard/api/config/providers/create_provider',
    'update_config_provider': '/dashboard/api/config/providers/update_provider/{id}',
    'delete_config_provider': '/dashboard/api/config/providers/delete_provider/{id}',
    'get_provider_options': '/dashboard/api/config/provider_options',

    // Provider Models
    'create_provider_model': '/dashboard/api/config/providers/{providerId}/create_model',
    'update_provider_model': '/dashboard/api/config/providers/{providerId}/update_model/{modelId}',
    'delete_provider_model': '/dashboard/api/config/providers/{providerId}/delete_model/{modelId}',

    // Agents.md
    'get_agents_md': '/dashboard/api/config/get_agents_md',
    'update_agents_md': '/dashboard/api/config/update_agents_md',

    // Files/Folders
    'get_root_sub_folders': '/dashboard/api/files/root_sub_folders',

    // Sites
    'get_sites': '/dashboard/api/site/sites',
    'get_site': '/dashboard/api/site/sites/{id}',
    'create_site': '/dashboard/api/site/sites/create_site',
    'update_site': '/dashboard/api/site/sites/update_site/{id}',
    'delete_site': '/dashboard/api/site/sites/delete_site/{id}',

    // Crons
    'get_crons': '/dashboard/api/cron/jobs',
    'get_cron': '/dashboard/api/cron/jobs/{id}',
    'create_cron': '/dashboard/api/cron/jobs/create_job',
    'update_cron': '/dashboard/api/cron/jobs/update_job/{id}',
    'delete_cron': '/dashboard/api/cron/jobs/delete_job/{id}',
    'get_cron_logs': '/dashboard/api/cron/jobs/{jobId}/logs',

    // Skills
    'get_skills': '/dashboard/api/skills',
    'create_skill': '/dashboard/api/skills',
    'update_skill': '/dashboard/api/skills/{id}',
    'delete_skill': '/dashboard/api/skills/{id}',

    // Tools
    'get_tools': '/dashboard/api/tools',
    'create_tool': '/dashboard/api/tools',
    'update_tool': '/dashboard/api/tools/{id}',
    'delete_tool': '/dashboard/api/tools/{id}',
  }

  const endpoint = commandToEndpoint[command]
  if (!endpoint) {
    console.warn(`Unknown command: ${command}, falling back to generic endpoint`)
    return `/dashboard/api/unknown/${command}`
  }

  return endpoint
}


/**
 * Handle streaming commands via WebSocket in web mode
 */
async function handleStreamingCommand<T>(command: string, params?: any): Promise<T> {
  return new Promise((resolve, reject) => {
    const wsProtocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:'
    const wsUrl = `${wsProtocol}//${window.location.host}/dashboard/ws`

    const ws = new WebSocket(wsUrl)

    ws.onopen = () => {
      const request = {
        command_type: command,
        project_path: params?.projectPath || '',
        prompt: params?.prompt || '',
        model: params?.model || '',
        session_id: params?.session_id ?? params?.sessionId,
      }

      ws.send(JSON.stringify(request))
    }

    ws.onmessage = (event) => {
      try {
        const message = JSON.parse(event.data)

        if (message.type === 'start') {
        } else if (message.type === 'output') {
          const content = typeof message.content === 'string'
            ? JSON.parse(message.content)
            : message.content

          const customEvent = new CustomEvent('chat-output', {
            detail: {
              content: content,
              session_id: message.session_id,
            }
          })

          window.dispatchEvent(customEvent)
        } else if (message.type === 'completion') {
          const completeEvent = new CustomEvent('chat-complete', {
            detail: {
              result: message.result
            }
          })

          window.dispatchEvent(completeEvent)

          ws.close()

          if (message.status === 'success') {
            resolve({} as T)
          } else {
            reject(new Error(message.error || 'Execution failed'))
          }
        } else if (message.type === 'error') {
          const errorEvent = new CustomEvent('chat-error', {
            detail: message.message || 'Unknown error'
          })

          window.dispatchEvent(errorEvent)

          reject(new Error(message.message || 'Unknown error'))
        }
      } catch (e) {
        console.error('[TRACE] Failed to parse WebSocket message:', e)
      }
    }

    ws.onerror = (error) => {
      console.error('[TRACE] WebSocket error:', error)

      const errorEvent = new CustomEvent('chat-error', {
        detail: 'WebSocket connection failed'
      })

      window.dispatchEvent(errorEvent)

      reject(new Error('WebSocket connection failed'))
    }

    ws.onclose = (event) => {
      if (event.code !== 1000 && event.code !== 1001) {
        const cancelEvent = new CustomEvent('chat-complete', {
          detail: false
        })
        window.dispatchEvent(cancelEvent)
      }
    }
  })
}

/**
 * Unified API adapter that works in web environments.
 *
 * @param command   Command name (mapped to REST endpoint via mapCommandToEndpoint)
 * @param params    Path/query parameters for GET; or path params + request body for POST/PUT
 * @param method    HTTP method (default 'GET')
 * @param body      Request body for POST/PUT/DELETE
 */
export async function apiCall<T>(
  command: string,
  params?: Record<string, any>,
  method: string = 'GET',
  body?: any
): Promise<T> {
  const streamingCommands = ['chat_execute', 'chat_execute_cancel']
  if (streamingCommands.includes(command)) {
    return handleStreamingCommand<T>(command, params)
  }

  const endpoint = mapCommandToEndpoint(command, params)
  return await restApiCall<T>(endpoint, params, method, body)
}
