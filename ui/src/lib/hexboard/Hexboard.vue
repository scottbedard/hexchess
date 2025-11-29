<template>
  <div class="border-4 border-[red]">
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
          onMouseup: () => onMouseupPosition(),
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
        v-if="selected !== null"
        :d="d(flipped ? board[selected][4] : board[selected][3])"
        :data-testid="`selected-${indexToPosition(selected)}`"
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
        v-if="hexchess"
        v-for="piece, index in hexchess.board">
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
        v-for="targetIndex in targets"
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
  </div>
</template>

<script lang="ts" setup>
import { board, box, defaultOptions, initialPosition, labels, pieceSize, perimeter } from './constants'
import { computed, onMounted, onUnmounted, shallowRef, useTemplateRef, watch, type Component } from 'vue'
import { d } from './dom'
import { Hexchess, position as indexToPosition, type Color } from '@bedard/hexchess'
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

/** svg rect */
const svgEl = useTemplateRef('svgEl')

/** rect of svg element on mousedown */
const svgRect = shallowRef<DOMRect>(new DOMRect())

//
// computed
//

const cursor = computed(() => {
  if (dragPiece.value) {
    return 'grabbing' // global cursor
  }

  if (
    !props.active ||
    !mouseoverPiece.value ||
    mouseoverPosition.value === null
  ) {
    return undefined
  }

  // When playing is true or a color, check if piece is draggable
  if (
    props.playing &&
    hexchess.value.turn === mouseoverColor.value &&
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
  if (mousedownPosition.value === null) {
    return null
  }

  return hexchess.value.board[mousedownPosition.value]
})

/** current hexchess state */
const hexchess = computed(() => Hexchess.parse(props.position))

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

  return hexchess.value.board[mouseoverPosition.value]
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

  // If autoselect is enabled and clicking an unoccupied position, deselect
  if (props.autoselect && !hexchess.value.board[index]) {
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
  const piece = hexchess.value.board[index]
  
  if (!piece) {
    return
  }

  const pieceColor: Color = piece === piece.toLowerCase() ? 'b' : 'w'
  const isPlayingColor = props.playing === true || props.playing === pieceColor

  if (props.autoselect) {
    selected.value = index
    targets.value = hexchess.value.movesFrom(index).map(san => san.to)
  }

  if (!isPlayingColor) {
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

/** mouseup position */
function onMouseupPosition() {
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
  document.body.style.setProperty('cursor', null)
  mousedownPosition.value = null
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