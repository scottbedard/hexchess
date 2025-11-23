<template>
  <div>
    <svg
      xmlns="http://www.w3.org/2000/svg"
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
          onMouseup: () => onMouseupPosition(index),
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
          v-if="piece"
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

    <pre>{{ { normalizedOptions } }}</pre>
  </div>
</template>

<script lang="ts" setup>
import { board, box, defaultOptions, initialPosition, labels, pieceSize, perimeter } from './constants'
import { computed, onMounted, onUnmounted, shallowRef, watch, type Component } from 'vue'
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
    flipped?: boolean
    highlight?: number[]
    options?: Partial<HexboardOptions>
    pieces?: Component
    playing?: Color | boolean
    position?: string
    targets?: number[]
  }>(),
  {
    active: false,
    flipped: false,
    highlight: () => [],
    options: () => ({}),
    pieces: () => GiocoPieces,
    playing: false,
    position: initialPosition,
    targets: () => [],
  }
)

//
// models
//

const mouseoverPosition = defineModel<number | null>('mouseover-position', { default: null, required: false })

const selected = defineModel<number | null>('selected', { default: null, required: false })

//
// state
//

const mousePosition = shallowRef<{ x: number; y: number } | null>(null)

//
// events
//

const emit = defineEmits<{
  clickPosition: [position: number]
}>()

//
// computed
//

/** current hexchess state */
const hexchess = computed(() => Hexchess.parse(props.position))

/** normalized options */
const normalizedOptions = computed(() => {
  return { ...defaultOptions, ...props.options }
})

//
// lifecycle
//

onMounted(() => {
  if (props.active) {
    trackMousemove()
  }
})

onUnmounted(() => {
  stopTrackingMousemove()
})

//
// watchers
//

watch(() => props.active, (active) => {
  if (active) trackMousemove()
  else stopTrackingMousemove()
})

//
// methods
//

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

/** click position */
function onClickPosition(index: number) {
  selected.value = index

  emit('clickPosition', index)
}

/** mousedown on position */
function onMousedownPosition(index: number) {
  console.log('onMousedownPosition', index)
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
function onMouseupPosition(index: number) {
  // ...
}

/** mousemove window */
function onMousemoveWindow(evt: MouseEvent) {
  mousePosition.value = { x: evt.clientX, y: evt.clientY }
}

/** track mousemove */
function trackMousemove() {
  window.addEventListener('mousemove', onMousemoveWindow)
  mousePosition.value = { x: 0, y: 0 }
}

/** stop tracking mousemove */
function stopTrackingMousemove() {
  window.removeEventListener('mousemove', onMousemoveWindow)
  mousePosition.value = { x: -1, y: -1 }
}
</script>