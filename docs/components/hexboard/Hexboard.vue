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

      <!-- positions & highlights -->
      <template
        v-for="hex, i in board"
        :key="i">
        <path
          :d="d(flipped ? hex.path[1] : hex.path[0])"
          :data-hexboard-position="i"
          :fill="selected === i ? 'oklch(63.7% 0.237 25.331)' : fill(hex)"
          @click.stop="$emit('positionClick', i)"
          @mouseenter="mouseover = i"
          @mouseleave="mouseover = null"
        />

        <path
          v-if="highlighted.includes(i)"
          class="opacity-85 pointer-events-none"
          fill="oklch(90.5% 0.182 98.111)"
          :d="d(flipped ? hex.path[1] : hex.path[0])"
          :data-hexboard-position="i"
        />
      </template>

      <!-- labels -->
      <text
        v-for="[text, p, positionFlipped], i in labels"
        v-text="text"
        dominant-baseline="central"
        text-anchor="middle"
        :class="[
          'font-bold pointer-events-none select-none text-[.5px]',
          mouseover !== null && (position(mouseover).startsWith(text) || position(mouseover).endsWith(text))
            ? 'fill-(--vp-code-color)'
            : 'fill-gray-400'
        ]"
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
import { Hexchess, San, position } from '../../../js/src'
import Piece from './Piece.vue'

const props = defineProps<{
  flipped: boolean
  hexchess: Hexchess
  highlighted: number[]
  selected: number | null
  targets: San[]
}>()

defineEmits<{
  positionClick: [position: number]
}>()

//
// state
//

const mouseover = ref<number | null>(null)

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