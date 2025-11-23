/** @jsxImportSource vue */
import { expect, test, vi } from 'vitest'
import { Hexboard } from '../lib'
import { index } from '@bedard/hexchess'
import { ref, nextTick } from 'vue'

test('update mouseover position on hover', async () => {
  const active = ref(false)
  const mouseover = ref(-1)

  setup(() => {
    return () => <Hexboard
      active={active.value}
      v-model:mouseover-position={mouseover.value}
    />
  })

  await page.getByTestId('position-f6').hover()
  await expect(mouseover.value).toBe(-1)

  active.value = true
  await nextTick()

  await page.getByTestId('position-f1').hover()
  await expect(mouseover.value).toBe(index('f1'))
})

test('calls handler on position click', async () => {
  const active = ref(false)
  const onClickPosition = vi.fn()

  setup(() => {
    return () => <Hexboard
      active={active.value}
      onClickPosition={onClickPosition}
    />
  })

  await page.getByTestId('position-f6').click()
  await expect(onClickPosition).not.toHaveBeenCalled()

  active.value = true
  await nextTick()

  await page.getByTestId('position-f6').click()
  await expect(onClickPosition).toHaveBeenCalledOnce()
  await expect(onClickPosition).toHaveBeenCalledWith(index('f6'))
})

test('flipped board', async () => {
  const flipped = ref(false)

  setup(() => {
    return () => <Hexboard flipped={flipped.value} />
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
    return () => <Hexboard
      options={{
        colors: ['red', 'green', 'blue'],
      }}
    />
  })

  await page.getByTestId('position-a6').hover()
  await expect(page.getByTestId('position-a6')).toHaveStyle({ fill: 'red' })

  await page.getByTestId('position-b7').hover()
  await expect(page.getByTestId('position-b7')).toHaveStyle({ fill: 'green' })

  await page.getByTestId('position-c8').hover()
  await expect(page.getByTestId('position-c8')).toHaveStyle({ fill: 'blue' })
})

test('labels and label colors', async () => {
  const active = ref(false)
  const labels = ref(false)

  setup(() => {
    return () => <Hexboard
      active={active.value}
      options={{
        labels: labels.value,
        labelColor: 'red',
        labelActiveColor: 'green',
        labelInactiveColor: 'blue',
      }}
    />
  })

  // Labels only show when enabled
  await expect.element(page.getByTestId('position-a1')).toBeInTheDocument()
  await expect.element(page.getByTestId('label-a')).not.toBeInTheDocument()
  labels.value = true
  await nextTick()
  await expect.element(page.getByTestId('label-a')).toBeVisible()

  // When no mouseover, all labels should have default labelColor (red)
  await expect.element(page.getByTestId('label-a')).toHaveStyle({ fill: 'red' })
  await expect.element(page.getByTestId('label-b')).toHaveStyle({ fill: 'red' })
  await expect.element(page.getByTestId('label-c')).toHaveStyle({ fill: 'red' })

  // No mouse events should be bound when inactive
  await page.getByTestId('position-f6').hover()
  await nextTick()
  await expect.element(page.getByTestId('label-a')).toHaveStyle({ fill: 'red' })
  await expect.element(page.getByTestId('label-b')).toHaveStyle({ fill: 'red' })
  await expect.element(page.getByTestId('label-c')).toHaveStyle({ fill: 'red' })

  // When hovering over f6, labels 'f' and '6' should be active (green)
  active.value = true
  await nextTick()
  await page.getByTestId('position-f5').hover()
  await expect.element(page.getByTestId('label-f')).toHaveStyle({ fill: 'green' })
  await expect.element(page.getByTestId('label-5').first()).toHaveStyle({ fill: 'green' })
  await expect.element(page.getByTestId('label-5').last()).toHaveStyle({ fill: 'green' })

  // Other labels should be inactive (blue)
  await expect.element(page.getByTestId('label-a')).toHaveStyle({ fill: 'blue' })
  await expect.element(page.getByTestId('label-1').first()).toHaveStyle({ fill: 'blue' })
  await expect.element(page.getByTestId('label-1').last()).toHaveStyle({ fill: 'blue' })
})

test('targets array controls rendering of target circles', async () => {
  const targets = ref<number[]>([])

  setup(() => {
    return () => <Hexboard
      active={true}
      targets={targets.value}
      options={{
        targetColor: 'red',
      }}
    />
  })

  await expect.element(page.getByTestId('target-a1')).not.toBeInTheDocument()

  targets.value = [index('a1')]

  await expect.element(page.getByTestId('target-a1')).toBeVisible()
  await expect.element(page.getByTestId('target-a1')).toHaveStyle({ fill: 'red' })
})

test('selected model is set when clicking a position', async () => {
  const active = ref(false)
  const selected = ref<number | null>(null)

  setup(() => {
    return () => <Hexboard
      active={active.value}
      v-model:selected={selected.value}
      options={{
        selectedColor: 'red',
      }}
    />
  })

  // Initially, no selected path should be in the document
  await expect.element(page.getByTestId('selected-f6')).not.toBeInTheDocument()
  await expect.element(page.getByTestId('selected-a1')).not.toBeInTheDocument()

  // Clicking when inactive should not set selected
  await page.getByTestId('position-f6').click()
  await expect(selected.value).toBeNull()
  await expect.element(page.getByTestId('selected-f6')).not.toBeInTheDocument()

  // Activate the board
  active.value = true
  await nextTick()

  // Clicking when active should set selected
  await page.getByTestId('position-f6').click()
  await expect(selected.value).toBe(index('f6'))
  await expect.element(page.getByTestId('selected-f6')).toBeVisible()
  await expect.element(page.getByTestId('selected-f6')).toHaveStyle({ fill: 'red' })

  // Clicking another position should update selected
  await page.getByTestId('position-a1').click()
  await expect(selected.value).toBe(index('a1'))
  await expect.element(page.getByTestId('selected-f6')).not.toBeInTheDocument()
  await expect.element(page.getByTestId('selected-a1')).toBeVisible()
  await expect.element(page.getByTestId('selected-a1')).toHaveStyle({ fill: 'red' })
})
