/** @jsxImportSource vue */
import { expect, test, vi } from 'vitest'
import { Hexboard } from '../lib'
import { index, position, positions } from '@bedard/hexchess'
import { page } from 'vitest/browser'
import { ref, nextTick } from 'vue'
import { render } from 'vitest-browser-vue'

test('update mouseover position on hover', async () => {
  render({
    setup() {
      const mouseover = ref(-1)

      return () => <>
        <Hexboard v-model:mouseover-position={mouseover.value} />

        <div
          v-text={position(mouseover.value)}
          data-testid="assertion" />
      </>
    },
  })

  const assertiontLocator = page.getByTestId('assertion')
  await expect.element(assertiontLocator).toBeInTheDocument()
  
  for (const p of positions) {
    await page.getByTestId(`position-${p}`).hover()
    await expect.element(assertiontLocator).toHaveTextContent(p)
  }
})

test('calls handler on position click', async () => {
  const onClickPosition = vi.fn()

  render({
    setup() {
      return () => <>
        <Hexboard onClickPosition={onClickPosition} />
      </>
    },
  })

  await page.getByTestId('position-f6').click()
  await expect(onClickPosition).toHaveBeenCalledOnce()
  await expect(onClickPosition).toHaveBeenCalledWith(index('f6'))
})

test('flipped board', async () => {
  const flipped = ref(false)

  render({
    setup() {
      return () => <>
        <Hexboard flipped={flipped.value} />
      </>
    },
  })

  const { y: startY } = page
    .getByTestId('position-f1')
    .element()
    .getBoundingClientRect()

  flipped.value = true

  await nextTick()

  const { y: endY } = page
    .getByTestId('position-f1')
    .element()
    .getBoundingClientRect()
  
  await expect(startY).toBeGreaterThan(endY) // f1 starts at the bottom, then moves to top
})
