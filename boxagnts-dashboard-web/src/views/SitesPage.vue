<template>
  <div>
    <div class="d-flex align-center justify-space-between mb-6">
      <div class="d-flex align-center">
        <v-icon size="32" color="primary" class="mr-3">mdi-web</v-icon>
        <h1 class="text-h4 font-weight-bold">Sites</h1>
      </div>
      <v-btn color="primary" prepend-icon="mdi-plus" @click="openAddDialog">
        Add Site
      </v-btn>
    </div>

    <v-row>
      <v-col cols="12" md="6" lg="4" v-for="site in siteStore.sites" :key="site.id">
        <v-card class="fill-height">
          <v-card-item>
            <template #prepend>
              <v-icon :color="site.enable_auth ? 'warning' : 'success'" size="28">
                {{ site.enable_auth ? 'mdi-web-lock' : 'mdi-web' }}
              </v-icon>
            </template>
            <v-card-title>{{ site.title || site.name }}</v-card-title>
            <v-card-subtitle>{{ site.name }}</v-card-subtitle>
          </v-card-item>

          <v-card-text>
            <div v-if="site.description" class="text-body-2 mb-3">{{ site.description }}</div>

            <v-list density="compact" class="pa-0">
              <v-list-item>
                <template #prepend>
                  <v-icon size="16" color="medium-emphasis">mdi-folder</v-icon>
                </template>
                <v-list-item-title class="text-caption">Path</v-list-item-title>
                <template #append>
                  <code class="text-caption">{{ site.path }}</code>
                </template>
              </v-list-item>
              <v-list-item v-if="site.entry_point">
                <template #prepend>
                  <v-icon size="16" color="medium-emphasis">mdi-file-code</v-icon>
                </template>
                <v-list-item-title class="text-caption">Entry Point</v-list-item-title>
                <template #append>
                  <code class="text-caption">{{ site.entry_point }}</code>
                </template>
              </v-list-item>
              <v-list-item v-if="site.component">
                <template #prepend>
                  <v-icon size="16" color="medium-emphasis">mdi-puzzle</v-icon>
                </template>
                <v-list-item-title class="text-caption">Component</v-list-item-title>
                <template #append>
                  <code class="text-caption">{{ site.component }}</code>
                </template>
              </v-list-item>
              <v-list-item v-if="site.enable_auth && site.auth_user">
                <template #prepend>
                  <v-icon size="16" color="medium-emphasis">mdi-account</v-icon>
                </template>
                <v-list-item-title class="text-caption">Auth User</v-list-item-title>
                <template #append>
                  <span class="text-caption">{{ site.auth_user }}</span>
                </template>
              </v-list-item>
            </v-list>

            <div class="d-flex align-center gap-2 mt-2">
              <v-chip :color="site.enable_auth ? 'warning' : 'success'" size="x-small" variant="tonal">
                {{ site.enable_auth ? 'Auth Enabled' : 'Public' }}
              </v-chip>
            </div>
          </v-card-text>

          <v-card-actions>
            <v-switch
              :model-value="site.enabled"
              :label="site.enabled ? 'Enabled' : 'Disabled'"
              color="success"
              density="compact"
              hide-details
              @update:model-value="(v: boolean | null) => handleToggleEnabled(site, !!v)"
            />
            <v-spacer />
            <v-btn variant="tonal" size="small" prepend-icon="mdi-pencil" @click="openEditDialog(site)">
              Edit
            </v-btn>
            <v-spacer />
            <v-btn
              icon="mdi-delete"
              variant="text"
              size="small"
              color="error"
              @click="confirmRemove(site)"
            />
          </v-card-actions>
        </v-card>
      </v-col>
    </v-row>

    <v-skeleton-loader v-if="siteStore.loading" type="card@3" />

    <div v-if="!siteStore.loading && siteStore.sites.length === 0" class="text-center py-12">
      <v-icon size="64" color="medium-emphasis">mdi-web-off</v-icon>
      <p class="text-medium-emphasis mt-4">No sites configured</p>
      <v-btn color="primary" class="mt-4" @click="openAddDialog">Add Site</v-btn>
    </div>

    <!-- Add / Edit Dialog -->
    <v-dialog v-model="showDialog" max-width="600">
      <v-card>
        <v-card-title>{{ editingSite ? 'Edit Site' : 'Add Site' }}</v-card-title>
        <v-card-text>
          <v-row dense>
              <v-col cols="12" md="6">
                <v-text-field
                  v-model="form.name"
                  label="Name"
                  variant="outlined"
                  placeholder="my-site"
                  hint="Unique identifier for the site"
                  persistent-hint
                />
              </v-col>
              <v-col cols="12" md="6">
                <v-text-field
                  v-model="form.title"
                  label="Title"
                  variant="outlined"
                  placeholder="My Site"
                />
              </v-col>
              <v-col cols="12">
                <v-textarea
                  v-model="form.description"
                  label="Description"
                  variant="outlined"
                  rows="2"
                  placeholder="Site description..."
                />
              </v-col>
              <v-col cols="12" md="6">
                <v-menu v-model="pathMenuOpen" activator="parent" location="bottom">
                  <template v-slot:activator="{ props }">
                    <v-text-field
                      v-model="form.path"
                      label="Path"
                      variant="outlined"
                      placeholder="Select folder"
                      append-icon="mdi-folder"
                      readonly
                      v-bind="props"
                    />
                  </template>
                  <v-list v-if="loadingFolders">
                    <v-progress-circular indeterminate color="primary" />
                  </v-list>
                  <v-list v-else-if="folderTree.length === 0">
                    <v-list-item title="No folders available" />
                  </v-list>
                  <v-list v-else density="compact">
                    <template v-for="node in folderTree" :key="node.path">
                      <v-list-group 
                        :model-value="node.path.includes(form.path)" 
                        prepend-icon="mdi-folder"
                      >
                        <template v-slot:activator="{ props }">
                          <v-list-item
                            v-bind="props"
                            :title="node.name"
                            :active="form.path === node.path"
                            :color="form.path === node.path ? 'primary' : undefined"
                            @click.stop="selectPath(node.path); pathMenuOpen = false"
                          />
                        </template>
                        <v-list>
                          <template v-for="child in node.children" :key="child.path">
                            <v-list-group 
                              :model-value="child.path.includes(form.path)"
                              prepend-icon="mdi-folder"
                              v-if="child.children.length > 0"
                            >
                              <template v-slot:activator="{ props }">
                                <v-list-item
                                  v-bind="props"
                                  :title="child.name"
                                  :active="form.path === child.path"
                                  :color="form.path === child.path ? 'primary' : undefined"
                                  @click.stop="selectPath(child.path); pathMenuOpen = false"
                                />
                              </template>
                              <v-list>
                                <template v-for="subChild in child.children" :key="subChild.path">
                                  <v-list-item
                                    :title="subChild.name"
                                    prepend-icon="mdi-folder-outline"
                                    :active="form.path === subChild.path"
                                    :color="form.path === subChild.path ? 'primary' : undefined"
                                    @click.stop="selectPath(subChild.path); pathMenuOpen = false"
                                  />
                                </template>
                              </v-list>
                            </v-list-group>
                            <v-list-item
                              v-else
                              :title="child.name"
                              prepend-icon="mdi-folder-outline"
                              :active="form.path === child.path"
                              :color="form.path === child.path ? 'primary' : undefined"
                              @click.stop="selectPath(child.path); pathMenuOpen = false"
                            />
                          </template>
                        </v-list>
                      </v-list-group>
                    </template>
                  </v-list>
                </v-menu>
              </v-col>
              <v-col cols="12" md="6">
                <v-text-field
                  v-model="form.entry_point"
                  label="Entry Point"
                  variant="outlined"
                  placeholder="index.html"
                />
              </v-col>
              <v-col cols="12" md="6">
                <v-select
                  v-model="form.component"
                  label="Component"
                  variant="outlined"
                  :items="componentOptions"
                  :readonly="true"
                  persistent-hint
                  hint="Only one option available"
                />
              </v-col>
              <v-col cols="12">
                <v-switch
                  v-model="form.enable_auth"
                  label="Enable Authentication"
                  color="primary"
                  hide-details
                  class="mt-1"
                />
              </v-col>
              <v-col cols="12">
                <v-switch
                  v-model="form.enabled"
                  label="Enabled"
                  color="success"
                  hide-details
                />
              </v-col>
              <v-col cols="12" md="6">
                <v-text-field
                  v-model="form.auth_user"
                  label="Auth User"
                  variant="outlined"
                  placeholder="admin"
                  :disabled="!form.enable_auth"
                />
              </v-col>
              <v-col cols="12" md="6">
                <v-text-field
                  v-model="form.auth_pass"
                  label="Auth Password"
                  variant="outlined"
                  type="password"
                  placeholder="Enter password"
                  :disabled="!form.enable_auth"
                />
              </v-col>
            </v-row>
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="showDialog = false">Cancel</v-btn>
          <v-btn
            color="primary"
            @click="handleSave"
            :loading="saving"
            :disabled="!form.name.trim()"
          >
            {{ editingSite ? 'Update' : 'Create' }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Delete Confirmation Dialog -->
    <v-dialog v-model="deleteDialog" max-width="400">
      <v-card>
        <v-card-title>Delete Site</v-card-title>
        <v-card-text>
          <p>
            Are you sure you want to delete
            <strong>{{ deleteTarget?.title || deleteTarget?.name }}</strong>?
          </p>
          <p class="text-caption text-error mt-2">This action cannot be undone.</p>
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="deleteDialog = false">Cancel</v-btn>
          <v-btn color="error" @click="handleDelete" :loading="deleting">Delete</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useSiteStore } from '@/stores/sites'
import { useAppStore } from '@/stores/app'
import { api, type Site, type FolderItem } from '@/api'

const siteStore = useSiteStore()
const appStore = useAppStore()

const showDialog = ref(false)
const editingSite = ref<Site | null>(null)
const saving = ref(false)
const deleteDialog = ref(false)
const deleteTarget = ref<Site | null>(null)
const deleting = ref(false)
const folders = ref<FolderItem[]>([])
const loadingFolders = ref(false)
const pathMenuOpen = ref(false)

// Component options
const componentOptions = ['boxed_static_server_component.wasm']

type SiteForm = Omit<Site, 'id'>

const defaultForm = (): SiteForm => ({
  name: '',
  title: '',
  description: '',
  path: '',
  entry_point: null,
  component: 'boxed_static_server_component.wasm',
  enabled: true,
  enable_auth: null,
  auth_user: null,
  auth_pass: null,
})

const form = ref<SiteForm>(defaultForm())

// Build folder tree structure
interface TreeNode {
  name: string
  path: string
  children: TreeNode[]
}

function buildTree(folderItems: FolderItem[]): TreeNode[] {
  const root: TreeNode[] = []
  const pathMap = new Map<string, TreeNode>()

  // Create all nodes first
  folderItems.forEach(item => {
    const node: TreeNode = { name: item.name, path: item.path, children: [] }
    pathMap.set(item.path, node)
  })

  // Build hierarchy
  folderItems.forEach(item => {
    const node = pathMap.get(item.path)!
    const parts = item.path.split('/')

    if (parts.length === 1) {
      root.push(node)
    } else {
      const parentPath = parts.slice(0, -1).join('/')
      const parent = pathMap.get(parentPath)
      if (parent) {
        parent.children.push(node)
      } else {
        root.push(node)
      }
    }
  })

  return root
}

const folderTree = computed(() => buildTree(folders.value))

async function fetchFolders() {
  loadingFolders.value = true
  try {
    folders.value = await api.getRootSubFolders()
  } catch (e) {
    console.error('Failed to fetch folders:', e)
    folders.value = []
  } finally {
    loadingFolders.value = false
  }
}

function openAddDialog() {
  editingSite.value = null
  form.value = defaultForm()
  showDialog.value = true
  fetchFolders()
}

function openEditDialog(site: Site) {
  editingSite.value = site
  form.value = {
    name: site.name,
    title: site.title,
    description: site.description,
    path: site.path,
    entry_point: site.entry_point,
    component: site.component,
    enabled: site.enabled,
    enable_auth: site.enable_auth,
    auth_user: site.auth_user,
    auth_pass: site.auth_pass,
  }
  showDialog.value = true
  fetchFolders()
}

function selectPath(path: string) {
  form.value.path = path
}

async function handleSave() {
  saving.value = true
  try {
    if (editingSite.value) {
      await siteStore.updateSite(editingSite.value.id, { ...form.value })
      appStore.showMessage('Site updated', 'success')
    } else {
      await siteStore.addSite({ ...form.value })
      appStore.showMessage('Site created', 'success')
    }
    showDialog.value = false
    form.value = defaultForm()
    editingSite.value = null
    // 刷新站点列表
    await siteStore.fetchSites()
  } catch {
    appStore.showMessage('Failed to save site', 'error')
  } finally {
    saving.value = false
  }
}

function confirmRemove(site: Site) {
  deleteTarget.value = site
  deleteDialog.value = true
}

async function handleDelete() {
  if (!deleteTarget.value) return
  deleting.value = true
  try {
    await siteStore.removeSite(deleteTarget.value.id)
    appStore.showMessage('Site deleted', 'success')
    // 刷新站点列表
    await siteStore.fetchSites()
  } catch {
    appStore.showMessage('Failed to delete site', 'error')
  } finally {
    deleting.value = false
    deleteDialog.value = false
    deleteTarget.value = null
  }
}

async function handleToggleEnabled(site: Site, enabled: boolean) {
  try {
    await siteStore.updateSite(site.id, { enabled })
    appStore.showMessage(enabled ? 'Site enabled' : 'Site disabled', 'success')
    // 刷新站点列表
    await siteStore.fetchSites()
  } catch {
    appStore.showMessage('Failed to update site', 'error')
  }
}

onMounted(() => {
  siteStore.fetchSites()
})
</script>
