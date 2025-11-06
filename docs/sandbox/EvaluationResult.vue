<template>
  <div class="leading-loose text-sm tracking-wide">
    <div>
      Depth: {{ result.depth }}
    </div>

    <div>
      Evaluations: {{ result.evaluations.toLocaleString() }}
    </div>

    <div>
      Duration: {{ result.duration.toFixed(0) }}ms
    </div>

    <div>
      Speed: {{ (result.evaluations / result.duration).toFixed(1) }} evals/ms
    </div>
  </div>
</template>

<script lang="ts" setup>
import type { SearchResult } from './use-engine'

const props = defineProps<{
  result: SearchResult
}>()
</script>