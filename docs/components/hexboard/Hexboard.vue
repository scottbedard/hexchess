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
        :style="{ pointerEvents: 'none' }" />

      <!-- positions -->
      <path
        v-for="hex, i in board"
        :d="d(flipped ? hex.path[1] : hex.path[0])"
        :data-hexboard-position="i"
        :fill="fill(hex)"
        :key="i" />

      <!-- labels -->
      <text
        v-for="[text, p, positionFlipped], i in labels"
        v-text="text"
        dominant-baseline="central"
        style="font-size: .45px"
        text-anchor="middle"
        :key="i"
        :style="{
          fontSize: '.5px',
          fill: 'oklch(70.4% 0.04 256.788)',
          pointerEvents: 'none',
        }"
        :x="x(flipped ? positionFlipped[0] : p[0])"
        :y="y(flipped ? positionFlipped[1] : p[1])" />
    </svg>
  </div>
</template>

<script lang="ts" setup>
import { board, box, colors, labels, perimeter } from './constants'
import { computed, ref } from 'vue'
import { d, x, y } from './geometry'
import { Hexchess } from '../../../js/src/hexchess'

defineProps<{
  flipped: boolean
  hexchess: Hexchess
}>()

//
// state
//

const flipped = ref(false)

//
// computed
//

const cursor = computed(() => {
  return 'pointer'
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