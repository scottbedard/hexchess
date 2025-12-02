import { nextTick } from 'vue'
import type { BrowserPage } from 'vitest/browser'
import { render } from 'vitest-browser-vue'

/**
 * Moves a piece by clicking on the source position and then the target position.
 * This simulates the click-to-move interaction pattern.
 */
export async function movePiece(page: BrowserPage, from: string | number, to: string | number): Promise<void> {
  const fromTestId = typeof from === 'string' ? `position-${from}` : `position-${from}`
  const toTestId = typeof to === 'string' ? `position-${to}` : `position-${to}`
  
  // Click on the source position to select it
  await page.getByTestId(fromTestId).click()
  await nextTick()
  
  // Click on the target position to move
  await page.getByTestId(toTestId).click()
  await nextTick()
}

/**
 * Sets up a Vue component for testing in the browser.
 */
export function setup(setup: () => any): ReturnType<typeof render> {
  return render({ setup })
}

