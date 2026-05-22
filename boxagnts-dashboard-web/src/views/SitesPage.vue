<template>
  <div class="sites-page">
    <div class="page-header d-flex align-center justify-space-between mb-6">
      <div class="d-flex align-center">
        <div class="header-icon-wrapper mr-3">
          <v-icon size="28" color="white">mdi-web</v-icon>
        </div>
        <div>
          <h1 class="text-h4 font-weight-bold mb-0">Sites</h1>
          <p class="text-body-2 text-medium-emphasis mt-1">Manage your static website deployments</p>
        </div>
      </div>
      <v-btn
        color="primary"
        size="large"
        prepend-icon="mdi-plus"
        variant="elevated"
        class="add-btn"
        @click="openAddDialog"
      >
        Add Site
      </v-btn>
    </div>

    <div class="stats-bar d-flex align-center pa-4 mb-6 rounded-lg" v-if="!siteStore.loading && siteStore.sites.length > 0">
      <div class="stat-item d-flex align-center">
        <v-icon size="20" color="primary" class="mr-2">mdi-web</v-icon>
        <span class="text-body-2 font-weight-medium">{{ siteStore.sites.length }} site{{ siteStore.sites.length > 1 ? 's' : '' }}</span>
      </div>
      <v-divider vertical class="mx-4" />
      <div class="stat-item d-flex align-center">
        <v-icon size="20" color="success" class="mr-2">mdi-check-circle</v-icon>
        <span class="text-body-2 font-weight-medium">{{ siteStore.sites.filter(s => s.enabled).length }} enabled</span>
      </div>
      <v-divider vertical class="mx-4" />
      <div class="stat-item d-flex align-center">
        <v-icon size="20" color="warning" class="mr-2">mdi-shield-lock</v-icon>
        <span class="text-body-2 font-weight-medium">{{ siteStore.sites.filter(s => s.enable_auth).length }} with auth</span>
      </div>
    </div>

    <v-row>
      <v-col cols="12" md="6" lg="4" v-for="(site, idx) in siteStore.sites" :key="site.id">
        <v-card
          class="site-card fill-height"
          :class="{ 'site-disabled': !site.enabled }"
          elevation="2"
        >
          <div class="card-top-bar" :class="site.enabled ? 'bg-primary' : 'bg-grey'"></div>

          <v-card-item class="pb-0">
            <template #prepend>
              <v-avatar :color="site.enable_auth ? 'warning' : 'success'" size="44" variant="tonal">
                <v-icon :color="site.enable_auth ? 'warning' : 'success'" size="22">
                  {{ site.enable_auth ? 'mdi-shield-lock' : 'mdi-web' }}
                </v-icon>
              </v-avatar>
            </template>
            <v-card-title class="text-h6 d-flex align-center">
              <span class="text-truncate">{{ site.title || site.name }}</span>
            </v-card-title>
            <v-card-subtitle class="text-caption">
              <code class="site-name-code">{{ site.name }}</code>
            </v-card-subtitle>
          </v-card-item>

          <v-card-text class="pt-2">
            <div v-if="site.description" class="site-description text-body-2 mb-3">
              {{ site.description }}
            </div>

            <div class="info-rows">
              <div class="info-row d-flex align-center">
                <v-icon size="15" color="medium-emphasis" class="mr-2">mdi-folder</v-icon>
                <span class="text-caption text-medium-emphasis">Path</span>
                <v-spacer />
                <code class="text-caption info-value">{{ site.path }}</code>
              </div>
              <div v-if="site.entry_point" class="info-row d-flex align-center">
                <v-icon size="15" color="medium-emphasis" class="mr-2">mdi-file-code</v-icon>
                <span class="text-caption text-medium-emphasis">Entry</span>
                <v-spacer />
                <code class="text-caption info-value">{{ site.entry_point }}</code>
              </div>
              <div v-if="site.enable_auth && site.auth_user" class="info-row d-flex align-center">
                <v-icon size="15" color="medium-emphasis" class="mr-2">mdi-account</v-icon>
                <span class="text-caption text-medium-emphasis">Auth</span>
                <v-spacer />
                <span class="text-caption info-value">{{ site.auth_user }}</span>
              </div>
            </div>

            <div class="d-flex align-center gap-2 mt-3">
              <v-chip
                v-if="site.enable_auth"
                color="warning"
                size="x-small"
                variant="tonal"
              >
                <v-icon start size="14">mdi-shield-lock</v-icon>
                Auth Enabled
              </v-chip>
              <v-chip
                v-else
                color="success"
                size="x-small"
                variant="tonal"
              >
                <v-icon start size="14">mdi-earth</v-icon>
                Public
              </v-chip>
              <v-chip
                :color="site.enabled ? 'success' : 'grey'"
                size="x-small"
                variant="tonal"
              >
                <v-icon start size="14">{{ site.enabled ? 'mdi-check-circle' : 'mdi-cancel' }}</v-icon>
                {{ site.enabled ? 'Active' : 'Inactive' }}
              </v-chip>
            </div>
          </v-card-text>

          <v-card-actions class="px-4 pb-3 pt-0">
            <v-switch
              :model-value="site.enabled"
              :label="site.enabled ? 'On' : 'Off'"
              color="success"
              density="compact"
              hide-details
              @update:model-value="(v: boolean | null) => handleToggleEnabled(site, !!v)"
            />
            <v-spacer />
            <v-btn
              variant="tonal"
              size="small"
              prepend-icon="mdi-open-in-new"
              color="info"
              @click="openSiteUrl(site)"
            >
              Browse
            </v-btn>
            <v-btn
              variant="text"
              size="small"
              icon="mdi-pencil"
              color="primary"
              @click="openEditDialog(site)"
            />
            <v-btn
              variant="text"
              size="small"
              icon="mdi-delete"
              color="error"
              @click="confirmRemove(site)"
            />
          </v-card-actions>
        </v-card>
      </v-col>
    </v-row>

    <v-row v-if="siteStore.loading">
      <v-col cols="12" md="6" lg="4" v-for="n in 3" :key="n">
        <v-skeleton-loader type="card" class="skeleton-card" />
      </v-col>
    </v-row>

    <div v-if="!siteStore.loading && siteStore.sites.length === 0" class="empty-state text-center py-16">
      <div class="empty-icon-wrapper mb-4">
        <v-icon size="72" color="medium-emphasis">mdi-web-off</v-icon>
      </div>
      <h3 class="text-h5 font-weight-medium mb-2">No sites configured</h3>
      <p class="text-body-1 text-medium-emphasis mb-6">Deploy your first static website to get started</p>
      <v-btn color="primary" size="large" prepend-icon="mdi-plus" variant="elevated" @click="openAddDialog">
        Create Your First Site
      </v-btn>
    </div>

    <v-dialog v-model="showDialog" max-width="800" transition="dialog-bottom-transition" scrim="rgba(0,0,0,0.5)">
      <v-card class="dialog-card">
        <div class="dialog-top-bar" :class="editingSite ? 'bg-primary' : 'bg-success'"></div>
        <v-card-title class="d-flex align-center pt-4 px-6">
          <v-avatar :color="editingSite ? 'primary' : 'success'" size="36" variant="tonal" class="mr-3">
            <v-icon size="20" color="white">
              {{ editingSite ? 'mdi-pencil' : 'mdi-plus' }}
            </v-icon>
          </v-avatar>
          <div>
            <div class="text-h6 font-weight-bold">{{ editingSite ? 'Edit Site' : 'Create New Site' }}</div>
            <div class="text-caption text-medium-emphasis">
              {{ editingSite ? 'Modify site configuration' : 'Configure and deploy a new static website' }}
            </div>
          </div>
        </v-card-title>
        <v-card-text>
          <div class="form-section mb-4">
            <div class="form-section-header d-flex align-center mb-3">
              <v-icon size="18" color="primary" class="mr-2">mdi-information</v-icon>
              <span class="text-body-2 font-weight-bold">Basic Information</span>
            </div>
            <v-row dense>
              <v-col cols="12" md="6">
                <v-text-field
                  v-model="form.name"
                  label="Name"
                  variant="outlined"
                  placeholder="my-site"
                  hint="Unique identifier for the site"
                  persistent-hint
                  density="comfortable"
                />
              </v-col>
              <v-col cols="12" md="6">
                <v-text-field
                  v-model="form.title"
                  label="Title"
                  variant="outlined"
                  placeholder="My Site"
                  density="comfortable"
                />
              </v-col>
              <v-col cols="12">
                <v-textarea
                  v-model="form.description"
                  label="Description"
                  variant="outlined"
                  rows="2"
                  placeholder="Site description..."
                  density="comfortable"
                />
              </v-col>
            </v-row>
          </div>

          <div class="form-section mb-4">
            <div class="form-section-header d-flex align-center mb-3">
              <v-icon size="18" color="primary" class="mr-2">mdi-folder-cog</v-icon>
              <span class="text-body-2 font-weight-bold">Deployment</span>
            </div>
            <v-row dense>
              <v-col cols="12" md="6">
                <v-menu v-model="pathMenuOpen" :close-on-content-click="false" location="bottom">
                  <template v-slot:activator="{ props }">
                    <v-text-field
                      v-model="form.path"
                      label="Path"
                      variant="outlined"
                      placeholder="Select folder"
                      append-icon="mdi-folder"
                      readonly
                      density="comfortable"
                      v-bind="props"
                    />
                  </template>
                  <v-card max-height="320" class="overflow-y-auto folder-picker" min-width="320">
                    <div class="pa-2 d-flex align-center">
                      <v-icon size="18" color="primary" class="mr-2">mdi-folder-open</v-icon>
                      <span class="text-caption font-weight-bold">Select Folder</span>
                      <v-spacer />
                      <v-btn
                        v-if="form.path"
                        variant="text"
                        size="x-small"
                        color="error"
                        @click="form.path = ''"
                      >
                        Clear
                      </v-btn>
                    </div>
                    <v-divider />
                    <v-progress-linear v-if="loadingFolders" indeterminate color="primary" />
                    <v-list v-else-if="folderTree.length === 0" density="compact">
                      <v-list-item title="No folders available" density="compact" />
                    </v-list>
                    <v-list v-else density="compact">
                      <FolderTreeItem
                        :nodes="folderTree"
                        :current-path="form.path"
                        :expanded-paths="expandedPaths"
                        @select="(p: string) => { selectPath(p); pathMenuOpen = false }"
                        @toggle="toggleExpand"
                      />
                    </v-list>
                  </v-card>
                </v-menu>
              </v-col>
              <v-col cols="12" md="6">
                <v-text-field
                  v-model="form.entry_point"
                  label="Entry Point"
                  variant="outlined"
                  placeholder="index.html"
                  density="comfortable"
                />
              </v-col>
              <v-col cols="12" md="6">
                <v-select
                  v-model="form.component"
                  label="Component"
                  variant="outlined"
                  :items="componentOptions"
                  density="comfortable"
                  hint="WASM component for serving static files"
                  persistent-hint
                />
              </v-col>
            </v-row>
          </div>

          <div class="form-section">
            <div class="form-section-header d-flex align-center mb-3">
              <v-icon size="18" color="primary" class="mr-2">mdi-tune</v-icon>
              <span class="text-body-2 font-weight-bold">Configuration</span>
            </div>
            <v-row dense>
              <v-col cols="12" md="6">
                <v-switch
                  v-model="form.enable_auth"
                  label="Enable Authentication"
                  color="warning"
                  hide-details
                  density="compact"
                />
              </v-col>
              <v-col cols="12" md="6">
                <v-switch
                  v-model="form.enabled"
                  label="Enabled"
                  color="success"
                  hide-details
                  density="compact"
                />
              </v-col>
              <v-col cols="12" md="6" v-if="form.enable_auth">
                <v-text-field
                  v-model="form.auth_user"
                  label="Auth User"
                  variant="outlined"
                  placeholder="admin"
                  density="comfortable"
                />
              </v-col>
              <v-col cols="12" md="6" v-if="form.enable_auth">
                <v-text-field
                  v-model="form.auth_pass"
                  label="Auth Password"
                  variant="outlined"
                  type="password"
                  placeholder="Enter password"
                  density="comfortable"
                />
              </v-col>
            </v-row>
          </div>
        </v-card-text>
        <v-card-actions class="pa-4 pt-0">
          <v-spacer />
          <v-btn variant="outlined" @click="showDialog = false">Cancel</v-btn>
          <v-btn
            :color="editingSite ? 'primary' : 'success'"
            variant="elevated"
            @click="handleSave"
            :loading="saving"
            :disabled="!form.name.trim()"
            class="ml-2"
          >
            <v-icon start size="18">{{ editingSite ? 'mdi-content-save' : 'mdi-plus-circle' }}</v-icon>
            {{ editingSite ? 'Save Changes' : 'Create Site' }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-dialog v-model="deleteDialog" max-width="420" transition="dialog-bottom-transition" scrim="rgba(0,0,0,0.5)">
      <v-card class="dialog-card">
        <div class="dialog-top-bar bg-error"></div>
        <v-card-title class="d-flex align-center pt-4">
          <v-icon color="error" size="24" class="mr-2">mdi-delete-alert</v-icon>
          Delete Site
        </v-card-title>
        <v-card-text>
          <div class="delete-confirm pa-4 rounded-lg mb-4">
            <p class="text-body-1 mb-1">
              Are you sure you want to delete
              <strong class="text-error">{{ deleteTarget?.title || deleteTarget?.name }}</strong>?
            </p>
            <p class="text-caption text-medium-emphasis mt-2">
              <v-icon size="14" color="error" class="mr-1">mdi-alert-circle</v-icon>
              This action cannot be undone. All site configuration will be permanently removed.
            </p>
          </div>
        </v-card-text>
        <v-card-actions class="pa-4 pt-0">
          <v-spacer />
          <v-btn variant="outlined" @click="deleteDialog = false">Cancel</v-btn>
          <v-btn color="error" variant="elevated" @click="handleDelete" :loading="deleting" class="ml-2">
            <v-icon start size="18">mdi-delete</v-icon>
            Delete Site
          </v-btn>
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
import FolderTreeItem from '@/components/FolderTreeItem.vue'

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
const expandedPaths = ref(new Set<string>())

const componentOptions = [
  'boxed_static_server_component.wasm',
]

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

interface TreeNode {
  name: string
  path: string
  children: TreeNode[]
}

function buildTree(folderItems: FolderItem[]): TreeNode[] {
  const root: TreeNode[] = []
  const pathMap = new Map<string, TreeNode>()

  folderItems.forEach(item => {
    const node: TreeNode = { name: item.name, path: item.path, children: [] }
    pathMap.set(item.path, node)
  })

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
  resetExpandedPaths()
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
  resetExpandedPaths()
  fetchFolders()
}

function selectPath(path: string) {
  form.value.path = path
}

function toggleExpand(path: string) {
  const next = new Set(expandedPaths.value)
  if (next.has(path)) {
    next.delete(path)
  } else {
    next.add(path)
  }
  expandedPaths.value = next
}

function buildSiteUrl(site: Site): string {
  const entry = site.entry_point || 'index.html'
  return `${window.location.origin}/sites/${site.name}/${entry}`
}

function openSiteUrl(site: Site) {
  window.open(buildSiteUrl(site), '_blank')
}

function resetExpandedPaths() {
  expandedPaths.value = new Set<string>()
}

async function handleSave() {
  saving.value = true
  try {
    if (editingSite.value) {
      await siteStore.updateSite(editingSite.value.id, { ...form.value })
      appStore.showMessage('Site updated successfully', 'success')
    } else {
      await siteStore.addSite({ ...form.value })
      appStore.showMessage('Site created successfully', 'success')
    }
    showDialog.value = false
    form.value = defaultForm()
    editingSite.value = null
  } catch {
    appStore.showMessage('Failed to save site', 'error')
    siteStore.fetchSites()
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
    appStore.showMessage('Site deleted successfully', 'success')
  } catch {
    appStore.showMessage('Failed to delete site', 'error')
    siteStore.fetchSites()
  } finally {
    deleting.value = false
    deleteDialog.value = false
    deleteTarget.value = null
  }
}

async function handleToggleEnabled(site: Site, enabled: boolean) {
  site.enabled = enabled
  try {
    await siteStore.updateSite(site.id, { enabled })
    appStore.showMessage(enabled ? 'Site enabled' : 'Site disabled', 'success')
  } catch {
    site.enabled = !enabled
    appStore.showMessage('Failed to update site', 'error')
    siteStore.fetchSites()
  }
}

onMounted(() => {
  siteStore.fetchSites()
})
</script>

<style scoped>
.sites-page {
  max-width: 1400px;
}

.page-header {
  flex-wrap: wrap;
  gap: 12px;
}

.header-icon-wrapper {
  width: 52px;
  height: 52px;
  border-radius: 14px;
  background: linear-gradient(135deg, var(--v-theme-primary), #7c4dff);
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 4px 16px rgba(var(--v-theme-primary), 0.3);
}

.add-btn {
  border-radius: 10px;
  font-weight: 600;
}

.stats-bar {
  background: rgba(var(--v-theme-surface-variant), 0.4);
  border: 1px solid rgba(var(--v-theme-on-surface), 0.06);
  backdrop-filter: blur(8px);
}

.stat-item {
  min-width: 120px;
}

.site-card {
  border-radius: 14px;
  transition: transform 0.2s ease, box-shadow 0.2s ease;
  overflow: hidden;
}

.site-card:hover {
  transform: translateY(-3px);
  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.12) !important;
}

.site-card.site-disabled {
  opacity: 0.7;
}

.card-top-bar {
  height: 4px;
  width: 100%;
  transition: background-color 0.3s ease;
}

.bg-primary .card-top-bar,
.dialog-top-bar.bg-primary {
  background: linear-gradient(135deg, var(--v-theme-primary), #7c4dff);
}

.bg-success .card-top-bar,
.dialog-top-bar.bg-success {
  background: linear-gradient(135deg, var(--v-theme-success), #4caf50);
}

.bg-error .card-top-bar,
.dialog-top-bar.bg-error {
  background: linear-gradient(135deg, var(--v-theme-error), #ff5252);
}

.bg-grey .card-top-bar,
.dialog-top-bar.bg-grey {
  background: linear-gradient(135deg, #9e9e9e, #bdbdbd);
}

.site-name-code {
  font-size: 0.75rem;
  padding: 2px 6px;
  border-radius: 4px;
  background: rgba(var(--v-theme-on-surface), 0.04);
}

.site-description {
  color: rgba(var(--v-theme-on-surface), 0.7);
  line-height: 1.5;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.info-rows {
  background: rgba(var(--v-theme-on-surface), 0.03);
  border-radius: 8px;
  padding: 4px 0;
}

.info-row {
  padding: 6px 12px;
}

.info-value {
  max-width: 160px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.empty-state {
  border-radius: 16px;
  background: rgba(var(--v-theme-surface-variant), 0.3);
  border: 2px dashed rgba(var(--v-theme-on-surface), 0.08);
}

.empty-icon-wrapper {
  opacity: 0.4;
}

.dialog-top-bar {
  height: 4px;
  width: 100%;
}

.dialog-card {
  border-radius: 14px;
  overflow: hidden;
}

.form-section {
  background: rgba(var(--v-theme-on-surface), 0.02);
  border: 1px solid rgba(var(--v-theme-on-surface), 0.05);
  border-radius: 12px;
  padding: 16px;
}

.form-section-header {
  padding-bottom: 8px;
  border-bottom: 1px solid rgba(var(--v-theme-on-surface), 0.06);
}

.delete-confirm {
  background: rgba(var(--v-theme-error), 0.04);
  border: 1px solid rgba(var(--v-theme-error), 0.12);
}

.folder-picker {
  border-radius: 10px;
}

.skeleton-card {
  border-radius: 14px;
  overflow: hidden;
}

@media (max-width: 600px) {
  .stats-bar {
    flex-wrap: wrap;
    gap: 8px;
  }

  .stats-bar :deep(.v-divider) {
    display: none;
  }

  .stat-item {
    min-width: auto;
  }
}
</style>
