<template>
  <div>
    <div class="p-3">
      <Checkbox
        v-model="flipped"
        label="Flipped" />

      <Combobox
        v-model="selectedSet"
        label="Pieces"
        :items="Object.keys(allPieces)" />
    </div>

    <Hexboard
      class="max-w-3xl mx-auto"
      :flipped
      :pieces
    />
  </div>
</template>

<script lang="ts" setup>
import { Checkbox, Combobox } from './components'
import { AlphaPieces, AnarcandyPieces, CalientePieces, GiocoPieces, Hexboard } from './lib'
import { computed, ref } from 'vue'

const allPieces = {
  alpha: AlphaPieces,
  anarcandy: AnarcandyPieces,
  caliente: CalientePieces,
  gioco: GiocoPieces,
}

const flipped = ref(false)

const selectedSet = ref<keyof typeof allPieces>('alpha')

const pieces = computed(() => allPieces[selectedSet.value])
</script>