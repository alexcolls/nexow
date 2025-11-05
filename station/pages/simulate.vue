<script setup lang="ts">
const config = useRuntimeConfig()
const running = ref(false)
const symbols = ref('BTC-USD,ETH-USD')
const barInterval = ref(250)
const lengthBars = ref(2000)
const rfTrees = ref(100)
const rfMaxDepth = ref(8)
const trainSplit = ref(0.7)
const startingCash = ref(100000)
const error = ref('')

async function startSimulation() {
  error.value = ''
  try {
    const res = await $fetch(config.public.apiBaseUrl + '/api/sim/start', {
      method: 'POST',
      body: {
        symbols: symbols.value.split(',').map(s => s.trim()),
        bar_interval_ms: barInterval.value,
        length_bars: lengthBars.value,
        rf_trees: rfTrees.value,
        rf_max_depth: rfMaxDepth.value,
        train_split: trainSplit.value,
        mode: 'simulate',
        starting_cash: startingCash.value
      }
    })
    running.value = true
  } catch (e: any) {
    error.value = e.message || 'Failed to start simulation'
  }
}
</script>

<template>
  <div class="p-6">
    <h1 class="text-2xl font-bold mb-6">Simulate</h1>
    
    <div class="border rounded-lg p-6 bg-white shadow max-w-2xl">
      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium mb-1">Symbols (comma-separated)</label>
          <input 
            v-model="symbols" 
            class="w-full border rounded px-3 py-2" 
            placeholder="BTC-USD,ETH-USD"
          />
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium mb-1">Bar Interval (ms)</label>
            <input 
              v-model.number="barInterval" 
              type="number" 
              class="w-full border rounded px-3 py-2"
            />
          </div>

          <div>
            <label class="block text-sm font-medium mb-1">Length (bars)</label>
            <input 
              v-model.number="lengthBars" 
              type="number" 
              class="w-full border rounded px-3 py-2"
            />
          </div>
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium mb-1">RF Trees</label>
            <input 
              v-model.number="rfTrees" 
              type="number" 
              class="w-full border rounded px-3 py-2"
            />
          </div>

          <div>
            <label class="block text-sm font-medium mb-1">RF Max Depth</label>
            <input 
              v-model.number="rfMaxDepth" 
              type="number" 
              class="w-full border rounded px-3 py-2"
            />
          </div>
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium mb-1">Train/Test Split</label>
            <input 
              v-model.number="trainSplit" 
              type="number" 
              step="0.1" 
              min="0" 
              max="1"
              class="w-full border rounded px-3 py-2"
            />
          </div>

          <div>
            <label class="block text-sm font-medium mb-1">Starting Cash ($)</label>
            <input 
              v-model.number="startingCash" 
              type="number" 
              class="w-full border rounded px-3 py-2"
            />
          </div>
        </div>

        <div v-if="error" class="text-red-600 text-sm">
          {{ error }}
        </div>

        <button 
          @click="startSimulation" 
          :disabled="running"
          class="w-full bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed"
        >
          {{ running ? 'Simulation Running...' : 'Start Simulation' }}
        </button>

        <div class="text-sm text-gray-500 text-center">
          After starting, view real-time results in the Dashboard
        </div>
      </div>
    </div>
  </div>
</template>
