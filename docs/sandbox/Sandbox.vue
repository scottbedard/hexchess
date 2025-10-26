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
import { Hexchess, San } from '../../js/src'
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

const targets = computed(() => selected.value ? hexchess.value.movesFrom(selected.value) : [])

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

function handleMove(from: number, to: number) {
  const san = new San({ from, to })

  hexchess.value.applyMoveUnsafe(san)
}

function onPositionClick(position: number) {
  if (selected.value === position) {
    deselect()
    return
  }

  if (
    selected.value !== null &&
    targets.value.some(p => p.to === position)
  ) {
    handleMove(selected.value, position)
    deselect()
    return
  }

  selected.value = position
}
</script>
