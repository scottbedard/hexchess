<template>
  <div>
    <svg
      ref="svgEl"
      xmlns="http://www.w3.org/2000/svg"
      :style="{ cursor }"
      :viewBox="`0 0 ${box} ${box}`">
      <!-- backdrop -->
      <path
        class="pointer-events-none"
        :d="d(perimeter)"
        :fill="colors[1]"
      />

      <!-- positions -->
      <path
        v-for="hex, i in board"
        :d="d(flipped ? hex.path[1] : hex.path[0])"
        :data-hexboard-position="i"
        :fill="selected === i ? 'oklch(63.7% 0.237 25.331)' : fill(hex)"
        :key="`position-${i}`"
        @click.stop="$emit('positionClick', i)"
      />

      <!-- labels -->
      <text
        v-for="[text, p, positionFlipped], i in labels"
        v-text="text"
        class="fill-gray-400 pointer-events-none text-[.5px]"
        dominant-baseline="central"
        text-anchor="middle"
        :key="`label-${i}`"
        :x="x(flipped ? positionFlipped[0] : p[0])"
        :y="y(flipped ? positionFlipped[1] : p[1])"
      />

      <!-- pieces -->
      <Piece
        v-for="type, i in pieces"
        class="pointer-events-none"
        :key="`piece-${i}`"
        :type
        :x="x(board[i]!.origin[flipped ? 1 : 0][0] - (pieceSize / 2))"
        :y="y(board[i]!.origin[flipped ? 1 : 0][1] + (pieceSize / 2))"
      />

      <circle
        v-for="san in targets"
        class="fill-red-500 pointer-events-none"
        :cx="x(board[san.to]!.origin[flipped ? 1 : 0][0])"
        :cy="y(board[san.to]!.origin[flipped ? 1 : 0][1])"
        :data-hexboard-target="san.to"
        :key="`target-${san.to}`"
        :r=".3"
      />
    </svg>
  </div>
</template>

<script lang="ts" setup>
import { board, box, colors, labels, perimeter, pieceSize } from './constants'
import { computed, ref } from 'vue'
import { d, x, y } from './geometry'
import { Hexchess, San } from '../../../js/src'
import Piece from './Piece.vue'

const props = defineProps<{
  flipped: boolean
  hexchess: Hexchess
  selected: number | null
  targets: San[]
}>()

defineEmits<{
  positionClick: [position: number]
}>()

//
// computed
//

const cursor = computed(() => {
  return 'auto'
})

const pieces = computed(() => {
  return props.hexchess.board
})

//
// helpers
//

/** fill color of position */
function fill(hex: typeof board[number]) {
  return colors[hex.color]
}
</script>