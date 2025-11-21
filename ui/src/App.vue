<template>
  <div>
    <div class="p-3">
      <Checkbox
        v-model="flipped"
        label="Flipped" />
    </div>

    <Hexboard
      class="max-w-3xl mx-auto"
      :flipped
      :pieces="AlphaPieces"
    />
  </div>
</template>

<script lang="ts" setup>
import { Checkbox } from './components'
import { AlphaPieces, GiocoPieces, Hexboard } from './lib'
import { ref } from 'vue'

const flipped = ref(false)

const pieces = ref([
  'alpha',
  'gioco',
])
</script>