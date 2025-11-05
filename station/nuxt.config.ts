// https://nuxt.com/docs/api/configuration/nuxt-config
import { fileURLToPath } from 'url'

export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },
  ssr: false,
  srcDir: './',
  typescript: {
    strict: true,
    typeCheck: false
  },
  modules: [
    '@nuxt/eslint',
    '@nuxt/ui',
    '@nuxt/test-utils',
    '@nuxt/scripts',
    '@pinia/nuxt'
  ],
  css: ['@/assets/css/tailwind.css'],
  runtimeConfig: {
    public: {
      apiBaseUrl: process.env.NUXT_PUBLIC_API_BASE_URL || 'http://127.0.0.1:8080',
    },
  },
  vite: {
    resolve: {
      alias: {
        '@': fileURLToPath(new URL('./', import.meta.url)),
        '@nexow': fileURLToPath(new URL('./', import.meta.url)),
      },
    },
  },
  app: {
    head: {
      title: 'Nexow Station'
    }
  },
})
