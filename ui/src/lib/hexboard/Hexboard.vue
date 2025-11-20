<template>
  <div>
    <svg
      xmlns="http://www.w3.org/2000/svg"
      :viewBox="`0 0 ${box} ${box}`">

      <!-- backdrop -->
      <path
        class="pointer-events-none"
        :d="d(perimeter)"
        :fill="colors[1]"
      />

      <!-- positions & highlights -->
      

      <slot name="P" />
      <slot name="N" />
      <slot name="B" />
      <slot name="R" />
      <slot name="Q" />
      <slot name="K" />
      <slot name="p" />
      <slot name="n" />
      <slot name="b" />
      <slot name="r" />
      <slot name="q" />
      <slot name="k" />
    </svg>
  </div>
</template>

<script lang="ts" setup>
import { box, emptyPosition } from './constants'
import { colors, perimeter } from './constants'
import { d } from './dom'

withDefaults(
  defineProps<{
    flipped?: boolean
    position?: string
  }>(),
  {
    flipped: false,
    position: emptyPosition
  }
)
</script>