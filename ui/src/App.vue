<template>
  <div>
    <input type="checkbox" id="flipped" v-model="flipped" />
    <label for="flipped">Flipped</label>

    <Hexboard
      :flipped
      :pieces="GiocoPieces"
    />
  </div>
</template>

<script lang="ts" setup>
import { GiocoPieces, Hexboard } from './lib'
import { ref } from 'vue'

const flipped = ref(false)
</script>