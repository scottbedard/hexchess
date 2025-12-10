<template>
  <div>
    <svg
      ref="svgEl"
      xmlns="http://www.w3.org/2000/svg"
      :style="{ cursor }"
      :viewBox="`0 0 ${box} ${box}`">
      <!-- backdrop -->
      <path
        :d="d(perimeter)"
        :fill="normalizedOptions.colors[1]"
        :style="{ pointerEvents: 'none' }"
      />

      <!-- positions -->
      <path
        v-for="position, index in board"
        v-bind="active && !staging.hexchess ? {
          onClick: evt => onClickPosition(index, evt),
          onPointerdown: evt => onPointerdownPosition(index, evt),
          onPointerenter: () => onPointerenter(index),
          onPointerleave: () => onPointerleave(),
          onPointerup: evt => onMouseupPosition(index, evt),
        } : {}"
        :d="d(flipped ? position[4] : position[3])"
        :data-hexboard-position="index"
        :data-testid="`position-${indexToPosition(index)}`"
        :fill="normalizedOptions.colors[board[index][0]]"
        :key="index"
      />

      <!-- highlighted positions -->
      <path
        v-for="highlightIndex in highlight"
        :key="`highlight-${highlightIndex}`"
        :d="d(flipped ? board[highlightIndex][4] : board[highlightIndex][3])"
        :data-testid="`highlight-${indexToPosition(highlightIndex)}`"
        :fill="normalizedOptions.highlightColor"
        :style="{ pointerEvents: 'none' }"
      />

      <!-- selected position -->
      <path
        v-if="typeof currentSelected === 'number'"
        ref="selectedEl"
        :d="d(flipped ? board[currentSelected][4] : board[currentSelected][3])"
        :data-testid="`selected-${indexToPosition(currentSelected)}`"
        :fill="normalizedOptions.selectedColor"
        :style="{ pointerEvents: 'none' }"
      />

      <!-- labels -->
      <text
        v-if="normalizedOptions.labels"
        v-for="[text, p, positionFlipped], i in labels"
        v-text="text"
        dominant-baseline="central"
        text-anchor="middle"
        :data-testid="`label-${text}`"
        :key="`label-${i}`"
        :style="{
          fill: getLabelFill(text),
          fontSize: '.5px',
          pointerEvents: 'none',
          userSelect: 'none',
        }"
        :x="x(flipped ? positionFlipped[0] : p[0])"
        :y="y(flipped ? positionFlipped[1] : p[1])"
      />

      <!-- pieces -->
      <template
        v-if="currentHexchess"
        v-for="piece, index in currentHexchess.board">
        <Component
          v-if="piece && index !== mousedownPosition"
          :data-piece-type="piece"
          :data-testid="`piece-${indexToPosition(index)}`"
          :height="pieceSize"
          :is="pieces"
          :style="{ pointerEvents: 'none' }"
          :type="piece"
          :width="pieceSize"
          :x="x(board[index][flipped ? 2 : 1][0] - (pieceSize / 2))"
          :y="y(board[index][flipped ? 2 : 1][1] + (pieceSize / 2))"
        />
      </template>

      <!-- targets -->
      <circle
        v-for="targetIndex in currentTargets"
        :cx="x(board[targetIndex][flipped ? 2 : 1][0])"
        :cy="y(board[targetIndex][flipped ? 2 : 1][1])"
        :data-testid="`target-${indexToPosition(targetIndex)}`"
        :fill="normalizedOptions.targetColor"
        :key="`target-${indexToPosition(targetIndex)}`"
        :r="0.3"
        :style="{ pointerEvents: 'none' }"
      />
    </svg>

    <!-- draggable piece -->
    <svg
      v-if="dragPiece"
      data-testid="drag-piece"
      xmlns="http://www.w3.org/2000/svg"
      :style="{
        height: svgRect.height + 'px',
        left: '0px',
        pointerEvents: 'none',
        position: 'fixed',
        top: '0px',
        transform: `translate(${dragCoords.x}px, ${dragCoords.y}px) scale(1.1)`,
        width: svgRect.width + 'px',
        willChange: 'transform',
      }"
      :viewBox="`0 0 ${box} ${box}`">
      <Component
        :height="pieceSize"
        :is="pieces"
        :style="{ pointerEvents: 'none' }"
        :type="dragPiece"
        :width="pieceSize"
        :x="x(pieceSize / -2)"
        :y="y(pieceSize / 2)"
      />
    </svg>

    <div 
      v-if="typeof staging.selected === 'number'"
      :style="{
        height: promotionRect.height + 'px',
        left: promotionRect.left + 'px',
        position: 'fixed',
        top: promotionRect.top + 'px',
        width: promotionRect.width + 'px',
      }"
      @pointerup.stop>
      <slot
        name="promotion"
        :b="promotionPieces.b"
        :cancel="cancelPromotion"
        :file="indexToPosition(staging.selected)[0]"
        :n="promotionPieces.n"
        :promote
        :q="promotionPieces.q"
        :r="promotionPieces.r"
        :rank="Number(indexToPosition(staging.selected).slice(1))" />
    </div>

    <pre class="fixed left-0 bottom-0 z-10 text-sm leading-loose text-gray-300 p-6">{{ {
      selected,
      targets,
      playing,
    } }}</pre>
  </div>
</template>

<script lang="ts" setup>
import { board, box, defaultOptions, initialPosition, labels, pieceSize, perimeter } from './constants'
import { computed, h, onMounted, onUnmounted, shallowRef, useTemplateRef, watch, type Component } from 'vue'
import { d } from './dom'
import { isPromotionPosition, Hexchess, position as indexToPosition, San, type Color } from '@bedard/hexchess'
import { x, y } from './geometry'
import GiocoPieces from '../pieces/GiocoPieces.vue'
import type { HexboardOptions } from './types'

//
// props
//

const props = withDefaults(
  defineProps<{
    active?: boolean
    autoselect?: boolean
    flipped?: boolean
    hexchess?: Hexchess
    highlight?: number[]
    ignoreTurn?: boolean
    options?: Partial<HexboardOptions>
    pieces?: Component
    playing?: Color | boolean
    position?: string
  }>(),
  {
    active: false,
    autoselect: false,
    flipped: false,
    hexchess: () => Hexchess.init(),
    highlight: () => [],
    ignoreTurn: false,
    options: () => ({}),
    pieces: () => GiocoPieces,
    playing: false,
    position: initialPosition,
  }
)

//
// events
//

const emit = defineEmits<{
  clickPosition: [position: number]
  move: [san: San]
}>()

//
// models
//

const mouseoverPosition = defineModel<number | null>('mouseover-position', {
  default: null,
  required: false,
})

const selected = defineModel<number | null>('selected', {
  default: null,
  required: false,
})

const targets = defineModel<number[]>('targets', {
  default: () => [],
  required: false,
})

//
// state
//

/** current mouse coordinates */
const mouseCoords = shallowRef({ x: 0, y: 0 })

/** fen position of mousedown */
const mousedownPosition = shallowRef<number | null>(null)

/** rect of promotion anchor element */
const promotionRect = shallowRef<DOMRect>(new DOMRect())

/** staging display data */
const staging = shallowRef<{
  hexchess: Hexchess | null
  promotionEl: Element | null
  promotionFrom: number | null
  promotionTo: number | null
  selected: number | null
}>({
  hexchess: null,
  promotionEl: null,
  promotionFrom: null,
  promotionTo: null,
  selected: null,
})

/** svg rect */
const svgEl = useTemplateRef('svgEl')

/** rect of svg element on mousedown */
const svgRect = shallowRef<DOMRect>(new DOMRect())

//
// computed
//

/** current targets */
const currentTargets = computed(() => {
  if (staging.value.hexchess) {
    return []
  }

  return targets.value
})

/** current hexchess state */
const currentHexchess = computed(() => {
  if (staging.value.hexchess) {
    return staging.value.hexchess
  }

  if (props.hexchess) {
    return props.hexchess
  }

  return Hexchess.init()
})

/** current selected position */
const currentSelected = computed(() => {
  if (typeof staging.value.selected === 'number') {
    return null
  }

  return selected.value
})

/** cursor type */
const cursor = computed(() => {
  if (dragPiece.value) {
    return 'grabbing' // global cursor
  }

  if (!props.active || mouseoverPosition.value === null || staging.value.hexchess) {
    return undefined
  }

  // If piece is selected and hovering over a target, show pointer
  if (
    selected.value !== null &&
    targets.value.includes(mouseoverPosition.value)
  ) {
    const selectedPiece = currentHexchess.value?.board[selected.value]
    if (selectedPiece) {
      const selectedPieceColor: Color = selectedPiece === selectedPiece.toLowerCase() ? 'b' : 'w'
      const isSelectedTurn = currentHexchess.value?.turn === selectedPieceColor
      
      // Allow moving if playing both colors, or if it's the selected piece's turn
      if ((props.playing === true || isSelectedTurn) && isPlayingPosition(selected.value)) {
        return 'pointer'
      }
    }
  }

  if (!mouseoverPiece.value) {
    return undefined
  }

  // When playing both colors, any piece is draggable
  if (props.playing === true) {
    return 'grab'
  }

  // When playing a single color, check if piece is draggable (must be their turn)
  if (
    props.playing &&
    mouseoverColor.value === currentHexchess.value?.turn &&
    props.playing === mouseoverColor.value
  ) {
    return 'grab'
  }

  return 'pointer'
})

/** coordinates of drag transformation */
const dragCoords = computed(() => {
  return {
    x: mouseCoords.value.x - (svgRect.value.width / 2),
    y: mouseCoords.value.y - (svgRect.value.height / 2),
  }
})

/** piece being dragged */
const dragPiece = computed(() => {
  if (
    !props.hexchess ||
    staging.value.hexchess ||
    mousedownPosition.value === null
  ) {
    return null
  }

  return props.hexchess.board[mousedownPosition.value]
})

/** normalized options */
const normalizedOptions = computed(() => {
  return { ...defaultOptions, ...props.options }
})

/** color of piece at mouseover position */
const mouseoverColor = computed<Color | null>(() => {
  if (!mouseoverPiece.value) {
    return null
  }

  return mouseoverPiece.value === mouseoverPiece.value.toLowerCase() ? 'b' : 'w'
})

/** piece at mouseover position */
const mouseoverPiece = computed(() => {
  if (mouseoverPosition.value === null) {
    return null
  }

  return currentHexchess.value?.board[mouseoverPosition.value] ?? null
})

/** promotion piece components for the promoting color */
const promotionPieces = computed(() => {
  const piece = staging.value.hexchess?.board[staging.value.selected ?? -1]
  const isWhite = piece === piece?.toUpperCase()
  
  const createPiece = (type: string) => {
    return (attrs: Record<string, unknown>) => h(props.pieces, { ...attrs, type })
  }
  
  return {
    n: createPiece(isWhite ? 'N' : 'n'),
    b: createPiece(isWhite ? 'B' : 'b'),
    r: createPiece(isWhite ? 'R' : 'r'),
    q: createPiece(isWhite ? 'Q' : 'q'),
  }
})

//
// lifecycle
//

onMounted(() => {
  if (props.active) {
    listen()
  }
})

onUnmounted(unlisten)

//
// watchers
//

watch(cursor, val => {
  document.body.style.setProperty('cursor', val === 'grabbing' ? 'grabbing' : null)
})

watch(() => props.active, val => val ? listen() : unlisten())

//
// methods
//

/** attempt to move piece from source to target position */
function attemptMove(
  san: San,
  evt?: MouseEvent
) {
  // Check if target is valid
  if (!targets.value.includes(san.to)) {
    return
  }

  const piece = props.hexchess?.board[san.from]
  
  if (!piece) {
    return
  }

  const pieceColor = piece === piece.toLowerCase() ? 'b' : 'w'

  const isCurrentTurn = props.hexchess?.turn === pieceColor
  
  // Check if this is a pawn promotion move
  if (
    props.hexchess &&
    (piece === 'p' || piece === 'P') &&
    isPromotionPosition(san.to, pieceColor)
  ) {
    const clone = props.hexchess.clone()
    clone.board[san.from] = null
    clone.board[san.to] = piece
    staging.value = {
      hexchess: clone,
      promotionEl: evt?.target instanceof Element ? evt.target : null,
      promotionFrom: san.from,
      promotionTo: san.to,
      selected: san.to,
    }

    if (evt?.target instanceof Element) {
      promotionRect.value = evt.target.getBoundingClientRect()
    }

    return
  }
  
  // Only call onPieceMove if playing this color and it's their turn (or ignoreTurn is true)
  if (isPlayingPosition(san.from) && (props.ignoreTurn || isCurrentTurn)) {
    onPieceMove(san)
  }
}

/** check if user is playing the color at a position */
function isPlayingPosition(index: number): boolean {
  const piece = props.hexchess?.board[index]

  if (!piece) {
    return false
  }
  
  const pieceColor: Color = piece === piece.toLowerCase() ? 'b' : 'w'

  return props.playing === true || props.playing === pieceColor
}

/** get fill color of label */
function getLabelFill(text: string) {
  if (mouseoverPosition.value === null) {
    return normalizedOptions.value.labelColor
  }

  if (
    indexToPosition(mouseoverPosition.value)?.startsWith(text) ||
    indexToPosition(mouseoverPosition.value)?.endsWith(text)
  ) {
    return normalizedOptions.value.labelActiveColor
  }

  return normalizedOptions.value.labelInactiveColor
}

/** listen for events */
function listen() {
  mouseCoords.value = { x: 0, y: 0 }

  window.addEventListener('keyup', onKeyupWindow)
  window.addEventListener('pointermove', onPointermoveWindow)
  window.addEventListener('pointerup', onPointerupWindow)
  window.addEventListener('resize', measurePromotionRect)
  window.addEventListener('scroll', measurePromotionRect)
}

/** measure promotion element rect */
function measurePromotionRect() {
  promotionRect.value = staging.value.promotionEl?.getBoundingClientRect() ?? new DOMRect()
}

/** click position */
function onClickPosition(index: number, evt: MouseEvent) {
  if (!props.active) {
    return
  }

  // If there's a selected piece and clicking a target, attempt to move
  if (selected.value !== null && targets.value.includes(index)) {
    const san = new San({ from: selected.value, to: index })
    attemptMove(san, evt)
    return
  }

  // If autoselect is enabled and clicking an unoccupied position, deselect
  if (props.autoselect && !props.hexchess.board[index]) {
    selected.value = null
    targets.value = []
  }

  emit('clickPosition', index)
}

/** keyup window */
function onKeyupWindow(evt: KeyboardEvent) {
  if (evt.key === 'Escape') {
    // If staging a promotion, cancel it
    if (staging.value.hexchess) {
      cancelPromotion()
      return
    }
    
    // Otherwise deselect if autoselect is enabled
    if (props.autoselect) {
      selected.value = null
      targets.value = []
    }
  }
}

/** handle piece move */
function onPieceMove(san: San) {
  emit('move', san)

  resetState()
}

/** mouseup position */
function onMouseupPosition(index: number, evt: MouseEvent) {
  evt.stopPropagation()

  // Check if we're dropping a piece on a valid target (drag and drop)
  if (mousedownPosition.value !== null) {
    const san = new San({ from: mousedownPosition.value, to: index })
    attemptMove(san, evt)
  }
  // Check if clicking on a target while a piece is selected (click to move)
  else if (selected.value !== null && targets.value.includes(index)) {
    const san = new San({ from: selected.value, to: index })
    attemptMove(san, evt)
  }

  /** do nothing if staging is set */
  if (staging.value.hexchess) {
    return
  }

  // If clicking on any piece, keep the selection (it was set in pointerdown)
  if (props.hexchess?.board[index]) {
    mousedownPosition.value = null
    svgRect.value = new DOMRect()
    return
  }

  resetState()
}

/** cancel promotion and restore original selection */
function cancelPromotion() {
  const from = staging.value.promotionFrom
  
  staging.value = {
    hexchess: null,
    promotionEl: null,
    promotionFrom: null,
    promotionTo: null,
    selected: null,
  }
  
  // Keep the original piece selected
  if (typeof from === 'number') {
    selected.value = from
    targets.value = props.hexchess.movesFrom(from).map(san => san.to) ?? []
  }
  
  mousedownPosition.value = null
}

/** pointerdown on position */
function onPointerdownPosition(index: number, evt: PointerEvent) {
  evt.preventDefault()

  const piece = props.hexchess?.board[index]
  
  if (!piece) {
    return
  }

  if (props.autoselect) {
    selected.value = index
    targets.value = props.hexchess?.movesFrom(index).map(san => san.to) ?? []
  }

  if (!isPlayingPosition(index)) {
    return
  }

  // Only allow dragging if it's the piece's turn (or ignoreTurn is true)
  const pieceColor: Color = piece === piece.toLowerCase() ? 'b' : 'w'
  const isCurrentTurn = props.hexchess?.turn === pieceColor
  
  if (!props.ignoreTurn && !isCurrentTurn) {
    return
  }

  mousedownPosition.value = index

  if (svgEl.value instanceof Element) {
    svgRect.value = svgEl.value.getBoundingClientRect()
  }
}

/** mouseenter position */
function onPointerenter(index: number) {
  mouseoverPosition.value = index
}

/** mouseleave position */
function onPointerleave() {
  mouseoverPosition.value = null
}

/** mousemove window */
function onPointermoveWindow(evt: MouseEvent) {
  if (!props.active) {
    return
  }

  mouseCoords.value = { x: evt.clientX, y: evt.clientY }
}

/** mouseup window */
function onPointerupWindow() {
  // If staging a promotion, cancel it but keep the original piece selected
  if (staging.value.hexchess) {
    cancelPromotion()
    return
  }

  // If dragging a piece, keep the selection but reset drag state
  if (mousedownPosition.value !== null) {
    mousedownPosition.value = null
    svgRect.value = new DOMRect()
    return
  }

  resetState()
}

/** promote piece */
function promote(promotion: 'n' | 'b' | 'r' | 'q') {
  if (
    typeof staging.value.promotionFrom === 'number' &&
    isPlayingPosition(staging.value.promotionFrom)
  ) {
    const san = new San({
      from: staging.value.promotionFrom ?? 0,
      to: staging.value.promotionTo ?? 0,
      promotion: promotion,
    })
    
    onPieceMove(san)
  }
}

/** reset state */
function resetState() {
  document.body.style.setProperty('cursor', null)
  mousedownPosition.value = null
  selected.value = null
  staging.value = {
    hexchess: null,
    promotionEl: null,
    promotionFrom: null,
    promotionTo: null,
    selected: null,
  }
  svgRect.value = new DOMRect()
  targets.value = []
  console.log('resetState')
}

/** stop listening for events */
function unlisten() {
  resetState()
  window.removeEventListener('keyup', onKeyupWindow)
  window.removeEventListener('pointermove', onPointermoveWindow)
  window.removeEventListener('pointerup', onPointerupWindow)
  window.removeEventListener('resize', measurePromotionRect)
  window.removeEventListener('scroll', measurePromotionRect)
}
</script>
