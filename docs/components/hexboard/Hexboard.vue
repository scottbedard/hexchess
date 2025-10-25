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
        :fill="colors[1]" />

      <!-- positions -->
      <path
        v-for="hex, i in board"
        :d="d(flipped ? hex.path[1] : hex.path[0])"
        :data-hexboard-position="i"
        :fill="fill(hex)"
        :key="i" />
    </svg>
  </div>
</template>

<script lang="ts" setup>
import { board, box, colors, perimeter } from './constants'
import { computed, ref } from 'vue'
import { d } from './geometry'

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