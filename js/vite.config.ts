/// <reference types="vitest" />
import { defineConfig } from 'vite'

export default defineConfig({
  test: {
    include: ['tests/**/*.spec.ts'],
    exclude: ['node_modules'],
    typecheck: {
      enabled: true,
    },
  },
})
