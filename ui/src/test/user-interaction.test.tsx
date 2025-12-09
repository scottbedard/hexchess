/** @jsxImportSource vue */
import { expect, test, vi } from 'vitest'
import { Hexboard } from '../lib'
import { Hexchess } from '@bedard/hexchess'
import { index, San } from '@bedard/hexchess'
import { movePiece, setup } from './utils'
import { page } from 'vitest/browser'
import { ref, nextTick } from 'vue'
import { userEvent } from 'vitest/browser'

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
      active
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

test('autoselect targets', async () => {
  const targets = ref<number[]>([])

  setup(() => {
    return () => <Hexboard
      active
      autoselect
      targets={targets.value}
    />
  })

  await page.getByTestId('position-f5').click()
  await expect.element(page.getByTestId('target-f6')).toBeVisible()
})

test('select options and logic', async () => {
  const active = ref(false)
  const selected = ref<number | null>(null)

  setup(() => {
    return () => <>
      <Hexboard
        v-model:selected={selected.value}
        active={active.value}
        autoselect
        playing
        options={{
          selectedColor: 'red',
        }}
      />

      <div
        v-text={selected.value}
        data-testid="assertion" />
    </>
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
  await page.getByTestId('position-f5').click()
  await expect(selected.value).toBe(index('f5'))
  await expect.element(page.getByTestId('selected-f5')).toBeVisible()
  await expect.element(page.getByTestId('selected-f5')).toHaveStyle({ fill: 'red' })

  // Clicking an unoccupied position should deselect
  await page.getByTestId('position-a1').click()
  await expect.element(page.getByTestId('assertion')).toBeEmptyDOMElement()

  // Escape should clear selected
  await page.getByTestId('position-f5').click()
  await expect.element(page.getByTestId('assertion')).toHaveTextContent(index('f5'))
  userEvent.keyboard('{Escape}')
  await expect.element(page.getByTestId('assertion')).toBeEmptyDOMElement()
})

test('highlight array controls rendering of highlight paths', async () => {
  const highlight = ref<number[]>([])

  setup(() => {
    return () => <Hexboard
      highlight={highlight.value}
      options={{
        highlightColor: 'pink',
      }}
    />
  })

  // Initially, no highlight paths should be in the document
  await expect.element(page.getByTestId('highlight-f6')).not.toBeInTheDocument()
  await expect.element(page.getByTestId('highlight-a1')).not.toBeInTheDocument()

  // Setting highlight to a single position should render one path
  highlight.value = [index('f6')]
  await nextTick()
  await expect.element(page.getByTestId('highlight-f6')).toBeVisible()
  await expect.element(page.getByTestId('highlight-f6')).toHaveStyle({ fill: 'pink' })
  await expect.element(page.getByTestId('highlight-a1')).not.toBeInTheDocument()

  // Setting highlight to multiple positions should render multiple paths
  highlight.value = [index('f6'), index('a1')]
  await nextTick()
  await expect.element(page.getByTestId('highlight-f6')).toBeVisible()
  await expect.element(page.getByTestId('highlight-f6')).toHaveStyle({ fill: 'pink' })
  await expect.element(page.getByTestId('highlight-a1')).toBeVisible()
  await expect.element(page.getByTestId('highlight-a1')).toHaveStyle({ fill: 'pink' })

  // Clearing highlight should remove all paths
  highlight.value = []
  await nextTick()
  await expect.element(page.getByTestId('highlight-f6')).not.toBeInTheDocument()
  await expect.element(page.getByTestId('highlight-a1')).not.toBeInTheDocument()
})

test('cursor shows grab for playable pieces', async () => {
  setup(() => {
    return () => <Hexboard
      active
      playing
    />
  })

  const svg = page.getByTestId('position-f5').element().closest('svg') as SVGElement

  // Hover over a white piece (initial position has white to move)
  // f5 should have a white piece in initial position
  await page.getByTestId('position-f5').hover()
  await nextTick()
  await expect(svg).toHaveStyle({ cursor: 'grab' }) // White's turn, so grab
})

test('cursor shows grab only when user can drag piece', async () => {
  setup(() => {
    return () => <Hexboard
      active
      playing={'w'}
    />
  })

  // Hover over a white piece when it's white's turn and user is playing white
  // Should show "grab" cursor
  await page.getByTestId('position-f5').hover()
  const svg = page.getByTestId('position-f5').element().closest('svg') as SVGElement
  await expect(svg).toHaveStyle({ cursor: 'grab' })

  // Hover over an empty position when user is only playing white
  // Should show "auto" cursor (no piece to interact with)
  await page.getByTestId('position-a6').hover()
  await expect(svg).toHaveStyle({ cursor: 'auto' })

  // Hover over a black piece when user is only playing white
  // Should show "pointer" cursor (can't drag black piece as white)
  await page.getByTestId('position-f7').hover()
  await expect(svg).toHaveStyle({ cursor: 'pointer' })
})

test('cursor behavior when playing both colors', async () => {
  setup(() => {
    return () => <Hexboard
      active
      playing
    />
  })

  const svg = page.getByTestId('position-f5').element().closest('svg') as SVGElement

  // Initial position is white's turn, so white pieces show "grab"
  await page.getByTestId('position-f5').hover()
  await expect(svg).toHaveStyle({ cursor: 'grab' })

  // Black pieces show "pointer" because it's not black's turn
  await page.getByTestId('position-b7').hover()
  await expect(svg).toHaveStyle({ cursor: 'grab' })

  // Empty positions show "auto" because there's no piece
  await page.getByTestId('position-a6').hover()
  await expect(svg).toHaveStyle({ cursor: 'auto' })
})

test('cursor shows pointer when user is not playing', async () => {
  setup(() => {
    return () => <Hexboard
      active
      playing={false}
    />
  })

  // When playing is false, should show pointer for any piece
  await page.getByTestId('position-f5').hover()
  const svg = page.getByTestId('position-f5').element().closest('svg') as SVGElement
  await expect(svg).toHaveStyle({ cursor: 'pointer' })

  await page.getByTestId('position-f7').hover()
  await expect(svg).toHaveStyle({ cursor: 'pointer' })
})

test('pieces of any color can be selected, but only playing color is draggable', async () => {
  const selected = ref<number | null>(null)
  const targets = ref<number[]>([])

  setup(() => {
    return () => <>
      <Hexboard
        active
        autoselect
        playing={'w'}
        v-model:selected={selected.value}
        v-model:targets={targets.value}
      />
      <div
        v-text={selected.value}
        data-testid="assertion" />
    </>
  })

  const svg = page.getByTestId('position-f5').element().closest('svg') as SVGElement

  // White piece shows grab cursor (user can drag)
  const whitePiecePosition = page.getByTestId('position-f5')
  await whitePiecePosition.hover()
  await expect(svg).toHaveStyle({ cursor: 'grab' })

  // Black piece shows pointer cursor (user cannot drag)
  const blackPiecePosition = page.getByTestId('position-b7')
  await blackPiecePosition.hover()
  await expect(svg).toHaveStyle({ cursor: 'pointer' })

  // Both pieces can be selected
  await whitePiecePosition.click()
  await expect.element(page.getByTestId('selected-f5')).toBeVisible()
  await expect.element(page.getByTestId('assertion')).toHaveTextContent(index('f5'))

  await blackPiecePosition.click()
  await expect.element(page.getByTestId('selected-b7')).toBeVisible()
  await expect.element(page.getByTestId('assertion')).toHaveTextContent(index('b7'))
})

test('dragging piece off board results in selection only, dragging state resets', async () => {
  const selected = ref<number | null>(null)
  const targets = ref<number[]>([])

  setup(() => {
    return () => <>
      <Hexboard
        active
        autoselect
        playing={'w'}
        v-model:selected={selected.value}
        v-model:targets={targets.value}
      />
      <div
        v-text={selected.value}
        data-testid="assertion" />
    </>
  })

  const whitePiecePosition = page.getByTestId('position-f5')

  // Start dragging the piece (mousedown)
  await whitePiecePosition.element().dispatchEvent(new MouseEvent('mousedown', { bubbles: true }))
  await nextTick()

  // Verify dragging started - draggable piece SVG should be visible
  await expect.element(page.getByTestId('drag-piece')).toBeVisible()

  // Move mouse off the board (simulate mousemove on window)
  window.dispatchEvent(new MouseEvent('mousemove', { clientX: 0, clientY: 0, bubbles: true }))
  await nextTick()

  // Release mouse (mouseup on window) - this should reset dragging state
  window.dispatchEvent(new MouseEvent('mouseup', { bubbles: true }))
  await nextTick()

  // Verify piece is still selected (autoselect should have set it on mousedown)
  await expect.element(page.getByTestId('selected-f5')).toBeVisible()
  await expect.element(page.getByTestId('assertion')).toHaveTextContent(index('f5'))

  // Verify dragging state is reset - draggable piece SVG should no longer exist
  await expect.element(page.getByTestId('drag-piece')).not.toBeInTheDocument()
})

test('drag and drop piece emits move event', async () => {
  const selected = ref<number | null>(null)
  const targets = ref<number[]>([])
  const onMove = vi.fn()

  setup(() => {
    return () => <>
      <Hexboard
        active
        autoselect
        playing={'w'}
        v-model:selected={selected.value}
        v-model:targets={targets.value}
        onMove={onMove}
      />
    </>
  })

  const fromPosition = page.getByTestId('position-f5')
  const toPosition = page.getByTestId('position-f6')

  // Start dragging the piece (mousedown)
  await fromPosition.element().dispatchEvent(new MouseEvent('mousedown', { bubbles: true }))
  await nextTick()

  // Verify dragging started
  await expect.element(page.getByTestId('drag-piece')).toBeVisible()

  // Move mouse to target position and release (mouseup)
  await toPosition.element().dispatchEvent(new MouseEvent('mouseup', { bubbles: true }))
  await nextTick()

  // Verify move event was emitted with correct San object
  await expect(onMove).toHaveBeenCalledOnce()
  await expect(onMove).toHaveBeenCalledWith(expect.any(San))
  const san = onMove.mock.calls[0][0] as San
  await expect(san.from).toBe(index('f5'))
  await expect(san.to).toBe(index('f6'))
})

test('click to move piece emits move event', async () => {
  const selected = ref<number | null>(null)
  const targets = ref<number[]>([])
  const onMove = vi.fn()

  setup(() => {
    return () => <>
      <Hexboard
        active
        autoselect
        playing={'w'}
        v-model:selected={selected.value}
        v-model:targets={targets.value}
        onMove={onMove}
      />
    </>
  })

  // Click on a piece to select it (f5 should have a white piece)
  await page.getByTestId('position-f5').click()
  await nextTick()

  // Verify piece is selected and targets are shown
  await expect.element(page.getByTestId('selected-f5')).toBeVisible()
  await expect.element(page.getByTestId('target-f6')).toBeVisible()

  // Click on a target position to move
  await page.getByTestId('position-f6').click()
  await nextTick()

  // Verify move event was emitted with correct San object
  await expect(onMove).toHaveBeenCalledOnce()
  await expect(onMove).toHaveBeenCalledWith(expect.any(San))
  const san = onMove.mock.calls[0][0] as San
  await expect(san.from).toBe(index('f5'))
  await expect(san.to).toBe(index('f6'))
})

test('cannot move piece of other turn color', async () => {
  const selected = ref<number | null>(null)
  const targets = ref<number[]>([])
  const onMove = vi.fn()

  setup(() => {
    return () => <>
      <Hexboard
        active
        autoselect
        playing={true}
        v-model:selected={selected.value}
        v-model:targets={targets.value}
        onMove={onMove}
      />
    </>
  })

  // Initial position is white's turn, so try to move a black piece
  // Click on a black piece (b7 should have a black rook in initial position)
  await page.getByTestId('position-b7').click()
  await nextTick()

  // Verify piece is selected and targets are shown
  await expect.element(page.getByTestId('selected-b7')).toBeVisible()
  await expect.element(page.getByTestId('target-b6')).toBeVisible()

  // Try to click on a target position to move
  await page.getByTestId('position-b6').click()
  await nextTick()

  // Verify move event was NOT called (can't move black piece on white's turn)
  await expect(onMove).not.toHaveBeenCalled()
})

test('promotion', async () => {
  setup(() => {
    const hexchess = ref(Hexchess.parse('1/1P1/5/7/9/11/11/11/11/11/11 w - 0 1'))
    
    return () => <>
      <Hexboard
        active
        autoselect
        playing={'w'}
        hexchess={hexchess.value}
        onMove={san => hexchess.value.applyMoveUnsafe(san)}>{{
        promotion: ({ promote }: any) => <button
          data-testid="promote"
          onClick={() => promote('q')}>
          q
        </button>
      }}</Hexboard>
    </>
  })

  await movePiece(page, 'f10', 'f11')
  await page.getByTestId('promote').click()
  await expect(page.getByTestId('piece-f11')).toHaveAttribute('data-piece-type', 'Q')
})