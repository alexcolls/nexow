<script setup lang="ts">
const config = useRuntimeConfig()
const { data, pending, error } = await useFetch(config.public.apiBaseUrl + '/api/assets')
</script>

<template>
  <div class="p-6">
    <h1 class="text-2xl font-bold mb-6">Assets</h1>
    
    <div v-if="pending" class="text-gray-500">Loading assets...</div>
    
    <div v-else-if="error" class="text-red-600">
      Error loading assets: {{ error.message }}
    </div>
    
    <div v-else class="border rounded-lg p-4 bg-white shadow">
      <div v-if="data && data.length > 0" class="space-y-2">
        <div v-for="(asset, index) in data" :key="index" class="py-2 border-b last:border-b-0">
          {{ asset }}
        </div>
      </div>
      <div v-else class="text-gray-500">
        No assets available yet. Assets will appear here once exchange connectivity is implemented.
      </div>
    </div>
  </div>
</template>
