/** @jsxImportSource vue */
import { expect, test, vi } from 'vitest'
import { Hexboard } from '../lib'
import { index, position, positions } from '@bedard/hexchess'
import { page } from 'vitest/browser'
import { ref } from 'vue'
import { render } from 'vitest-browser-vue'

test('update mouseover position on hover', async () => {
  render({
    setup() {
      const mouseover = ref(-1)

      return () => <>
        <Hexboard v-model:mouseover-position={mouseover.value} />

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

test('calls handler on position click', async () => {
  const onClickPosition = vi.fn()

  render({
    setup() {
      return () => <>
        <Hexboard onClickPosition={onClickPosition} />
      </>
    }
  })

  await page.getByTestId('position-f6').click()
  await expect(onClickPosition).toHaveBeenCalledOnce()
  await expect(onClickPosition).toHaveBeenCalledWith(index('f6'))
})
