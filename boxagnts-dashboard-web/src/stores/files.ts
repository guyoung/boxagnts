import { defineStore } from 'pinia'
import { ref } from 'vue'
import fileApi, { type FileItem } from '@/api/fileApi'

export interface TreeNode {
  name: string
  path: string
  is_dir: boolean
  children: TreeNode[]
  expanded: boolean
  loading: boolean
  loaded: boolean
}

export const useFileStore = defineStore('files', () => {
  const treeRoots = ref<TreeNode[]>([])
  const treeLoading = ref(false)
  const currentPath = ref('')
  const items = ref<FileItem[]>([])
  const loading = ref(false)
  const selectedFile = ref<string | null>(null)

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

  function fetchTree() {
    treeRoots.value = [{
      name: 'root',
      path: '',
      is_dir: true,
      children: [],
      expanded: false,
      loading: false,
      loaded: false,
    }]
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
    collapseAllNodes(treeRoots.value)
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

  async function createDirectory(name: string) {
    await fileApi.createDirectory(currentPath.value, name)
    await fetchCurrentItems()
    invalidateTreeForPath(currentPath.value)
  }

  async function uploadFiles(files: File[]) {
    if (!files.length) return
    await fileApi.uploadFiles(currentPath.value, files)
    await fetchCurrentItems()
    invalidateTreeForPath(currentPath.value)
  }

  async function deleteItem(itemPath: string) {
    await fileApi.deleteFile(itemPath)
    await fetchCurrentItems()
    invalidateTreeForPath(currentPath.value)
    const parentPath = itemPath.substring(0, itemPath.lastIndexOf('/'))
    if (parentPath !== currentPath.value) {
      invalidateTreeForPath(parentPath)
    }
  }

  async function renameItem(oldPath: string, newName: string) {
    await fileApi.renameFile(oldPath, newName)
    await fetchCurrentItems()
    invalidateTreeForPath(currentPath.value)
  }

  async function downloadItem(itemPath: string) {
    const result = await fileApi.downloadFile(itemPath)
    fileApi.saveBlob(result.blob, result.fileName)
  }

  function invalidateTreeForPath(path: string) {
    function invalidateNodes(nodes: TreeNode[]): boolean {
      for (const n of nodes) {
        if (n.path === path) {
          n.loaded = false
          n.children = []
          if (n.expanded) {
            n.loading = true
            fileApi.listFiles(path).then(res => {
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
  }

  async function saveFileContent(filePath: string, content: string) {
    const blob = new Blob([content], { type: 'text/plain' })
    const fileName = filePath.split('/').pop() || 'file.txt'
    const parentDir = filePath.substring(0, filePath.lastIndexOf('/'))
    const file = new File([blob], fileName, { type: 'text/plain' })
    await fileApi.uploadFiles(parentDir, [file])
    if (currentPath.value === parentDir) {
      await fetchCurrentItems()
    }
    invalidateTreeForPath(parentDir)
    if (parentDir !== currentPath.value && currentPath.value && !currentPath.value.startsWith(parentDir)) {
      invalidateTreeForPath(currentPath.value)
    }
  }

  return {
    treeRoots,
    treeLoading,
    currentPath,
    items,
    loading,
    selectedFile,
    fetchTree,
    toggleTreeNode,
    collapseAllNodes,
    onTreeItemClick,
    setCurrentPath,
    selectFile,
    clearSelectedFile,
    expandAll,
    collapseAll,
    fetchCurrentItems,
    createDirectory,
    uploadFiles,
    deleteItem,
    renameItem,
    downloadItem,
    saveFileContent,
    refresh: fetchCurrentItems,
    refreshTree: fetchTree,
  }
})
