<template>
  <div class="crons-page">
    <div class="page-header d-flex align-center justify-space-between mb-6">
      <div class="d-flex align-center">
        <div class="header-icon-wrapper mr-3">
          <v-icon size="28" color="white">mdi-clock-outline</v-icon>
        </div>
        <div>
          <h1 class="text-h4 font-weight-bold mb-0">Cron Jobs</h1>
          <p class="text-body-2 text-medium-emphasis mt-1">Manage scheduled AI agent tasks</p>
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
        Add Cron
      </v-btn>
    </div>

    <div class="stats-bar d-flex align-center pa-4 mb-6 rounded-lg" v-if="!cronStore.loading && cronStore.crons.length > 0">
      <div class="stat-item d-flex align-center">
        <v-icon size="20" color="primary" class="mr-2">mdi-clock-outline</v-icon>
        <span class="text-body-2 font-weight-medium">{{ cronStore.crons.length }} job{{ cronStore.crons.length > 1 ? 's' : '' }}</span>
      </div>
      <v-divider vertical class="mx-4" />
      <div class="stat-item d-flex align-center">
        <v-icon size="20" color="success" class="mr-2">mdi-play-circle</v-icon>
        <span class="text-body-2 font-weight-medium">{{ cronStore.crons.filter(c => c.enabled).length }} running</span>
      </div>
      <v-divider vertical class="mx-4" />
      <div class="stat-item d-flex align-center">
        <v-icon size="20" color="medium-emphasis" class="mr-2">mdi-pause-circle</v-icon>
        <span class="text-body-2 font-weight-medium">{{ cronStore.crons.filter(c => !c.enabled).length }} paused</span>
      </div>
    </div>

    <v-row>
      <v-col cols="12" md="6" lg="4" v-for="cron in cronStore.crons" :key="cron.id">
        <v-card
          class="cron-card fill-height"
          :class="{ 'cron-disabled': !cron.enabled }"
          elevation="2"
        >
          <div class="card-top-bar" :class="cron.enabled ? 'bg-primary' : 'bg-grey'"></div>

          <v-card-item class="pb-1">
            <template #prepend>
              <v-avatar :color="cron.enabled ? 'primary' : 'grey'" size="40" variant="tonal">
                <v-icon :color="cron.enabled ? 'primary' : 'grey-darken-1'" size="22">
                  mdi-clock-outline
                </v-icon>
              </v-avatar>
            </template>
            <v-card-title class="text-body-1 font-weight-bold pr-2">
              {{ cron.name }}
            </v-card-title>
            <template #append>
              <v-chip
                :color="cron.enabled ? 'success' : 'medium-emphasis'"
                size="x-small"
                variant="tonal"
                label
              >
                {{ cron.enabled ? 'Running' : 'Paused' }}
              </v-chip>
            </template>
            <v-card-subtitle v-if="cron.description" class="mt-1">
              {{ cron.description }}
            </v-card-subtitle>
          </v-card-item>

          <v-card-text class="pt-2">
            <div class="info-grid">
              <div class="info-item info-item-wide">
                <span class="info-label">Cron</span>
                <code class="cron-expr-badge">{{ cron.cron }}</code>
              </div>
              <div class="info-item">
                <span class="info-label">Timeout</span>
                <span class="info-value">{{ cron.timeout ? `${cron.timeout}s` : '-' }}</span>
              </div>
              <div class="info-item">
                <span class="info-label">Model</span>
                <span class="info-value">{{ cron.model || 'Default' }}</span>
              </div>
              <div class="info-item">
                <span class="info-label">Last Run</span>
                <span class="info-value d-flex align-center">
                  <template v-if="cron.last_run_at">
                    <v-icon
                      :color="cron.last_run_success ? 'success' : 'error'"
                      size="14"
                      class="mr-1"
                    >
                      {{ cron.last_run_success ? 'mdi-check-circle' : 'mdi-alert-circle' }}
                    </v-icon>
                    {{ formatDate(cron.last_run_at) }}
                  </template>
                  <template v-else>
                    <v-icon size="14" color="medium-emphasis" class="mr-1">mdi-minus-circle</v-icon>
                    Never
                  </template>
                </span>
              </div>
            </div>
            <div class="info-item info-item-wide mt-2" v-if="cron.prompt">
              <span class="info-label">Prompt</span>
              <span class="info-value text-truncate">{{ cron.prompt }}</span>
            </div>
          </v-card-text>

          <v-card-actions class="px-4 pb-3">
            <v-btn
              variant="tonal"
              size="small"
              prepend-icon="mdi-history"
              @click.stop="openHistory(cron)"
            >
              History
            </v-btn>
            <v-btn
              variant="tonal"
              size="small"
              prepend-icon="mdi-pencil"
              @click="openEditDialog(cron)"
            >
              Edit
            </v-btn>
            <v-spacer />
            <v-switch
              :model-value="cron.enabled"
              color="success"
              density="compact"
              hide-details
              class="mr-1"
              @update:model-value="(v: boolean | null) => handleToggleEnabled(cron, !!v)"
            />
          </v-card-actions>
        </v-card>
      </v-col>
    </v-row>

    <v-skeleton-loader v-if="cronStore.loading" type="card@3" />

    <div v-if="!cronStore.loading && cronStore.crons.length === 0" class="empty-state text-center py-12">
      <div class="empty-icon-wrapper mb-6">
        <v-icon size="72" color="primary">mdi-clock-off</v-icon>
      </div>
      <h3 class="text-h5 font-weight-medium mb-2">No cron jobs configured</h3>
      <p class="text-body-1 text-medium-emphasis mb-6">Schedule recurring AI agent tasks with cron expressions</p>
      <v-btn color="primary" size="large" variant="elevated" prepend-icon="mdi-plus" @click="openAddDialog">
        Create Your First Cron
      </v-btn>
    </div>

    <v-dialog v-model="showDialog" max-width="820" transition="dialog-bottom-transition">
      <v-card class="dialog-card">
        <div class="dialog-top-bar dialog-top-bar-thick" :class="editingCron ? 'bg-warning' : 'bg-primary'"></div>

        <v-card-title class="d-flex align-center pt-5 px-6">
          <div class="dialog-title-icon mr-3" :class="editingCron ? 'bg-warning' : 'bg-primary'">
            <v-icon color="white" size="22">
              {{ editingCron ? 'mdi-pencil' : 'mdi-plus' }}
            </v-icon>
          </div>
          <div>
            <div class="text-h6 font-weight-bold">{{ editingCron ? 'Edit Cron Job' : 'New Cron Job' }}</div>
            <div class="text-caption text-medium-emphasis mt-1">
              {{ editingCron ? 'Modify schedule, prompt, and model settings' : 'Configure a new recurring AI agent task' }}
            </div>
          </div>
        </v-card-title>

        <v-card-text class="px-6">
          <div class="dialog-section">
            <div class="section-header d-flex align-center mb-3">
              <v-icon size="18" color="primary" class="mr-2">mdi-information-outline</v-icon>
              <span class="text-body-2 font-weight-bold">Basic Information</span>
            </div>
            <v-row dense>
              <v-col cols="12" md="7">
                <v-text-field
                  v-model="form.name"
                  label="Name *"
                  variant="outlined"
                  placeholder="daily-report"
                  :error-messages="nameError"
                  density="comfortable"
                  @input="nameError = ''"
                />
              </v-col>
              <v-col cols="12" md="5">
                <v-select
                  v-model="form.model"
                  label="Model"
                  :items="availableModels"
                  variant="outlined"
                  clearable
                  placeholder="System default"
                  density="comfortable"
                />
              </v-col>
              <v-col cols="12">
                <v-textarea
                  v-model="form.description"
                  label="Description"
                  variant="outlined"
                  rows="2"
                  placeholder="Describe what this cron job does..."
                  density="comfortable"
                />
              </v-col>
            </v-row>
          </div>

          <div class="dialog-section schedule-section">
            <div class="section-header d-flex align-center mb-3">
              <v-icon size="18" color="secondary" class="mr-2">mdi-calendar-sync</v-icon>
              <span class="text-body-2 font-weight-bold">Schedule</span>
              <v-spacer />
              <v-btn
                variant="text"
                size="x-small"
                :color="cronBuildMode ? 'secondary' : 'medium-emphasis'"
                :prepend-icon="cronBuildMode ? 'mdi-code-tags' : 'mdi-tune-variant'"
                @click="toggleBuildMode"
              >
                {{ cronBuildMode ? 'Expression' : 'Builder' }}
              </v-btn>
            </div>

            <v-expand-transition>
              <div v-if="cronBuildMode" class="cron-builder-panel pa-3 rounded-lg mb-3">
                <div class="builder-row">
                  <div class="builder-field">
                    <label class="builder-label">Second</label>
                    <v-select
                      :model-value="cronBuilder.second"
                      :items="CRON_FIELD_DEFS.second"
                      item-title="title"
                      item-value="value"
                      variant="outlined"
                      density="compact"
                      hide-details
                      @update:model-value="cronBuilder.second = $event; onBuilderFieldChange()"
                    />
                  </div>
                  <div class="builder-field">
                    <label class="builder-label">Minute</label>
                    <v-select
                      :model-value="cronBuilder.minute"
                      :items="CRON_FIELD_DEFS.minute"
                      item-title="title"
                      item-value="value"
                      variant="outlined"
                      density="compact"
                      hide-details
                      @update:model-value="cronBuilder.minute = $event; onBuilderFieldChange()"
                    />
                  </div>
                  <div class="builder-field">
                    <label class="builder-label">Hour</label>
                    <v-select
                      :model-value="cronBuilder.hour"
                      :items="CRON_FIELD_DEFS.hour"
                      item-title="title"
                      item-value="value"
                      variant="outlined"
                      density="compact"
                      hide-details
                      @update:model-value="cronBuilder.hour = $event; onBuilderFieldChange()"
                    />
                  </div>
                </div>
                <div class="builder-row">
                  <div class="builder-field">
                    <label class="builder-label">Day of Month</label>
                    <v-select
                      :model-value="cronBuilder.day"
                      :items="CRON_FIELD_DEFS.day"
                      item-title="title"
                      item-value="value"
                      variant="outlined"
                      density="compact"
                      hide-details
                      @update:model-value="cronBuilder.day = $event; onBuilderFieldChange()"
                    />
                  </div>
                  <div class="builder-field">
                    <label class="builder-label">Month</label>
                    <v-select
                      :model-value="cronBuilder.month"
                      :items="CRON_FIELD_DEFS.month"
                      item-title="title"
                      item-value="value"
                      variant="outlined"
                      density="compact"
                      hide-details
                      @update:model-value="cronBuilder.month = $event; onBuilderFieldChange()"
                    />
                  </div>
                  <div class="builder-field">
                    <label class="builder-label">Day of Week</label>
                    <v-select
                      :model-value="cronBuilder.dow"
                      :items="CRON_FIELD_DEFS.dow"
                      item-title="title"
                      item-value="value"
                      variant="outlined"
                      density="compact"
                      hide-details
                      @update:model-value="cronBuilder.dow = $event; onBuilderFieldChange()"
                    />
                  </div>
                </div>
              </div>
            </v-expand-transition>

            <v-row dense>
              <v-col cols="12">
                <v-text-field
                  v-model="form.cron"
                  label="Cron Expression *"
                  variant="outlined"
                  :placeholder="cronBuildMode ? 'auto-generated from builder' : '0 0 9 * * Mon-Fri'"
                  hint="Click to edit — presets and next-run preview will appear"
                  persistent-hint
                  density="comfortable"
                  :error-messages="cronError"
                  :readonly="cronBuildMode"
                  @input="onCronInput()"
                  @focus="cronFocused = true"
                  @blur="onCronBlur"
                >
                  <template #prepend-inner>
                    <v-icon size="18" color="medium-emphasis">mdi-code-tags</v-icon>
                  </template>
                </v-text-field>
              </v-col>
            </v-row>

            <div v-if="cronDescription" class="cron-description pa-2 rounded-lg mt-2 mb-2">
              <v-icon size="14" color="medium-emphasis" class="mr-1">mdi-text-short</v-icon>
              <span class="text-caption text-medium-emphasis">{{ cronDescription }}</span>
            </div>

            <v-expand-transition>
              <div v-if="cronFocused && nextRuns.length > 0" class="next-runs-panel pa-3 rounded-lg mt-3">
                <div class="d-flex align-center">
                  <div class="next-runs-icon-wrapper mr-2">
                    <v-icon size="16" color="primary">mdi-calendar-clock</v-icon>
                  </div>
                  <span class="text-caption font-weight-bold text-primary">Next Execution</span>
                  <v-spacer />
                  <v-chip size="x-small" color="primary" variant="flat" label>
                    {{ nextRunRelative(nextRuns[0]) }}
                  </v-chip>
                </div>
                <div class="d-flex align-center mt-2">
                  <v-icon size="14" color="medium-emphasis" class="mr-1">mdi-clock-outline</v-icon>
                  <span class="text-body-2 text-medium-emphasis">{{ formatNextRunTime(nextRuns[0]) }} UTC</span>
                </div>
              </div>
            </v-expand-transition>

            <v-divider class="my-3" />

            <v-expand-transition>
              <div v-if="cronFocused" class="schedule-helper-section mb-2">
                <div class="preset-grid">
                  <div
                    v-for="preset in cronPresets"
                    :key="preset.label"
                    class="preset-chip"
                    :class="{ 'preset-active': form.cron === preset.expr }"
                    @mousedown.prevent
                    @click="form.cron = preset.expr; cronError = ''; syncBuilderFromExpr(preset.expr)"
                  >
                    <span class="preset-label text-caption">{{ preset.label }}</span>
                    <code class="preset-expr text-caption">{{ preset.expr }}</code>
                  </div>
                </div>
              </div>
            </v-expand-transition>

            <v-expand-transition>
              <v-alert v-if="cronFocused" type="info" variant="tonal" density="compact" class="syntax-alert mt-0">
                <template #text>
                  <div class="syntax-grid">
                    <div class="syntax-item">
                      <code class="syntax-expr">0 2,14,26 * * * *</code>
                      <span class="text-caption text-medium-emphasis">Comma values</span>
                    </div>
                    <div class="syntax-item">
                      <code class="syntax-expr">0 0 * 5-10 * *</code>
                      <span class="text-caption text-medium-emphasis">Ranges</span>
                    </div>
                    <div class="syntax-item">
                      <code class="syntax-expr">0 */15 * * * *</code>
                      <span class="text-caption text-medium-emphasis">Steps</span>
                    </div>
                    <div class="syntax-item">
                      <code class="syntax-expr">0 0 6 * * Sun,Sat</code>
                      <span class="text-caption text-medium-emphasis">Day names</span>
                    </div>
                  </div>
                </template>
              </v-alert>
            </v-expand-transition>
          </div>

          <div class="dialog-section">
            <div class="section-header d-flex align-center mb-3">
              <v-icon size="18" color="tertiary" class="mr-2">mdi-message-text-outline</v-icon>
              <span class="text-body-2 font-weight-bold">Task</span>
            </div>
            <v-row dense>
              <v-col cols="12">
                <v-textarea
                  v-model="form.prompt"
                  label="Prompt"
                  variant="outlined"
                  rows="4"
                  placeholder="Enter the AI prompt to execute on schedule..."
                  density="comfortable"
                  auto-grow
                />
              </v-col>
              <v-col cols="6" md="4">
                <v-text-field
                  v-model.number="form.timeout"
                  label="Timeout (s)"
                  type="number"
                  variant="outlined"
                  placeholder="30"
                  density="comfortable"
                  min="1"
                >
                  <template #prepend-inner>
                    <v-icon size="16" color="medium-emphasis">mdi-timer-outline</v-icon>
                  </template>
                </v-text-field>
              </v-col>
              <v-col cols="6" md="4">
                <div class="enabled-toggle">
                  <span class="text-caption text-medium-emphasis mr-2">Enabled</span>
                  <v-switch
                    v-model="form.enabled"
                    color="success"
                    hide-details
                    density="compact"
                  />
                </div>
              </v-col>
            </v-row>
          </div>
        </v-card-text>

        <v-card-actions class="dialog-actions px-6 pb-5 pt-2">
          <v-spacer />
          <v-btn variant="outlined" size="large" @click="showDialog = false" class="px-6">Cancel</v-btn>
          <v-btn
            color="primary"
            variant="elevated"
            size="large"
            @click="handleSave"
            :loading="saving"
            :disabled="!form.name.trim() || !form.cron.trim()"
            class="ml-3 px-8"
          >
            <v-icon start size="20">{{ editingCron ? 'mdi-content-save' : 'mdi-plus' }}</v-icon>
            {{ editingCron ? 'Save Changes' : 'Create Cron' }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-dialog v-model="deleteDialog" max-width="440" transition="dialog-bottom-transition">
      <v-card class="dialog-card">
        <div class="dialog-top-bar bg-error"></div>
        <v-card-title class="d-flex align-center pt-5">
          <v-icon color="error" size="24" class="mr-2">mdi-delete-alert</v-icon>
          Delete Cron Job
        </v-card-title>
        <v-card-text>
          <p class="text-body-1">
            Are you sure you want to delete
            <strong class="text-error">{{ deleteTarget?.name }}</strong>?
          </p>
          <v-alert type="warning" variant="tonal" density="compact" class="mt-3">
            This action cannot be undone. All execution history will be permanently removed.
          </v-alert>
        </v-card-text>
        <v-card-actions class="px-6 pb-5">
          <v-spacer />
          <v-btn variant="outlined" @click="deleteDialog = false">Cancel</v-btn>
          <v-btn color="error" variant="elevated" @click="handleDelete" :loading="deleting" class="ml-3">
            Delete
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-dialog v-model="historyDialog" max-width="760" transition="dialog-bottom-transition">
      <v-card class="dialog-card">
        <div class="dialog-top-bar bg-primary"></div>
        <v-card-title class="d-flex align-center pt-5">
          <v-icon color="primary" size="24" class="mr-2">mdi-history</v-icon>
          Execution History — {{ historyTarget?.name }}
        </v-card-title>
        <v-card-text>
          <div class="d-flex align-center mb-4">
            <v-btn
              variant="tonal"
              size="small"
              prepend-icon="mdi-refresh"
              @click="refreshHistory"
              :loading="historyLoading"
            >
              Refresh
            </v-btn>
            <v-spacer />
            <span class="text-caption text-medium-emphasis" v-if="historyLogs.length">
              {{ historyLogs.length }} record{{ historyLogs.length > 1 ? 's' : '' }}
            </span>
          </div>

          <div v-if="historyLoading" class="text-center pa-8">
            <v-progress-circular indeterminate size="32" width="3" color="primary" />
          </div>

          <v-list v-else-if="historyLogs.length > 0" lines="two" density="compact" class="history-list rounded-lg">
            <v-list-item
              v-for="log in historyLogs"
              :key="log.id"
              rounded="lg"
              class="mb-1 history-item"
              :class="log.success ? 'history-success' : 'history-error'"
            >
              <template #prepend>
                <v-avatar :color="log.success ? 'success' : 'error'" size="32" variant="tonal">
                  <v-icon :color="log.success ? 'success' : 'error'" size="18">
                    {{ log.success ? 'mdi-check-circle' : 'mdi-alert-circle' }}
                  </v-icon>
                </v-avatar>
              </template>
              <v-list-item-title class="d-flex align-center">
                <span class="text-body-2 font-weight-medium">{{ formatDate(log.executed_at) }}</span>
                <v-chip
                  :color="log.success ? 'success' : 'error'"
                  size="x-small"
                  variant="tonal"
                  class="ml-2"
                  label
                >
                  {{ log.success ? 'Success' : 'Failed' }}
                </v-chip>
              </v-list-item-title>
              <v-list-item-subtitle v-if="log.message" class="mt-1">
                <pre class="log-output" :class="{ 'text-error': !log.success }">{{ log.message }}</pre>
              </v-list-item-subtitle>
            </v-list-item>
          </v-list>

          <div v-else class="text-center py-8">
            <v-icon size="48" color="medium-emphasis" class="mb-3">mdi-text-box-outline</v-icon>
            <p class="text-body-1 text-medium-emphasis">No execution history yet</p>
            <p class="text-caption text-medium-emphasis mt-1">Records will appear here after the cron job runs</p>
          </div>
        </v-card-text>
        <v-card-actions class="px-6 pb-5">
          <v-spacer />
          <v-btn variant="outlined" @click="historyDialog = false">Close</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, nextTick } from 'vue'
import { useCronStore } from '@/stores/crons'
import { useAppStore } from '@/stores/app'
import { api, type CronJob, type CronLog } from '@/api'
import { getNextRunTimes, isValidCronExpr, CRON_FIELD_DEFS, DEFAULT_BUILDER, builderToExpr, exprToBuilder, describeCron } from '@/utils/cron'

const cronStore = useCronStore()
const appStore = useAppStore()

const availableModels = ref<string[]>([])

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

const nameError = ref('')
const cronError = ref('')
const cronFocused = ref(false)
const cronBuilder = ref({ ...DEFAULT_BUILDER })
const cronBuildMode = ref(false)

const cronPresets = [
  { label: 'Every minute', expr: '0 * * * * *' },
  { label: 'Every 5 min', expr: '0 */5 * * * *' },
  { label: 'Every 15 min', expr: '0 */15 * * * *' },
  { label: 'Every hour', expr: '0 0 * * * *' },
  { label: 'Daily midnight', expr: '0 0 0 * * *' },
  { label: 'Daily 6 AM', expr: '0 0 6 * * *' },
  { label: 'Weekends 6 AM', expr: '0 0 6 * * Sun,Sat' },
  { label: 'Monthly 1st', expr: '0 0 0 1 * *' },
  { label: 'Weekdays 9 AM', expr: '0 0 9 * * Mon-Fri' },
]

type CronForm = Omit<CronJob, 'id' | 'last_run_at' | 'last_run_success'>

const defaultForm = (): CronForm => ({
  name: '',
  description: '',
  cron: '',
  enabled: true,
  timeout: null,
  prompt: null,
  model: null,
})

const form = ref<CronForm>(defaultForm())

const nextRuns = computed(() => {
  const expr = form.value.cron.trim()
  if (!expr || !isValidCronExpr(expr)) {
    return []
  }
  return getNextRunTimes(expr, 1)
})

const cronDescription = computed(() => {
  const expr = form.value.cron.trim()
  if (!expr || !isValidCronExpr(expr)) return ''
  return describeCron(expr)
})

function syncBuilderFromExpr(expr: string) {
  const trimmed = expr.trim()
  if (isValidCronExpr(trimmed)) {
    cronBuilder.value = exprToBuilder(trimmed)
  }
}

function applyBuilderToExpr() {
  form.value.cron = builderToExpr(cronBuilder.value)
  cronError.value = ''
}

function onCronInput() {
  cronError.value = ''
  syncBuilderFromExpr(form.value.cron)
}

function onBuilderFieldChange() {
  applyBuilderToExpr()
}

function toggleBuildMode() {
  cronBuildMode.value = !cronBuildMode.value
  if (cronBuildMode.value) {
    syncBuilderFromExpr(form.value.cron)
  }
}

function validateName(): boolean {
  const trimmed = form.value.name.trim()
  if (!trimmed) {
    nameError.value = 'Name is required'
    return false
  }
  const existing = cronStore.crons.find(
    c => c.name.toLowerCase() === trimmed.toLowerCase() && c.id !== editingCron.value?.id
  )
  if (existing) {
    nameError.value = 'A cron job with this name already exists'
    return false
  }
  return true
}

function validateCron(): boolean {
  if (!form.value.cron.trim()) {
    cronError.value = 'Cron expression is required'
    return false
  }
  if (!isValidCronExpr(form.value.cron.trim())) {
    cronError.value = 'Must have 6 fields: sec min hour day month day_of_week'
    return false
  }
  return true
}

function onCronBlur() {
  nextTick(() => {
    cronFocused.value = false
  })
}

function openAddDialog() {
  editingCron.value = null
  form.value = defaultForm()
  nameError.value = ''
  cronError.value = ''
  cronFocused.value = false
  cronBuildMode.value = false
  cronBuilder.value = { ...DEFAULT_BUILDER }
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
    model: cron.model,
  }
  nameError.value = ''
  cronError.value = ''
  cronFocused.value = false
  cronBuildMode.value = false
  syncBuilderFromExpr(cron.cron)
  showDialog.value = true
}

async function handleSave() {
  nameError.value = ''
  cronError.value = ''

  if (!validateName()) return
  if (!validateCron()) return

  saving.value = true
  try {
    if (editingCron.value) {
      await cronStore.updateCron(editingCron.value.id, { ...form.value })
      appStore.showMessage('Cron updated successfully', 'success')
    } else {
      await cronStore.addCron({ ...form.value })
      appStore.showMessage('Cron created successfully', 'success')
    }
    showDialog.value = false
    form.value = defaultForm()
    editingCron.value = null
  } catch {
    appStore.showMessage('Failed to save cron job', 'error')
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
  return new Date(dateStr).toLocaleDateString('zh-CN', {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

function formatNextRunTime(date: Date): string {
  return date.toLocaleDateString('zh-CN', {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

function nextRunRelative(date: Date): string {
  const now = new Date()
  const diffMs = date.getTime() - now.getTime()
  const diffMin = Math.round(diffMs / 60000)
  const diffHr = Math.round(diffMs / 3600000)
  const diffDay = Math.round(diffMs / 86400000)

  if (diffMin < 1) return 'now'
  if (diffMin < 60) return `in ${diffMin}m`
  if (diffHr < 24) return `in ${diffHr}h`
  return `in ${diffDay}d`
}

onMounted(() => {
  cronStore.fetchCrons()
  api.getModels().then(m => (availableModels.value = m)).catch(() => (availableModels.value = []))
})
</script>

<style scoped>
.crons-page {
  max-width: 1200px;
}

.page-header {
  flex-wrap: wrap;
  gap: 12px;
}

.header-icon-wrapper {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  background: linear-gradient(135deg, rgb(var(--v-theme-primary)), rgba(var(--v-theme-primary), 0.7));
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.add-btn {
  border-radius: 10px;
}

.stats-bar {
  background: rgb(var(--v-theme-surface));
  border: 1px solid rgba(var(--v-theme-on-surface), 0.08);
}

.stat-item {
  white-space: nowrap;
}

.cron-card {
  border-radius: 12px;
  transition: transform 0.2s ease, box-shadow 0.2s ease;
  overflow: hidden;
}

.cron-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.12) !important;
}

.cron-card.cron-disabled {
  opacity: 0.75;
}

.card-top-bar {
  height: 4px;
  width: 100%;
}

.card-top-bar.bg-grey {
  background-color: rgb(var(--v-theme-grey)) !important;
}

.info-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 8px 16px;
}

.info-item {
  min-width: 0;
}

.info-item-wide {
  grid-column: 1 / -1;
}

.info-label {
  display: block;
  font-size: 0.7rem;
  color: rgba(var(--v-theme-on-surface), 0.5);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  font-weight: 600;
  margin-bottom: 2px;
}

.info-value {
  font-size: 0.85rem;
  display: flex;
  align-items: center;
}

.cron-expr-badge {
  display: inline-block;
  background: rgba(var(--v-theme-primary), 0.08);
  color: rgb(var(--v-theme-primary));
  padding: 3px 10px;
  border-radius: 6px;
  font-family: 'Consolas', 'Cascadia Code', 'Courier New', monospace;
  font-size: 0.82rem;
  font-weight: 500;
  letter-spacing: 0.3px;
  max-width: 100%;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.empty-state {
  max-width: 480px;
  margin: 0 auto;
}

.empty-icon-wrapper {
  width: 120px;
  height: 120px;
  border-radius: 50%;
  background: rgba(var(--v-theme-primary), 0.06);
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.dialog-card {
  border-radius: 16px;
  overflow: hidden;
}

.dialog-top-bar {
  height: 4px;
  width: 100%;
}

.dialog-top-bar-thick {
  height: 5px;
}

.dialog-title-icon {
  width: 42px;
  height: 42px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.dialog-title-icon.bg-primary {
  background: linear-gradient(135deg, rgb(var(--v-theme-primary)), rgba(var(--v-theme-primary), 0.75));
}

.dialog-title-icon.bg-warning {
  background: linear-gradient(135deg, rgb(var(--v-theme-warning)), rgba(var(--v-theme-warning), 0.75));
}

.dialog-section {
  margin-bottom: 20px;
}

.dialog-section:last-of-type {
  margin-bottom: 0;
}

.section-header {
  padding-bottom: 4px;
  border-bottom: 1px solid rgba(var(--v-theme-on-surface), 0.06);
}

.enabled-toggle {
  display: flex;
  align-items: center;
  background: rgba(var(--v-theme-on-surface), 0.04);
  padding: 10px 14px;
  border-radius: 8px;
  height: 56px;
}

.schedule-section {
  background: rgba(var(--v-theme-secondary), 0.03);
  border: 1px solid rgba(var(--v-theme-secondary), 0.1);
  border-radius: 10px;
  padding: 16px 18px 18px;
  margin-bottom: 20px;
}

.schedule-section .section-header {
  border-bottom-color: rgba(var(--v-theme-secondary), 0.12);
}

.cron-builder-panel {
  background: rgba(var(--v-theme-surface-variant), 0.4);
  border: 1px solid rgba(var(--v-theme-on-surface), 0.08);
}

.builder-row {
  display: flex;
  gap: 10px;
  margin-bottom: 10px;
}

.builder-row:last-child {
  margin-bottom: 0;
}

.builder-field {
  flex: 1;
  min-width: 0;
}

.builder-label {
  display: block;
  font-size: 0.65rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.4px;
  color: rgba(var(--v-theme-on-surface), 0.45);
  margin-bottom: 3px;
  padding-left: 2px;
}

.cron-description {
  background: rgba(var(--v-theme-secondary), 0.06);
  display: flex;
  align-items: center;
}

.next-runs-panel {
  background: rgba(var(--v-theme-primary), 0.03);
  border: 1px solid rgba(var(--v-theme-primary), 0.1);
  border-radius: 10px;
}

.next-runs-icon-wrapper {
  width: 28px;
  height: 28px;
  border-radius: 8px;
  background: rgba(var(--v-theme-primary), 0.1);
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.schedule-helper-section {
  margin-top: 8px;
}

.preset-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 6px;
}

.preset-chip {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  padding: 8px 10px;
  border-radius: 8px;
  border: 1px solid rgba(var(--v-theme-on-surface), 0.08);
  cursor: pointer;
  transition: all 0.15s ease;
  user-select: none;
}

.preset-chip:hover {
  background: rgba(var(--v-theme-primary), 0.04);
  border-color: rgba(var(--v-theme-primary), 0.25);
}

.preset-chip.preset-active {
  background: rgba(var(--v-theme-primary), 0.08);
  border-color: rgb(var(--v-theme-primary));
}

.preset-label {
  font-weight: 600;
  margin-bottom: 2px;
  color: rgba(var(--v-theme-on-surface), 0.85);
}

.preset-expr {
  font-family: 'Consolas', 'Cascadia Code', 'Courier New', monospace;
  font-size: 0.68rem;
  color: rgba(var(--v-theme-on-surface), 0.5);
  letter-spacing: 0.2px;
}

.preset-chip.preset-active .preset-expr {
  color: rgb(var(--v-theme-primary));
}

.syntax-alert {
  border-radius: 8px;
}

.syntax-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 4px 12px;
}

.syntax-item {
  display: flex;
  align-items: center;
  gap: 6px;
}

.syntax-expr {
  background: rgba(var(--v-theme-on-surface), 0.06);
  padding: 1px 6px;
  border-radius: 4px;
  font-family: 'Consolas', 'Cascadia Code', 'Courier New', monospace;
  font-size: 0.68rem;
  white-space: nowrap;
}

.dialog-actions {
  border-top: 1px solid rgba(var(--v-theme-on-surface), 0.06);
  background: rgba(var(--v-theme-on-surface), 0.02);
}

.gap-2 {
  gap: 6px;
}

.h-100 {
  height: 100%;
}

.history-list {
  background: transparent;
  max-height: 420px;
  overflow-y: auto;
}

.history-item {
  border-left: 3px solid transparent;
  transition: background 0.15s ease;
}

.history-item.history-success {
  border-left-color: rgb(var(--v-theme-success));
}

.history-item.history-error {
  border-left-color: rgb(var(--v-theme-error));
}

.log-output {
  white-space: pre-wrap;
  word-break: break-all;
  font-family: 'Consolas', 'Cascadia Code', 'Courier New', monospace;
  font-size: 11px;
  max-height: 100px;
  overflow-y: auto;
  background: rgba(var(--v-theme-on-surface), 0.03);
  padding: 6px 10px;
  border-radius: 6px;
  margin: 0;
}
</style>
