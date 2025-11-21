/** @jsxImportSource vue */
import { expect, test } from 'vitest'
import { Hexboard } from '../lib'
import { position, positions } from '@bedard/hexchess'
import { page } from 'vitest/browser'
import { ref } from 'vue'
import { render } from 'vitest-browser-vue'

test('properly handles form inputs', async () => {
  render({
    setup() {
      const mouseover = ref(-1)

      return () => <>
        <Hexboard v-model:mouseover={mouseover.value} />

        <div
          v-text={position(mouseover.value)}
          data-testid="assert" />
      </>
    }
  })

  const assertLocator = page.getByTestId('assert')
  await expect.element(assertLocator).toBeInTheDocument()
  
  for (const p of positions) {
    await page.getByTestId(`position-${p}`).hover()
    await expect.element(assertLocator).toHaveTextContent(p)
  }
})