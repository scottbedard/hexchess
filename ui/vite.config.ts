import { defineConfig } from 'vite'
import { fileURLToPath } from 'node:url'
import { resolve } from 'node:path'
import dts from 'vite-plugin-dts'
import tailwindcss from '@tailwindcss/vite'
import vue from '@vitejs/plugin-vue'

const __dirname = fileURLToPath(new URL('.', import.meta.url))

export default defineConfig({
  plugins: [
    dts({
      include: ['src/**/*'],
      outDir: 'dist',
      rollupTypes: true
    }),
    tailwindcss(),
    vue(),
  ],
  build: {
    lib: {
      entry: resolve(__dirname, 'src/lib/index.ts'),
      name: '@bedard/hexchess-ui',
      fileName: 'index',
      formats: ['es']
    },
    rollupOptions: {
      external: ['@bedard/hexchess', 'vue']
    }
  },
  server: {
    port: 3000,
    open: true
  }
})

