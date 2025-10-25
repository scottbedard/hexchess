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
        fill="#f00" />
    </svg>
  </div>
</template>

<script lang="ts" setup>
import { box, perimeter } from './constants'
import { computed } from 'vue'
import { d } from './helpers'

//
// computed
//

const cursor = computed(() => {
  return 'pointer'
})
</script>