<template>
  <div class="min-h-screen bg-gray-50">
    <nav class="bg-white border-b shadow-sm">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between h-16">
          <div class="flex items-center">
            <span class="text-xl font-bold text-blue-600">Nexow Station</span>
          </div>
          <div class="flex items-center space-x-8">
            <NuxtLink 
              to="/dashboard" 
              class="text-gray-700 hover:text-blue-600 px-3 py-2 rounded-md text-sm font-medium"
              active-class="text-blue-600 bg-blue-50"
            >
              Dashboard
            </NuxtLink>
            <NuxtLink 
              to="/assets" 
              class="text-gray-700 hover:text-blue-600 px-3 py-2 rounded-md text-sm font-medium"
              active-class="text-blue-600 bg-blue-50"
            >
              Assets
            </NuxtLink>
            <NuxtLink 
              to="/simulate" 
              class="text-gray-700 hover:text-blue-600 px-3 py-2 rounded-md text-sm font-medium"
              active-class="text-blue-600 bg-blue-50"
            >
              Simulate
            </NuxtLink>
          </div>
        </div>
      </div>
    </nav>
    <main class="max-w-7xl mx-auto">
      <NuxtPage />
    </main>
  </div>
</template>
