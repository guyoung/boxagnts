import { a4 as ref } from "./main-gWZPyuWK.js";
function useCrudOperations(api, storeName) {
  const items = ref([]);
  const loading = ref(false);
  async function fetch() {
    loading.value = true;
    try {
      items.value = await api.fetchAll();
    } catch (e) {
      console.error(`Failed to fetch ${storeName}:`, e);
      items.value = [];
    } finally {
      loading.value = false;
    }
  }
  async function add(data) {
    const item = await api.create(data);
    items.value.push(item);
    return item;
  }
  async function update(id, data) {
    const item = await api.update(id, data);
    const idx = items.value.findIndex((i) => i.id === id);
    if (idx >= 0) {
      items.value[idx] = item;
    }
    return item;
  }
  async function remove(id) {
    await api.remove(id);
    items.value = items.value.filter((i) => i.id !== id);
  }
  return { items, loading, fetch, add, update, remove };
}
export {
  useCrudOperations as u
};
