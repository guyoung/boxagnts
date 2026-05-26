import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import fileApi, { type FileItem, startFileWatcher, type FileChangeEvent, type FileWatcher } from '@/api/fileApi'

export interface TreeNode {
  name: string
  path: string
  is_dir: boolean
  children: TreeNode[]
  expanded: boolean
  loading: boolean
  loaded: boolean
}

export interface ClipboardItem {
  path: string
  name: string
  is_dir: boolean
  mode: 'copy' | 'cut'
}

export interface OpenFileTab {
  path: string
  name: string
  dirty: boolean
}

export const useFileStore = defineStore('files', () => {
  const treeRoots = ref<TreeNode[]>([])
  const treeLoading = ref(false)
  const treeAllExpanded = ref(false)
  const currentPath = ref('')
  const items = ref<FileItem[]>([])
  const loading = ref(false)
  const selectedFile = ref<string | null>(null)
  const openTabs = ref<OpenFileTab[]>([])
  const activeTabPath = ref<string | null>(null)
  const clipboard = ref<ClipboardItem | null>(null)
  const fileWatcher = ref<FileWatcher | null>(null)

  const activeTab = computed(() => {
    if (!activeTabPath.value) return null
    return openTabs.value.find(t => t.path === activeTabPath.value) || null
  })

  function sortItems(list: FileItem[]): FileItem[] {
    return [...list].sort((a, b) => {
      if (a.is_dir !== b.is_dir) return a.is_dir ? -1 : 1
      return a.name.localeCompare(b.name)
    })
  }

  function buildTreeNode(item: FileItem): TreeNode {
    return {
      name: item.name,
      path: item.path,
      is_dir: item.is_dir,
      children: [],
      expanded: false,
      loading: false,
      loaded: false,
    }
  }

  function collectExpandedPaths(nodes: TreeNode[], set: Set<string>) {
    for (const n of nodes) {
      if (n.expanded) {
        set.add(n.path)
      }
      if (n.children.length) {
        collectExpandedPaths(n.children, set)
      }
    }
  }

  async function fetchTreeRoot() {
    treeLoading.value = true
    try {
      const expandedPaths = new Set<string>()
      if (!treeAllExpanded.value) {
        collectExpandedPaths(treeRoots.value, expandedPaths)
      }
      const res = await fileApi.listFiles('')
      const fetched = res.data?.items || []
      const newRoots = sortItems(fetched).map(buildTreeNode)
      const restorePromises: Promise<void>[] = []
      if (treeAllExpanded.value) {
        expandAll()
      } else if (expandedPaths.size > 0) {
        function restoreExpanded(nodes: TreeNode[]) {
          for (const n of nodes) {
            if (expandedPaths.has(n.path)) {
              n.expanded = true
              n.loading = true
              restorePromises.push(
                fileApi.listFiles(n.path).then(childRes => {
                  n.children = sortItems(childRes.data?.items || []).map(buildTreeNode)
                  n.loaded = true
                  n.loading = false
                }).catch(() => {
                  n.loading = false
                })
              )
            }
          }
        }
        restoreExpanded(newRoots)
      }
      treeRoots.value = newRoots
      if (restorePromises.length > 0) {
        await Promise.all(restorePromises)
      }
    } catch (e) {
      console.error('Failed to fetch tree root:', e)
      treeRoots.value = []
    } finally {
      treeLoading.value = false
    }
  }

  function fetchTree() {
    fetchTreeRoot()
  }

  async function toggleTreeNode(node: TreeNode) {
    if (!node.is_dir) {
      return
    }
    if (node.expanded) {
      node.expanded = false
      return
    }
    node.expanded = true
    if (!node.loaded) {
      node.loading = true
      try {
        const res = await fileApi.listFiles(node.path)
        const fetched = res.data?.items || []
        node.children = sortItems(fetched).map(buildTreeNode)
        node.loaded = true
      } catch (e) {
        console.error('Failed to fetch tree children:', e)
      } finally {
        node.loading = false
      }
    }
  }

  function collapseAllNodes(nodes: TreeNode[]) {
    for (const n of nodes) {
      n.expanded = false
      if (n.children.length) collapseAllNodes(n.children)
    }
  }

  function onTreeItemClick(node: TreeNode) {
    if (node.is_dir) {
      toggleTreeNode(node)
    }
  }

  function setCurrentPath(path: string) {
    currentPath.value = path
  }

  function selectFile(path: string | null) {
    selectedFile.value = path
  }

  function clearSelectedFile() {
    selectedFile.value = null
  }

  function expandAll() {
    treeAllExpanded.value = true
    function expand(nodes: TreeNode[]) {
      for (const n of nodes) {
        if (!n.is_dir) continue
        n.expanded = true
        if (!n.loaded) {
          n.loading = true
          fileApi.listFiles(n.path).then(res => {
            n.children = sortItems(res.data?.items || []).map(buildTreeNode)
            n.loaded = true
            n.loading = false
            expand(n.children)
          }).catch(() => {
            n.loading = false
          })
        } else {
          expand(n.children)
        }
      }
    }
    expand(treeRoots.value)
  }

  function collapseAll() {
    treeAllExpanded.value = false
    collapseAllNodes(treeRoots.value)
  }

  function toggleExpandAll() {
    if (treeAllExpanded.value) {
      collapseAll()
    } else {
      expandAll()
    }
  }

  async function fetchCurrentItems() {
    loading.value = true
    try {
      const res = await fileApi.listFiles(currentPath.value)
      items.value = sortItems(res.data?.items || [])
    } catch (e) {
      console.error('Failed to fetch files:', e)
      items.value = []
    } finally {
      loading.value = false
    }
  }

  async function checkNameConflict(parentPath: string, name: string): Promise<boolean> {
    const res = await fileApi.listFiles(parentPath || '')
    const items = res.data?.items || []
    return items.some(item => item.name === name)
  }

  async function createDirectory(parentPath: string, name: string) {
    if (await checkNameConflict(parentPath, name)) {
      throw new Error(`"${name}" already exists`)
    }
    await fileApi.createDirectory(parentPath, name)
    invalidateTreeForPath(parentPath)
  }

  async function createFile(parentPath: string, name: string) {
    if (await checkNameConflict(parentPath, name)) {
      throw new Error(`"${name}" already exists`)
    }
    const filePath = parentPath ? parentPath + '/' + name : name
    const blob = new Blob([''], { type: 'text/plain' })
    const file = new File([blob], name, { type: 'text/plain' })
    await fileApi.uploadFiles(parentPath, [file])
    invalidateTreeForPath(parentPath)
    return filePath
  }

  async function uploadFiles(parentPath: string, files: File[]) {
    if (!files.length) return
    await fileApi.uploadFiles(parentPath, files)
    invalidateTreeForPath(parentPath)
  }

  async function deleteItem(itemPath: string) {
    await fileApi.deleteFile(itemPath)
    const parentPath = itemPath.substring(0, itemPath.lastIndexOf('/'))
    if (currentPath.value === itemPath || currentPath.value.startsWith(itemPath + '/')) {
      currentPath.value = parentPath
    }
    invalidateTreeForPath(parentPath)
    if (parentPath !== currentPath.value) {
      invalidateTreeForPath(currentPath.value)
    }
    closeOpenFile(itemPath)
  }

  async function renameItem(oldPath: string, newName: string) {
    await fileApi.renameFile(oldPath, newName)
    const parentPath = oldPath.substring(0, oldPath.lastIndexOf('/'))
    invalidateTreeForPath(parentPath)
    invalidateTreeForPath(currentPath.value)
  }

  async function downloadItem(itemPath: string) {
    const result = await fileApi.downloadFile(itemPath)
    fileApi.saveBlob(result.blob, result.fileName)
  }

  async function invalidateTreeForPath(path: string): Promise<void> {
    if (path === '' || path === '/') {
      await fetchTreeRoot()
      return
    }
    let loadingPromise: Promise<void> = Promise.resolve()

    function invalidateNodes(nodes: TreeNode[]): boolean {
      for (const n of nodes) {
        if (n.path === path) {
          n.loaded = false
          n.children = []
          if (n.expanded) {
            n.loading = true
            loadingPromise = fileApi.listFiles(path).then(res => {
              n.children = sortItems(res.data?.items || []).map(buildTreeNode)
              n.loaded = true
              n.loading = false
            }).catch(() => {
              n.loading = false
            })
          }
          return true
        }
        if (n.children.length && invalidateNodes(n.children)) return true
      }
      return false
    }
    invalidateNodes(treeRoots.value)
    await loadingPromise
  }

  async function saveFileContent(filePath: string, content: string) {
    const blob = new Blob([content], { type: 'text/plain' })
    const fileName = filePath.split('/').pop() || 'file.txt'
    const parentDir = filePath.substring(0, filePath.lastIndexOf('/'))
    const file = new File([blob], fileName, { type: 'text/plain' })
    await fileApi.uploadFiles(parentDir, [file])
    invalidateTreeForPath(parentDir)
    const tab = openTabs.value.find(t => t.path === filePath)
    if (tab) {
      tab.dirty = false
    }
  }

  function setClipboard(item: ClipboardItem) {
    clipboard.value = item
  }

  function clearClipboard() {
    clipboard.value = null
  }

  async function pasteFile(targetDir: string, newName?: string) {
    if (!clipboard.value) return
    const { path: srcPath, mode } = clipboard.value
    const baseName = newName || srcPath.split('/').pop() || srcPath

    if (mode === 'cut') {
      await fileApi.moveFile(srcPath, targetDir, baseName)
      const srcParentPath = srcPath.substring(0, srcPath.lastIndexOf('/'))
      if (currentPath.value === srcPath || currentPath.value.startsWith(srcPath + '/')) {
        currentPath.value = srcParentPath
      }
      await invalidateTreeForPath(srcParentPath)
      closeOpenFile(srcPath)
    } else {
      await fileApi.copyFile(srcPath, targetDir, baseName)
    }
    await invalidateTreeForPath(targetDir)
    clearClipboard()
  }

  function openFile(filePath: string) {
    if (!filePath) return
    const existing = openTabs.value.find(t => t.path === filePath)
    if (existing) {
      activeTabPath.value = filePath
      return
    }
    openTabs.value.push({
      path: filePath,
      name: filePath.split('/').pop() || filePath,
      dirty: false,
    })
    activeTabPath.value = filePath
  }

  function closeOpenFile(filePath: string) {
    const idx = openTabs.value.findIndex(t => t.path === filePath)
    if (idx === -1) return
    openTabs.value.splice(idx, 1)
    if (activeTabPath.value === filePath) {
      if (openTabs.value.length > 0) {
        activeTabPath.value = openTabs.value[Math.min(idx, openTabs.value.length - 1)].path
      } else {
        activeTabPath.value = null
      }
    }
  }

  function setActiveOpenFile(filePath: string | null) {
    activeTabPath.value = filePath
  }

  function setTabDirty(filePath: string, isDirty: boolean) {
    const tab = openTabs.value.find(t => t.path === filePath)
    if (tab) {
      tab.dirty = isDirty
    }
  }

  function isImageFile(filePath: string): boolean {
    const ext = filePath.split('.').pop()?.toLowerCase() || ''
    return ['png', 'jpg', 'jpeg', 'gif', 'svg', 'webp', 'bmp', 'ico'].includes(ext)
  }

  function getParentPath(filePath: string): string {
    const idx = filePath.lastIndexOf('/')
    return idx >= 0 ? filePath.substring(0, idx) : ''
  }

  function handleFileChange(event: FileChangeEvent) {
    const isRename = event.kind === 'Modify(Name(Both))'

    if (isRename) {
      if (event.old_path) {
        invalidateTreeForPath(getParentPath(event.old_path))
      }
      invalidateTreeForPath(getParentPath(event.path))
    } else {
      invalidateTreeForPath(getParentPath(event.path))
    }

    if (currentPath.value) {
      invalidateTreeForPath(currentPath.value)
    }
  }

  function setupFileWatcher() {
    if (fileWatcher.value) return
    fileWatcher.value = startFileWatcher(handleFileChange)
  }

  function teardownFileWatcher() {
    fileWatcher.value?.stop()
    fileWatcher.value = null
  }

  return {
    treeRoots,
    treeLoading,
    treeAllExpanded,
    currentPath,
    items,
    loading,
    selectedFile,
    openTabs,
    activeTabPath,
    activeTab,
    clipboard,
    fetchTree,
    fetchTreeRoot,
    toggleTreeNode,
    collapseAllNodes,
    onTreeItemClick,
    setCurrentPath,
    selectFile,
    clearSelectedFile,
    expandAll,
    collapseAll,
    toggleExpandAll,
    fetchCurrentItems,
    createDirectory,
    createFile,
    uploadFiles,
    checkNameConflict,
    deleteItem,
    renameItem,
    downloadItem,
    saveFileContent,
    setClipboard,
    clearClipboard,
    pasteFile,
    openFile,
    closeOpenFile,
    setActiveOpenFile,
    setTabDirty,
    isImageFile,
    setupFileWatcher,
    teardownFileWatcher,
    refresh: fetchCurrentItems,
    refreshTree: fetchTreeRoot,
  }
})
