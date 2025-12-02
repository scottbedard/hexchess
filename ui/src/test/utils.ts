import { expect } from 'vitest'
import { nextTick } from 'vue'
import { position } from '@bedard/hexchess'
import type { BrowserPage } from 'vitest/browser'

/**
 * Moves a piece by clicking on the source position and then the target position.
 * This simulates the click-to-move interaction pattern
 */
export async function movePiece(page: BrowserPage, from: string | number, to: string | number): Promise<void> {
  const fromId = typeof from === 'string' ? `position-${from}` : `position-${position(from)}`
  const toId = typeof to === 'string' ? `position-${to}` : `position-${position(to)}`
  
  await expect.element(page.getByTestId(fromId)).toBeVisible()
  await expect.element(page.getByTestId(toId)).toBeVisible()
  
  // Click on the source position to select it
  await page.getByTestId(fromId).click()
  await nextTick()
  
  // Click on the target position to move
  await page.getByTestId(toId).click()
  await nextTick()
}

