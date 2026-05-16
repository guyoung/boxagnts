<template>
  <div>
    <div class="d-flex align-center justify-space-between mb-6">
      <div class="d-flex align-center">
        <v-icon size="32" color="primary" class="mr-3">mdi-clock-outline</v-icon>
        <h1 class="text-h4 font-weight-bold">Cron Jobs</h1>
      </div>
      <v-btn color="primary" prepend-icon="mdi-plus" @click="openAddDialog">
        Add Cron
      </v-btn>
    </div>

    <v-row>
      <v-col cols="12" v-for="cron in cronStore.crons" :key="cron.id">
        <v-card>
          <v-card-item>
            <template #prepend>
              <v-icon :color="cron.enabled ? 'success' : 'medium-emphasis'" size="28">
                mdi-clock-outline
              </v-icon>
            </template>
            <v-card-title>{{ cron.name }}</v-card-title>
            <v-card-subtitle v-if="cron.description">{{ cron.description }}</v-card-subtitle>
          </v-card-item>

          <v-card-text>
            <v-row dense>
              <v-col cols="12" md="6">
                <div class="text-caption text-medium-emphasis mb-1">Cron Expression</div>
                <code class="text-body-2 cron-expr">{{ cron.cron }}</code>
              </v-col>
              <v-col cols="6" md="3">
                <div class="text-caption text-medium-emphasis mb-1">Timeout</div>
                <div class="text-body-2">{{ cron.timeout ? `${cron.timeout}s` : '-' }}</div>
              </v-col>
              <v-col cols="6" md="3">
                <div class="text-caption text-medium-emphasis mb-1">Last Run</div>
                <div class="text-body-2">{{ cron.last_run_at ? formatDate(cron.last_run_at) : 'Never' }}</div>
              </v-col>
            </v-row>
            <v-row dense class="mt-2">
              <v-col cols="12">
                <div class="text-caption text-medium-emphasis mb-1">Prompt</div>
                <div class="text-body-2 text-truncate">{{ cron.prompt || '-' }}</div>
              </v-col>
            </v-row>
          </v-card-text>

          <v-card-actions>
            <v-switch
              :model-value="cron.enabled"
              :label="cron.enabled ? 'Enabled' : 'Disabled'"
              color="success"
              density="compact"
              hide-details
              @update:model-value="(v: boolean | null) => handleToggleEnabled(cron, !!v)"
            />
            <v-spacer />
            <v-btn variant="tonal" size="small" prepend-icon="mdi-history" @click.stop="openHistory(cron)">
              History
            </v-btn>
            <v-btn variant="tonal" size="small" prepend-icon="mdi-pencil" @click="openEditDialog(cron)">
              Edit
            </v-btn>
            <v-btn
              icon="mdi-delete"
              variant="text"
              size="small"
              color="error"
              @click="confirmRemove(cron)"
            />
          </v-card-actions>
        </v-card>
      </v-col>
    </v-row>

    <v-skeleton-loader v-if="cronStore.loading" type="card@3" />

    <div v-if="!cronStore.loading && cronStore.crons.length === 0" class="text-center py-12">
      <v-icon size="64" color="medium-emphasis">mdi-clock-off</v-icon>
      <p class="text-medium-emphasis mt-4">No cron jobs configured</p>
      <v-btn color="primary" class="mt-4" @click="openAddDialog">Add Cron</v-btn>
    </div>

    <!-- Add / Edit Dialog -->
    <v-dialog v-model="showDialog" max-width="600">
      <v-card>
        <v-card-title>{{ editingCron ? 'Edit Cron' : 'Add Cron' }}</v-card-title>
        <v-card-text>
          <v-row dense>
            <v-col cols="12" md="6">
              <v-text-field
                v-model="form.name"
                label="Name"
                variant="outlined"
                placeholder="daily-backup"
                hint="Unique name for the cron job"
                persistent-hint
              />
            </v-col>
            <v-col cols="12">
              <v-textarea
                v-model="form.description"
                label="Description"
                variant="outlined"
                rows="2"
                placeholder="Cron job description..."
              />
            </v-col>
            <v-col cols="12">
              <v-text-field
                v-model="form.cron"
                label="Cron Expression"
                variant="outlined"
                placeholder="0 0 0 * * *"
                hint="sec min hour day month day_of_week (UTC timezone)"
                persistent-hint
              />
            </v-col>
            <v-col cols="12" md="6">
              <v-text-field
                v-model.number="form.timeout"
                label="Timeout"
                type="number"
                variant="outlined"
                placeholder="30"
                hint="Timeout in seconds"
                persistent-hint
              />
            </v-col>
            <v-col cols="12" md="6">
              <v-switch
                v-model="form.enabled"
                label="Enabled"
                color="success"
                hide-details
              />
            </v-col>
            <v-col cols="12">
              <v-textarea
                v-model="form.prompt"
                label="Prompt"
                variant="outlined"
                rows="4"
                placeholder="Enter your prompt here..."
              />
            </v-col>
          </v-row>

          <v-divider class="my-4" />

          <div class="text-caption text-medium-emphasis mb-2">
            Cron Expression: <code class="ml-2">sec min hour day month day_of_week</code> (UTC timezone)
          </div>
          <v-alert type="info" variant="tonal" density="compact" class="mb-3">
            <ul class="mb-0" style="list-style-type: disc; padding-left: 1.5rem; font-size: 0.75rem;">
              <li>Comma-separated: <code>0 2,14,26 * * * *</code></li>
              <li>Ranges: <code>0 0 * 5-10 * *</code></li>
              <li>Day of week: <code>0 0 6 * * Sun,Sat</code></li>
            </ul>
          </v-alert>
          <div class="text-caption text-medium-emphasis mb-2">Quick Presets</div>
          <v-row dense>
            <v-col cols="4" v-for="preset in cronPresets" :key="preset.label">
              <v-chip size="x-small" variant="tonal" class="mr-1 mb-1" @click="form.cron = preset.expr">
                {{ preset.label }}
              </v-chip>
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
            :disabled="!form.name.trim() || !form.cron.trim()"
          >
            {{ editingCron ? 'Update' : 'Create' }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Delete Confirmation Dialog -->
    <v-dialog v-model="deleteDialog" max-width="400">
      <v-card>
        <v-card-title>Delete Cron</v-card-title>
        <v-card-text>
          <p>
            Are you sure you want to delete
            <strong>{{ deleteTarget?.name }}</strong>?
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

    <!-- History Dialog -->
    <v-dialog v-model="historyDialog" max-width="700">
      <v-card>
        <v-card-title class="d-flex align-center">
          <v-icon start>mdi-history</v-icon>
          Execution History — {{ historyTarget?.name }}
        </v-card-title>
        <v-card-text>
          <div class="d-flex align-center mb-3">
            <v-btn variant="tonal" size="small" prepend-icon="mdi-refresh" @click="refreshHistory" :loading="historyLoading">
              Refresh
            </v-btn>
          </div>

          <div v-if="historyLoading" class="text-center pa-4">
            <v-progress-circular indeterminate size="24" width="2" color="primary" />
          </div>

          <v-list v-else-if="historyLogs.length > 0" lines="two" density="compact">
            <v-list-item v-for="log in historyLogs" :key="log.id" rounded="lg" class="mb-1">
              <template #prepend>
                <v-icon :color="log.success ? 'success' : 'error'" size="20">
                  {{ log.success ? 'mdi-check-circle' : 'mdi-alert-circle' }}
                </v-icon>
              </template>
              <v-list-item-title class="text-body-2">
                {{ formatDate(log.executed_at) }}
                <v-chip :color="log.success ? 'success' : 'error'" size="x-small" variant="tonal" class="ml-2">
                  {{ log.success ? 'Success' : 'Failed' }}
                </v-chip>
              </v-list-item-title>
              <v-list-item-subtitle v-if="log.message" class="mt-1">
                <pre class="log-output" :class="!log.success ? 'text-error' : ''">{{ log.message }}</pre>
              </v-list-item-subtitle>
            </v-list-item>
          </v-list>

          <div v-else class="text-center py-6">
            <v-icon size="40" color="medium-emphasis">mdi-text-box-outline</v-icon>
            <p class="text-medium-emphasis mt-2 text-body-2">No execution history yet</p>
          </div>
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="historyDialog = false">Close</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useCronStore } from '@/stores/crons'
import { useAppStore } from '@/stores/app'
import type { CronJob, CronLog } from '@/api'

const cronStore = useCronStore()
const appStore = useAppStore()

const showDialog = ref(false)
const editingCron = ref<CronJob | null>(null)
const saving = ref(false)
const deleteDialog = ref(false)
const deleteTarget = ref<CronJob | null>(null)
const deleting = ref(false)

const historyDialog = ref(false)
const historyTarget = ref<CronJob | null>(null)
const historyLogs = ref<CronLog[]>([])
const historyLoading = ref(false)

const cronPresets = [
  { label: 'Every minute', expr: '0 * * * * *' },
  { label: 'Every 5 min', expr: '0 */5 * * * *' },
  { label: 'Every 15 min', expr: '0 */15 * * * *' },
  { label: 'Every hour', expr: '0 0 * * * *' },
  { label: 'Daily midnight (UTC)', expr: '0 0 0 * * *' },
  { label: 'Daily 6 AM (UTC)', expr: '0 0 6 * * *' },
  { label: 'Weekends', expr: '0 0 6 * * Sun,Sat' },
  { label: 'Monthly 1st', expr: '0 0 0 1 * *' },
  { label: 'Weekdays 9AM (UTC)', expr: '0 0 9 * * Mon-Fri' },
]

type CronForm = Omit<CronJob, 'id' | 'last_run_at' | 'last_run_success'>

const defaultForm = (): CronForm => ({
  name: '',
  description: '',
  cron: '',
  enabled: true,
  timeout: null,
  prompt: null,
})

const form = ref<CronForm>(defaultForm())

function openAddDialog() {
  editingCron.value = null
  form.value = defaultForm()
  showDialog.value = true
}

function openEditDialog(cron: CronJob) {
  editingCron.value = cron
  form.value = {
    name: cron.name,
    description: cron.description,
    cron: cron.cron,
    enabled: cron.enabled,
    timeout: cron.timeout,
    prompt: cron.prompt,
  }
  showDialog.value = true
}

async function handleSave() {
  saving.value = true
  try {
    if (editingCron.value) {
      await cronStore.updateCron(editingCron.value.id, { ...form.value })
      appStore.showMessage('Cron updated', 'success')
    } else {
      await cronStore.addCron({ ...form.value })
      appStore.showMessage('Cron created', 'success')
    }
    showDialog.value = false
    form.value = defaultForm()
    editingCron.value = null
    await cronStore.fetchCrons()
  } catch {
    appStore.showMessage('Failed to save cron', 'error')
  } finally {
    saving.value = false
  }
}

function confirmRemove(cron: CronJob) {
  deleteTarget.value = cron
  deleteDialog.value = true
}

async function handleDelete() {
  if (!deleteTarget.value) return
  deleting.value = true
  try {
    await cronStore.removeCron(deleteTarget.value.id)
    appStore.showMessage('Cron deleted', 'success')
    await cronStore.fetchCrons()
  } catch {
    appStore.showMessage('Failed to delete cron', 'error')
  } finally {
    deleting.value = false
    deleteDialog.value = false
    deleteTarget.value = null
  }
}

async function handleToggleEnabled(cron: CronJob, enabled: boolean) {
  try {
    await cronStore.updateCron(cron.id, { enabled })
    appStore.showMessage(enabled ? 'Cron enabled' : 'Cron disabled', 'success')
    await cronStore.fetchCrons()
  } catch {
    appStore.showMessage('Failed to update cron', 'error')
  }
}

async function openHistory(cron: CronJob) {
  historyTarget.value = cron
  historyDialog.value = true
  historyLoading.value = true
  try {
    historyLogs.value = await cronStore.fetchCronLogs(cron.id)
  } catch {
    historyLogs.value = []
  } finally {
    historyLoading.value = false
  }
}

async function refreshHistory() {
  if (!historyTarget.value) return
  historyLoading.value = true
  try {
    historyLogs.value = await cronStore.fetchCronLogs(historyTarget.value.id)
  } catch {
    historyLogs.value = []
  } finally {
    historyLoading.value = false
  }
}

function formatDate(dateStr: string): string {
  return new Date(dateStr).toLocaleDateString('zh-CN', { month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' })
}

onMounted(() => {
  cronStore.fetchCrons()
})
</script>

<style scoped>
.cron-expr {
  background: rgba(var(--v-theme-on-surface), 0.06);
  padding: 2px 8px;
  border-radius: 4px;
  font-family: 'Consolas', 'Courier New', monospace;
  font-size: 14px;
}

.log-output {
  white-space: pre-wrap;
  word-break: break-all;
  font-family: 'Consolas', 'Courier New', monospace;
  font-size: 11px;
  max-height: 120px;
  overflow-y: auto;
  background: rgba(var(--v-theme-on-surface), 0.04);
  padding: 6px 8px;
  border-radius: 4px;
  margin: 0;
}
</style>
