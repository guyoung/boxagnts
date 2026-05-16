const API_BASE_URL = '/dashboard/api'

export interface FileItem {
    name: string
    path: string
    is_dir: boolean
    size: number
    modified?: string | null
}

export interface ListFilesData {
    path: string
    items: FileItem[]
}

export interface ApiResponse<T> {
    code: number
    message: string
    data: T
}

export interface CreateDirectoryData {
    path: string
    name: string
}

export interface UploadedFileItem {
    name: string
    path: string
    size: number
}

export interface UploadFilesData {
    files: UploadedFileItem[]
}

export interface DeleteData {
    path: string
}

export interface RenameData {
    old_path: string
    new_path: string
    is_dir: boolean
}

export interface DownloadFileResult {
    blob: Blob
    fileName: string
}

type RequestError = Error & {
    status?: number
    data?: unknown
}

async function parseResponse(response: Response): Promise<unknown> {
    const contentType = response.headers.get('content-type') || ''

    if (contentType.includes('application/json')) {
        return response.json()
    }

    return response.blob()
}

async function request<T>(url: string, options: RequestInit = {}): Promise<T> {
    const response = await fetch(`${API_BASE_URL}${url}`, options)
    const data = await parseResponse(response)

    if (!response.ok) {
        const message =
            typeof data === 'object' &&
                data !== null &&
                'message' in data &&
                typeof (data as { message?: unknown }).message === 'string'
                ? (data as { message: string }).message
                : `Request failed with status ${response.status}`

        const error = new Error(message) as RequestError
        error.status = response.status
        error.data = data
        throw error
    }

    return data as T
}

function buildQuery(params: Record<string, string | undefined | null>): string {
    const searchParams = new URLSearchParams()

    Object.entries(params).forEach(([key, value]) => {
        if (value !== undefined && value !== null && value !== '') {
            searchParams.set(key, value)
        }
    })

    const queryString = searchParams.toString()
    return queryString ? `?${queryString}` : ''
}

export async function listFiles(path = ''): Promise<ApiResponse<ListFilesData>> {
    return request<ApiResponse<ListFilesData>>(`/files${buildQuery({ path })}`, {
        method: 'GET'
    })
}

export async function createDirectory(
    path = '',
    name: string
): Promise<ApiResponse<CreateDirectoryData>> {
    return request<ApiResponse<CreateDirectoryData>>('/files/mkdir', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            path,
            name
        })
    })
}

export async function uploadFiles(
    path = '',
    files: File[] = []
): Promise<ApiResponse<UploadFilesData>> {
    const formData = new FormData()

    for (const file of files) {
        formData.append('file', file)
    }

    return request<ApiResponse<UploadFilesData>>(`/files/upload${buildQuery({ path })}`, {
        method: 'POST',
        body: formData
    })
}

export async function deleteFile(path: string): Promise<ApiResponse<DeleteData>> {
    return request<ApiResponse<DeleteData>>('/files/delete', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            path
        })
    })
}

export async function renameFile(
    path: string,
    newName: string
): Promise<ApiResponse<RenameData>> {
    return request<ApiResponse<RenameData>>('/files/rename', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            path,
            new_name: newName
        })
    })
}

export async function downloadFile(path: string): Promise<DownloadFileResult> {
    const response = await fetch(`${API_BASE_URL}/files/download${buildQuery({ path })}`, {
        method: 'GET'
    })

    if (!response.ok) {
        const contentType = response.headers.get('content-type') || ''
        let errorData: unknown = null

        if (contentType.includes('application/json')) {
            errorData = await response.json()
        }

        const message =
            typeof errorData === 'object' &&
                errorData !== null &&
                'message' in errorData &&
                typeof (errorData as { message?: unknown }).message === 'string'
                ? (errorData as { message: string }).message
                : `Download failed with status ${response.status}`

        const error = new Error(message) as RequestError
        error.status = response.status
        error.data = errorData
        throw error
    }

    const blob = await response.blob()
    const contentDisposition = response.headers.get('content-disposition') || ''

    let fileName = 'download.bin'
    const match = contentDisposition.match(/filename="(.+?)"/)

    if (match && match[1]) {
        fileName = match[1]
    }

    return {
        blob,
        fileName
    }
}

export function saveBlob(blob: Blob, fileName: string): void {
    const objectUrl = window.URL.createObjectURL(blob)
    const link = document.createElement('a')

    link.href = objectUrl
    link.download = fileName
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)

    window.URL.revokeObjectURL(objectUrl)
}

const fileApi = {
    listFiles,
    createDirectory,
    uploadFiles,
    deleteFile,
    renameFile,
    downloadFile,
    saveBlob
}

export default fileApi

