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
        v-bind="active ? {
          onClick: () => onClickPosition(index),
          onMousedown: () => onMousedownPosition(index),
          onMouseenter: () => onMouseenterPosition(index),
          onMouseleave: () => onMouseleavePosition(),
          onMouseup: evt => onMouseupPosition(index, evt),
        } : {}"
        :d="d(flipped ? position[4] : position[3])"
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

    <pre>{{ { staging } }}</pre>
  </div>
</template>

<script lang="ts" setup>
import { board, box, defaultOptions, initialPosition, labels, pieceSize, perimeter } from './constants'
import { computed, onMounted, onUnmounted, shallowRef, useTemplateRef, watch, type Component } from 'vue'
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
    options?: Partial<HexboardOptions>
    pieces?: Component
    playing?: Color | boolean
    position?: string
  }>(),
  {
    active: false,
    autoselect: false,
    flipped: false,
    highlight: () => [],
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

/** staging display data */
const staging = shallowRef<{
  hexchess: Hexchess | null
  selected: number | null
}>({
  hexchess: null,
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
    return staging.value.selected
  }

  return selected.value
})

const cursor = computed(() => {
  if (dragPiece.value) {
    return 'grabbing' // global cursor
  }

  if (!props.active || mouseoverPosition.value === null) {
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
      
      if (isSelectedTurn && isPlayingPosition(selected.value)) {
        return 'pointer'
      }
    }
  }

  if (!mouseoverPiece.value) {
    return undefined
  }

  // When playing is true or a color, check if piece is draggable
  if (
    props.playing &&
    mouseoverColor.value === currentHexchess.value?.turn &&
    (props.playing === true || props.playing === mouseoverColor.value)
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

watch(() => props.active, active => {
  if (active) listen()
  else unlisten()
})

//
// methods
//

/** attempt to move piece from source to target position */
function attemptMove(from: number, to: number) {
  // Check if target is valid
  if (!targets.value.includes(to)) {
    return
  }

  const piece = props.hexchess?.board[from]
  
  if (!piece) {
    return
  }

  const pieceColor = piece === piece.toLowerCase() ? 'b' : 'w'

  const isCurrentTurn = props.hexchess?.turn === pieceColor
  
  // Check if this is a pawn promotion move
  if (
    props.hexchess &&
    (piece === 'p' || piece === 'P') &&
    isPromotionPosition(to, pieceColor)
  ) {
    const clone = props.hexchess.clone()
    clone.board[from] = null
    clone.board[to] = piece
    staging.value = {
      hexchess: clone,
      selected: to,
    }
    console.log('staging', staging.value)
    return
  }
  
  // Only call onPieceMove if playing this color and it's their turn
  if (isPlayingPosition(from) && isCurrentTurn) {
    onPieceMove(from, to)
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
  window.addEventListener('mousemove', onMousemoveWindow)
  window.addEventListener('mouseup', onMouseupWindow)
}

/** click position */
function onClickPosition(index: number) {
  if (!props.active) {
    return
  }

  // If there's a selected piece and clicking a target, attempt to move
  if (selected.value !== null && targets.value.includes(index)) {
    attemptMove(selected.value, index)
    return
  }

  // If autoselect is enabled and clicking an unoccupied position, deselect
  if (props.autoselect && !props.hexchess?.board[index]) {
    selected.value = null
    targets.value = []
  }

  emit('clickPosition', index)
}

/** keyup window */
function onKeyupWindow(evt: KeyboardEvent) {
  if (props.autoselect && evt.key === 'Escape') {
    selected.value = null
    targets.value = []
  }
}

/** mousedown on position */
function onMousedownPosition(index: number) {
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

  // Only allow dragging if it's the piece's turn
  const pieceColor: Color = piece === piece.toLowerCase() ? 'b' : 'w'
  const isCurrentTurn = props.hexchess?.turn === pieceColor
  
  if (!isCurrentTurn) {
    return
  }

  mousedownPosition.value = index

  if (svgEl.value instanceof Element) {
    svgRect.value = svgEl.value.getBoundingClientRect()
  }
}

/** mouseenter position */
function onMouseenterPosition(index: number) {
  mouseoverPosition.value = index
}

/** mouseleave position */
function onMouseleavePosition() {
  mouseoverPosition.value = null
}

/** handle piece move */
function onPieceMove(from: number, to: number) {
  const san = new San({ from, to })

  emit('move', san)

  if (props.hexchess) {
    props.hexchess.applyMoveUnsafe(san)
    selected.value = null
    targets.value = []
  }
}

/** mouseup position */
function onMouseupPosition(index: number, evt: MouseEvent) {
  evt.stopPropagation()

  // Check if we're dropping a piece on a valid target
  if (mousedownPosition.value !== null) {
    attemptMove(mousedownPosition.value, index)
  }

  /** do nothing if staging is set */
  if (staging.value.hexchess) {
    return
  }

  resetState()
}

/** mousemove window */
function onMousemoveWindow(evt: MouseEvent) {
  mouseCoords.value = { x: evt.clientX, y: evt.clientY }
}

/** mouseup window */
function onMouseupWindow() {
  resetState()
}

/** reset state */
function resetState() {
  console.log('resetState')
  document.body.style.setProperty('cursor', null)
  mousedownPosition.value = null
  staging.value = {
    hexchess: null,
    selected: null,
  }
  svgRect.value = new DOMRect()
}

/** stop listening for events */
function unlisten() {
  resetState()
  window.removeEventListener('keyup', onKeyupWindow)
  window.removeEventListener('mousemove', onMousemoveWindow)
  window.removeEventListener('mouseup', onMouseupWindow)
}
</script>