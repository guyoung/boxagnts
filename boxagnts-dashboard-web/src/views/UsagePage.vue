<template>
  <div>
    <div class="d-flex align-center mb-6">
      <v-icon size="32" color="primary" class="mr-3">mdi-chart-bar</v-icon>
      <h1 class="text-h4 font-weight-bold">Usage Analytics</h1>
    </div>

    <v-row class="mb-6">
      <v-col cols="12" md="3" v-for="card in summaryCards" :key="card.title">
        <v-card :color="card.color" variant="tonal">
          <v-card-item>
            <template #prepend>
              <v-icon :icon="card.icon" size="28" :color="card.iconColor" />
            </template>
            <v-card-title class="text-h5 font-weight-bold">{{ card.value }}</v-card-title>
            <v-card-subtitle>{{ card.title }}</v-card-subtitle>
          </v-card-item>
        </v-card>
      </v-col>
    </v-row>

    <v-card class="mb-6">
      <v-card-title class="d-flex align-center">
        <v-icon start>mdi-chart-line</v-icon>
        Daily Token Usage
      </v-card-title>
      <v-card-text>
        <div v-if="usageStore.summary?.by_date?.length" style="height: 300px">
          <canvas ref="chartRef"></canvas>
        </div>
        <p v-else class="text-center py-8 text-medium-emphasis">No usage data available yet</p>
      </v-card-text>
    </v-card>

    <v-card class="mb-6">
      <v-card-title class="d-flex align-center">
        <v-icon start>mdi-cube</v-icon>
        By Model
      </v-card-title>
      <v-card-text>
        <v-list v-if="usageStore.summary?.by_model?.length">
          <v-list-item
            v-for="m in usageStore.summary.by_model"
            :key="m.model"
            rounded="lg"
          >
            <template #prepend>
              <v-icon>mdi-cube</v-icon>
            </template>
            <v-list-item-title>{{ m.model }}</v-list-item-title>
            <template #append>
              <div class="text-right">
                <div class="text-body-2 font-weight-medium">
                  {{ formatNum(m.total_tokens) }} tokens
                </div>
                <div class="text-caption text-success">${{ m.total_cost.toFixed(4) }}</div>
              </div>
            </template>
          </v-list-item>
        </v-list>
        <p v-else class="text-center py-4 text-medium-emphasis">No model breakdown available</p>
      </v-card-text>
    </v-card>

    <v-card>
      <v-card-title class="d-flex align-center">
        <v-icon start>mdi-folder</v-icon>
        By Project
      </v-card-title>
      <v-card-text>
        <v-list v-if="usageStore.summary?.by_project?.length">
          <v-list-item
            v-for="p in usageStore.summary.by_project"
            :key="p.project_name"
            rounded="lg"
          >
            <template #prepend>
              <v-icon>mdi-folder</v-icon>
            </template>
            <v-list-item-title>{{ p.project_name }}</v-list-item-title>
            <template #append>
              <div class="text-right">
                <div class="text-body-2 font-weight-medium">
                  {{ formatNum(p.total_tokens) }} tokens
                </div>
                <div class="text-caption text-success">${{ p.total_cost.toFixed(4) }}</div>
              </div>
            </template>
          </v-list-item>
        </v-list>
        <p v-else class="text-center py-4 text-medium-emphasis">No project breakdown available</p>
      </v-card-text>
    </v-card>
  </div>
</template>

<script setup lang="ts">
import { onMounted, computed, ref, watch, nextTick } from 'vue'
import { useUsageStore } from '@/stores/usage'
import { Chart, LineController, CategoryScale, LinearScale, PointElement, LineElement, Filler, Tooltip, Legend } from 'chart.js'

Chart.register(LineController, CategoryScale, LinearScale, PointElement, LineElement, Filler, Tooltip, Legend)

const usageStore = useUsageStore()
const chartRef = ref<HTMLCanvasElement | null>(null)
let chartInstance: Chart | null = null

function formatNum(n: number) {
  return n >= 1e6 ? `${(n / 1e6).toFixed(1)}M` : n >= 1e3 ? `${(n / 1e3).toFixed(1)}K` : `${n}`
}

onMounted(async () => {
  console.log("UsagePage onMounted")

  await usageStore.fetchUsage()
  await nextTick()
  renderChart()
})

watch(() => usageStore.summary, async () => {
  await nextTick()
  renderChart()
})

function renderChart() {
  if (chartInstance) {
    chartInstance.destroy()
    chartInstance = null
  }
  if (!chartRef.value) return

  const daily = usageStore.summary?.by_date
  if (!daily?.length) return

  const totalTokens = daily.reduce((s, d) => s + d.total_tokens, 0)
  const totalCost = daily.reduce((s, d) => s + d.total_cost, 0)
  const tokensChartCost = daily.map(d =>
    totalTokens > 0 ? (d.total_tokens / totalTokens) * totalCost * 100 : 0
  )

  chartInstance = new Chart(chartRef.value, {
    type: 'line',
    data: {
      labels: daily.map(d => d.date),
      datasets: [{
        label: 'Tokens',
        data: daily.map(d => d.total_tokens),
        borderColor: '#89B4FA',
        backgroundColor: 'rgba(137,180,250,0.1)',
        fill: true,
        tension: 0.4,
        yAxisID: 'y',
      }, {
        label: 'Cost (est)',
        data: tokensChartCost,
        borderColor: '#A6E3A1',
        backgroundColor: 'rgba(166,227,161,0.1)',
        fill: true,
        tension: 0.4,
        yAxisID: 'y1',
      }],
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      plugins: {
        legend: { labels: { color: '#B0B0C0' } },
      },
      scales: {
        x: {
          ticks: { color: '#B0B0C0', maxTicksLimit: 15 },
          grid: { color: 'rgba(255,255,255,0.05)' },
        },
        y: {
          type: 'linear',
          display: true,
          position: 'left',
          ticks: { color: '#89B4FA' },
          grid: { color: 'rgba(255,255,255,0.05)' },
          beginAtZero: true,
        },
        y1: {
          type: 'linear',
          display: true,
          position: 'right',
          ticks: { color: '#A6E3A1' },
          grid: { drawOnChartArea: false },
          beginAtZero: true,
        },
      },
    },
  })
}

const summaryCards = computed(() => [
  {
    title: 'Total Tokens',
    value: usageStore.summary ? formatNum(usageStore.summary.total_tokens) : '0',
    icon: 'mdi-code-tags',
    color: 'primary',
    iconColor: 'primary',
  },
  {
    title: 'Total Cost',
    value: usageStore.summary ? `$${usageStore.summary.total_cost.toFixed(4)}` : '$0',
    icon: 'mdi-currency-usd',
    color: 'success',
    iconColor: 'success',
  },
  {
    title: 'Models Used',
    value: usageStore.summary ? usageStore.summary.by_model.length : 0,
    icon: 'mdi-cube',
    color: 'secondary',
    iconColor: 'secondary',
  },
  {
    title: 'Projects',
    value: usageStore.summary ? usageStore.summary.by_project.length : 0,
    icon: 'mdi-folder',
    color: 'accent',
    iconColor: 'accent',
  },
])
</script>
