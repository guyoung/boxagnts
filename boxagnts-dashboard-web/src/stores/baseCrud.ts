import { ref, type Ref } from 'vue'

interface CrudApi<T, CreateData, UpdateData> {
  fetchAll: () => Promise<T[]>
  create: (data: CreateData) => Promise<T>
  update: (id: string, data: UpdateData) => Promise<T>
  remove: (id: string) => Promise<void>
}

export function useCrudOperations<T extends { id: string }, CreateData, UpdateData>(
  api: CrudApi<T, CreateData, UpdateData>,
  storeName: string
) {
  const items = ref<T[]>([]) as Ref<T[]>
  const loading = ref(false)

  async function fetch() {
    loading.value = true
    try {
      items.value = await api.fetchAll()
    } catch (e) {
      console.error(`Failed to fetch ${storeName}:`, e)
      items.value = []
    } finally {
      loading.value = false
    }
  }

  async function add(data: CreateData): Promise<T> {
    const item = await api.create(data)
    items.value.push(item)
    return item
  }

  async function update(id: string, data: UpdateData): Promise<T> {
    const item = await api.update(id, data)
    const idx = items.value.findIndex(i => i.id === id)
    if (idx >= 0) {
      items.value[idx] = item
    }
    return item
  }

  async function remove(id: string) {
    await api.remove(id)
    items.value = items.value.filter(i => i.id !== id)
  }

  return { items, loading, fetch, add, update, remove }
}
