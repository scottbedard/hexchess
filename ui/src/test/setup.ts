// Vitest setup file
// This file runs before each test file

// Make page available globally in browser mode
import { page } from 'vitest/browser'
import '../style.css'

declare global {
  // eslint-disable-next-line no-var
  var page: import('playwright').Page
}

;(globalThis as any).page = page

