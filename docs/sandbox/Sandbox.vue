<template>
  <div>
    <Hexboard
      :flipped
      :hexchess
      :targets
      @position-click="onPositionClick"
    />
  </div>
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue'
import Hexboard from '../components/hexboard/Hexboard.vue'
import { Hexchess } from '../../js/src/hexchess'

//
// state
//

const flipped = ref(false)

const hexchess = ref(Hexchess.init())

const selected = ref<number | null>(null)

//
// computed
//

const targets = computed(() => {
  return selected.value ? hexchess.value.movesFrom(selected.value) : []
})

//
// methods
//

function onPositionClick(position: number) {
  selected.value = position
}
</script>
