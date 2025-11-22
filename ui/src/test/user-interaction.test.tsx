/** @jsxImportSource vue */
import { expect, test, vi } from 'vitest'
import { Hexboard } from '../lib'
import { index, positions } from '@bedard/hexchess'
import { ref, nextTick } from 'vue'

test('update mouseover position on hover', async () => {
  const mouseover = ref(-1)

  setup(() => {
    return () => <>
      <Hexboard v-model:mouseover-position={mouseover.value} />
    </>
  })

  for (const p of positions) {
    await page.getByTestId(`position-${p}`).hover()
    await expect(mouseover.value).toBe(index(p))
  }
})

test('calls handler on position click', async () => {
  const onClickPosition = vi.fn()

  setup(() => {
    return () => <>
      <Hexboard onClickPosition={onClickPosition} />
    </>
  })

  await page.getByTestId('position-f6').click()
  await expect(onClickPosition).toHaveBeenCalledOnce()
  await expect(onClickPosition).toHaveBeenCalledWith(index('f6'))
})

test('flipped board', async () => {
  const flipped = ref(false)

  setup(() => {
    return () => <>
      <Hexboard flipped={flipped.value} />
    </>
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

test('custom colors', async () => {
  setup(() => {
    return () => <>
      <Hexboard
        options={{
          colors: ['red', 'green', 'blue'],
        }}
      />
    </>
  })

  await page.getByTestId('position-a6').hover()
  await expect(page.getByTestId('position-a6')).toHaveStyle({ fill: 'red' })

  await page.getByTestId('position-b7').hover()
  await expect(page.getByTestId('position-b7')).toHaveStyle({ fill: 'green' })

  await page.getByTestId('position-c8').hover()
  await expect(page.getByTestId('position-c8')).toHaveStyle({ fill: 'blue' })
})

test('label colors', async () => {
  setup(() => {
    return () => <>
      <Hexboard
        options={{
          labelColor: 'red',
          labelActiveColor: 'green',
          labelInactiveColor: 'blue',
        }}
      />
    </>
  })

  // When no mouseover, all labels should have default labelColor (red)
  await expect.element(page.getByTestId('label-a')).toHaveStyle({ fill: 'red' })
  await expect.element(page.getByTestId('label-b')).toHaveStyle({ fill: 'red' })
  await expect.element(page.getByTestId('label-c')).toHaveStyle({ fill: 'red' })

  // When hovering over f6, labels 'f' and '6' should be active (green)
  await page.getByTestId('position-f6').hover()
  await expect.element(page.getByTestId('label-f')).toHaveStyle({ fill: 'green' })
  await expect.element(page.getByTestId('label-6').first()).toHaveStyle({ fill: 'green' })
  await expect.element(page.getByTestId('label-6').last()).toHaveStyle({ fill: 'green' })

  // Other labels should be inactive (blue)
  await expect.element(page.getByTestId('label-a')).toHaveStyle({ fill: 'blue' })
  await expect.element(page.getByTestId('label-1').first()).toHaveStyle({ fill: 'blue' })
  await expect.element(page.getByTestId('label-1').last()).toHaveStyle({ fill: 'blue' })
})