<script setup lang="ts">
const config = useRuntimeConfig()
const store = useEngineStore()

onMounted(() => {
  const wsUrl = config.public.apiBaseUrl.replace('http', 'ws') + '/ws/stream'
  store.connectWs(wsUrl)
})

onUnmounted(() => {
  store.disconnect()
})

const latestMetrics = computed(() => store.metrics.at(-1))
const latestBar = computed(() => store.bars.at(-1))
</script>

<template>
  <div class="p-6">
    <h1 class="text-2xl font-bold mb-6">Dashboard</h1>
    
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
      <div class="border rounded-lg p-4 bg-white shadow">
        <h2 class="text-lg font-semibold mb-2">Latest Metrics</h2>
        <div v-if="latestMetrics" class="space-y-2">
          <div><span class="font-medium">PnL:</span> ${{ latestMetrics.pnl.toFixed(2) }}</div>
          <div><span class="font-medium">Max Drawdown:</span> {{ (latestMetrics.max_drawdown * 100).toFixed(2) }}%</div>
          <div><span class="font-medium">Win Rate:</span> {{ (latestMetrics.win_rate * 100).toFixed(2) }}%</div>
          <div><span class="font-medium">Trades:</span> {{ latestMetrics.trades }}</div>
        </div>
        <div v-else class="text-gray-500">No metrics yet</div>
      </div>

      <div class="border rounded-lg p-4 bg-white shadow">
        <h2 class="text-lg font-semibold mb-2">Latest Bar</h2>
        <div v-if="latestBar" class="space-y-2">
          <div><span class="font-medium">Symbol:</span> {{ latestBar.symbol }}</div>
          <div><span class="font-medium">Close:</span> ${{ latestBar.close.toFixed(2) }}</div>
          <div><span class="font-medium">High:</span> ${{ latestBar.high.toFixed(2) }}</div>
          <div><span class="font-medium">Low:</span> ${{ latestBar.low.toFixed(2) }}</div>
        </div>
        <div v-else class="text-gray-500">No bars yet</div>
      </div>
    </div>

    <div class="border rounded-lg p-4 bg-white shadow">
      <h2 class="text-lg font-semibold mb-2">Connection Status</h2>
      <div :class="store.running ? 'text-green-600' : 'text-red-600'">
        {{ store.running ? '● Connected' : '● Disconnected' }}
      </div>
      <div class="text-sm text-gray-500 mt-1">
        Total bars: {{ store.bars.length }} | Total metrics: {{ store.metrics.length }}
      </div>
    </div>
  </div>
</template>
