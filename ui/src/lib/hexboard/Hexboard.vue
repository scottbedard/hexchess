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
        :d="d(flipped ? position[4] : position[3])"
        :data-testid="`position-${indexToPosition(index)}`"
        :fill="fill(index)"
        :key="index"
        @click="onClickPosition(index)"
        @mouseenter="onMouseenterPosition(index)"
        @mouseleave="onMouseleavePosition"
      />

      <!-- labels -->
      <text
        v-for="[text, p, positionFlipped], i in labels"
        v-text="text"
        :data-testid="`label-${text}`"
        dominant-baseline="central"
        text-anchor="middle"
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
    </svg>
  </div>
</template>

<script lang="ts" setup>
import { board, box, defaultOptions, initialPosition, labels, pieceSize, perimeter } from './constants'
import { computed, type Component } from 'vue'
import { d } from './dom'
import { Hexchess, position as indexToPosition } from '@bedard/hexchess'
import { x, y } from './geometry'
import GiocoPieces from '../pieces/GiocoPieces.vue'
import type { HexboardOptions } from './types'

//
// props
//

const props = withDefaults(
  defineProps<{
    flipped?: boolean
    pieces?: Component
    position?: string
    options?: Partial<HexboardOptions>
  }>(),
  {
    flipped: false,
    options: () => ({}),
    pieces: () => GiocoPieces,
    position: initialPosition,
  }
)

//
// models
//

const mouseoverPosition = defineModel<number | null>('mouseover-position', { default: null, required: false })

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
// methods
//

/** fill color of position */
function fill(i: number) {
  return normalizedOptions.value.colors[board[i][0]]
}

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

/** handle click on position */
function onClickPosition(index: number) {
  emit('clickPosition', index)
}

/** handle mouse enter on position */
function onMouseenterPosition(index: number) {
  mouseoverPosition.value = index
}

/** handle mouse leave on position */
function onMouseleavePosition() {
  mouseoverPosition.value = null
}
</script>