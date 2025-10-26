<template>
  <div>
    <Hexboard
      :flipped
      :hexchess
      :selected
      :targets
      @position-click="onPositionClick"
    />
  </div>
</template>

<script lang="ts" setup>
import { computed, onMounted, ref } from 'vue'
import { Hexchess } from '../../js/src/hexchess'
import { useEventListener } from '@vueuse/core'
import Hexboard from '../components/hexboard/Hexboard.vue'

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
// lifecycle
//

onMounted(() => {
  useEventListener(document.body, 'click', deselect)

  useEventListener(window, 'keydown', (evt) => {
    if (evt.key === 'Escape') {
      deselect()
    }
  })
})

//
// methods
//

function deselect() {
  selected.value = null
}

function onPositionClick(position: number) {
  selected.value = position
}
</script>
