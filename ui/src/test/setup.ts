import { render } from 'vitest-browser-vue'

// Vitest setup file
// This file runs before each test file

// Make page available globally in browser mode
import { page } from 'vitest/browser'
import '../style.css'

declare global {
  var page: import('vitest/browser').BrowserPage
  var setup: (setup: () => any) => ReturnType<typeof render>
}

;(globalThis as any).page = page

;(globalThis as any).setup = (setup: () => any) => {
  return render({ setup })
}
