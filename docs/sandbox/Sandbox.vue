<template>
  <div>
    <Hexboard
      :flipped
      :hexchess
    />
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import Hexboard from '../components/hexboard/Hexboard.vue'
import { Hexchess } from '../../js/src/hexchess'

const flipped = ref(false)

const hexchess = ref(Hexchess.init())
</script> 