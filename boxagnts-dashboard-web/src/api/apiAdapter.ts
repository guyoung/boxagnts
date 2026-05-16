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

/**
 * Make a REST API call to our web server
 */
async function restApiCall<T>(endpoint: string, params?: any): Promise<T> {
  // First handle path parameters in the endpoint string
  let processedEndpoint = endpoint;
  
  if (params) {
    Object.keys(params).forEach(key => {
      // Try different case variations for the placeholder
      const placeholders = [
        `{${key}}`,
        `{${key.charAt(0).toLowerCase() + key.slice(1)}}`,
        `{${key.charAt(0).toUpperCase() + key.slice(1)}}`
      ];
      
      placeholders.forEach(placeholder => {
        if (processedEndpoint.includes(placeholder)) {     
          processedEndpoint = processedEndpoint.replace(placeholder, encodeURIComponent(String(params[key])));
        }
      });
    });
  } 
  
  const url = new URL(processedEndpoint, window.location.origin);
  
  // Add remaining params as query parameters for GET requests (if no placeholders remain)
  if (params && !processedEndpoint.includes('{')) {
    Object.keys(params).forEach(key => {
      // Only add as query param if it wasn't used as a path param
      if (!endpoint.includes(`{${key}}`) && 
          !endpoint.includes(`{${key.charAt(0).toLowerCase() + key.slice(1)}}`) &&
          !endpoint.includes(`{${key.charAt(0).toUpperCase() + key.slice(1)}}`) &&
          params[key] !== undefined && 
          params[key] !== null) {
        url.searchParams.append(key, String(params[key]));
      }
    });
  }

  try {
    const response = await fetch(url.toString(), {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    const result: ApiResponse<T> = await response.json();
    
    if (!result.success) {
      throw new Error(result.error || 'API call failed');
    }

    return result.data as T;
  } catch (error) {
    console.error(`REST API call failed for ${endpoint}:`, error);
    throw error;
  }
}



/**
 * Map Tauri command names to REST API endpoints
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
  };

  const endpoint = commandToEndpoint[command];
  if (!endpoint) {
    console.warn(`Unknown command: ${command}, falling back to generic endpoint`);
    return `/dashboard/api/unknown/${command}`;
  }

  return endpoint;
}


/**
 * Handle streaming commands via WebSocket in web mode
 */
async function handleStreamingCommand<T>(command: string, params?: any): Promise<T> {
  return new Promise((resolve, reject) => {
    // Use wss:// for HTTPS connections (e.g., ngrok), ws:// for HTTP (localhost)
    const wsProtocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const wsUrl = `${wsProtocol}//${window.location.host}/dashboard/ws`;

    const ws = new WebSocket(wsUrl);
    
    ws.onopen = () => {      
      // Send execution request
      const request = {
        command_type: command, 
        project_path: params?.projectPath || '',
        prompt: params?.prompt || '',
        model: params?.model || '',
        session_id: params?.session_id ?? params?.sessionId,
      };      

      // console.log(`[TRACE] Request JSON:`, JSON.stringify(request));
      
      ws.send(JSON.stringify(request));
      // console.log(`[TRACE] WebSocket request sent`);
    };
    
    ws.onmessage = (event) => {
      try {
        const message = JSON.parse(event.data);
        
        if (message.type === 'start') {
          // console.log(`[TRACE] Start message: ${message.message}`);
        } else if (message.type === 'output') {
          // console.log(`[TRACE] Output message, message.content`);
          
          const content = typeof message.content === 'string' 
            ? JSON.parse(message.content) 
            : message.content;
          
          const customEvent = new CustomEvent('chat-output', {
            detail: {
              content: content,
              session_id: message.session_id,
            }
          });

          window.dispatchEvent(customEvent);
        } else if (message.type === 'completion') {
          // console.log(`[TRACE] Completion message:`, message);
          
          // Dispatch chat-complete event for UI state management
          const completeEvent = new CustomEvent('chat-complete', {
            detail: {
              result: message.result
            }
          });

          window.dispatchEvent(completeEvent);
          
          ws.close();

          if (message.status === 'success') {
            resolve({} as T); // Return empty object for now
          } else {
            reject(new Error(message.error || 'Execution failed'));
          }
        } else if (message.type === 'error') {
          console.log(`[TRACE] Error message:`, message);
          
          // Dispatch chat-error event for UI error handling
          const errorEvent = new CustomEvent('chat-error', {
            detail: message.message || 'Unknown error'
          });

          window.dispatchEvent(errorEvent);
          
          reject(new Error(message.message || 'Unknown error'));
        } else {
          console.log(`[TRACE] Unknown message type: ${message.type}`);
        }
      } catch (e) {
        console.error('[TRACE] Failed to parse WebSocket message:', e);
      }
    };
    
    ws.onerror = (error) => {
      console.error('[TRACE] WebSocket error:', error);
      
      // Dispatch chat-error event for connection errors
      const errorEvent = new CustomEvent('chat-error', {
        detail: 'WebSocket connection failed'
      });

      window.dispatchEvent(errorEvent);
      
      reject(new Error('WebSocket connection failed'));
    };
    
    ws.onclose = (event) => {      
      // If connection closed unexpectedly (not a normal close), dispatch cancelled event
      if (event.code !== 1000 && event.code !== 1001) {
        const cancelEvent = new CustomEvent('chat-complete', {
          detail: false // false indicates cancellation/failure
        });   
        window.dispatchEvent(cancelEvent);
      }
    };
  });
}

/**
 * Unified API adapter that works in web environments
 */
export async function apiCall<T>(command: string, params?: any): Promise<T> {  
  // Special handling for commands that use streaming/events
  const streamingCommands = ['chat_execute', 'chat_execute_cancel'];
  if (streamingCommands.includes(command)) {
    return handleStreamingCommand<T>(command, params);
  }
  
  // REST endpoints
  const endpoint = mapCommandToEndpoint(command, params);
  return await restApiCall<T>(endpoint, params);
}