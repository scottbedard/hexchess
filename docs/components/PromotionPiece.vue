<template>
  <button class="aspect-square border-r! border-gray-500/30! flex items-center justify-center h-14 w-14 dark:hover:bg-gray-600! first:rounded-l-lg">
    <Component 
      class="size-10"
      :is="piece" />
  </button>
</template>

<script lang="ts" setup>
import type { FunctionalComponent } from 'vue'

defineProps<{
  piece: FunctionalComponent
}>()
</script>

