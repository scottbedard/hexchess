import { defineConfig } from 'vitest/config'
import { fileURLToPath } from 'node:url'
import { playwright } from '@vitest/browser-playwright'
import { resolve } from 'node:path'
import tailwindcss from '@tailwindcss/vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'

const __dirname = fileURLToPath(new URL('.', import.meta.url))

// Get browser from environment variable, default to chromium
const browser = (process.env.VITEST_BROWSER || 'chromium') as 'chromium' | 'firefox' | 'webkit'

export default defineConfig({
  plugins: [
    tailwindcss(),
    vue(),
    vueJsx(),
  ],
  test: {
    browser: {
      provider: playwright(),
      enabled: true,
      instances: [
        { browser },
      ],
    },
    environment: 'jsdom',
    testTimeout: 3000,
  },
  resolve: {
    alias: {
      '@': resolve(__dirname, './src'),
    },
  },
})

