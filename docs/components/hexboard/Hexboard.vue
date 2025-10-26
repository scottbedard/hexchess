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
        :fill="colors[1]"
        :style="{ pointerEvents: 'none' }"
      />

      <!-- positions -->
      <path
        v-for="hex, i in board"
        :d="d(flipped ? hex.path[1] : hex.path[0])"
        :data-hexboard-position="i"
        :fill="fill(hex)"
        :key="`position-${i}`"
        @click="$emit('positionClick', i)"
      />

      <!-- labels -->
      <text
        v-for="[text, p, positionFlipped], i in labels"
        v-text="text"
        dominant-baseline="central"
        text-anchor="middle"
        :key="`label-${i}`"
        :style="{
          fill: 'oklch(70.4% 0.04 256.788)',
          fontSize: '.5px',
          pointerEvents: 'none',
        }"
        :x="x(flipped ? positionFlipped[0] : p[0])"
        :y="y(flipped ? positionFlipped[1] : p[1])"
      />

      <!-- pieces -->
      <Piece
        v-for="type, i in pieces"
        :key="`piece-${i}`"
        :style="{ pointerEvents: 'none' }"
        :type
        :x="x(board[i]!.origin[flipped ? 1 : 0][0] - (pieceSize / 2))"
        :y="y(board[i]!.origin[flipped ? 1 : 0][1] + (pieceSize / 2))"
      />

      <circle
        v-for="san in targets"
        fill="oklch(63.7% 0.237 25.331)"
        :cx="x(board[san.to]!.origin[flipped ? 1 : 0][0])"
        :cy="y(board[san.to]!.origin[flipped ? 1 : 0][1])"
        :data-hexboard-target="san.to"
        :key="`target-${san.to}`"
        :r=".3"
        :style="{ pointerEvents: 'none' }"
      />
    </svg>
  </div>
</template>

<script lang="ts" setup>
import { board, box, colors, labels, perimeter, pieceSize } from './constants'
import { computed, ref } from 'vue'
import { d, x, y } from './geometry'
import { Hexchess, San } from '../../../js/src/hexchess'
import Piece from './Piece.vue'

const props = defineProps<{
  flipped: boolean
  hexchess: Hexchess
  targets: San[]
}>()

defineEmits<{
  positionClick: [position: number]
}>()

//
// state
//

const flipped = ref(false)

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
  // if (selected.value === board.indexOf(hex)) {
  //   return props.theme.selectedColor
  // }

  return colors[hex.color]
}
</script>