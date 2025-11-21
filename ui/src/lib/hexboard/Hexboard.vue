<template>
  <div>
    <svg
      xmlns="http://www.w3.org/2000/svg"
      :viewBox="`0 0 ${box} ${box}`">
      <!-- backdrop -->
      <path
        :d="d(perimeter)"
        :fill="colors[1]"
        :style="{
          pointerEvents: 'none',
        }"
      />

      <!-- positions -->
      <g>
        <path
          v-for="position, index in board"
          :key="index"
          :d="d(flipped ? position[4] : position[3])"
          :fill="fill(index)"
        />
      </g>

      <!-- pieces -->
      <g 
        v-if="hexchess"
        v-for="piece, index in hexchess.board">
        <Component
          v-if="piece"
          :is="pieces"
          :height="pieceSize"
          :type="piece"
          :width="pieceSize"
          :x="x(board[index][flipped ? 2 : 1][0] - (pieceSize / 2))"
          :y="y(board[index][flipped ? 2 : 1][1] + (pieceSize / 2))"
        />
      </g>
    </svg>
  </div>
</template>

<script lang="ts" setup>
import { board, box, colors, initialPosition, pieceSize, perimeter } from './constants'
import { computed, type Component } from 'vue'
import { d } from './dom'
import { Hexchess } from '@bedard/hexchess'
import { x, y } from './geometry'

const props = withDefaults(
  defineProps<{
    flipped?: boolean
    pieces: Component
    position?: string
  }>(),
  {
    flipped: false,
    position: initialPosition
  }
)

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
</script>