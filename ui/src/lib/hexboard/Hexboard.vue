<template>
  <div>
    <svg
      xmlns="http://www.w3.org/2000/svg"
      :viewBox="`0 0 ${box} ${box}`">
      <!-- backdrop -->
      <path
        :d="d(perimeter)"
        :fill="colors[1]"
        :style="{ pointerEvents: 'none' }"
      />

      <!-- positions -->
      <path
        v-for="position, index in board"
        :d="d(flipped ? position[4] : position[3])"
        :data-testid="`position-${indexToPosition(index)}`"
        :fill="fill(index)"
        :key="index"
        @click="onClick(index)"
        @mouseenter="onMouseenter(index)"
        @mouseleave="onMouseleave"
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
import { board, box, colors, initialPosition, pieceSize, perimeter } from './constants'
import { computed, type Component } from 'vue'
import { d } from './dom'
import { Hexchess, position as indexToPosition } from '@bedard/hexchess'
import { x, y } from './geometry'
import GiocoPieces from '../pieces/GiocoPieces.vue'

//
// props
//

const props = withDefaults(
  defineProps<{
    flipped?: boolean
    pieces?: Component
    position?: string
  }>(),
  {
    flipped: false,
    pieces: () => GiocoPieces,
    position: initialPosition
  }
)

//
// models
//

const mouseover = defineModel<number | null>('mouseover', { default: null, required: false })

//
// events
//

const emit = defineEmits<{
  positionClick: [position: number]
}>()

//
// computed
//

const hexchess = computed(() => Hexchess.parse(props.position))

//
// methods
//

/** fill color of position */
function fill(i: number) {
  return colors[board[i][0]]
}

/** handle click on position */
function onClick(index: number) {
  emit('positionClick', index)
}

/** handle mouse enter on position */
function onMouseenter(index: number) {
  mouseover.value = index
}

/** handle mouse leave on position */
function onMouseleave() {
  mouseover.value = null
}
</script>