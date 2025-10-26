<template>
  <div>
    <Hexboard
      :flipped
      :hexchess
      :selected
      :targets
      @position-click="onPositionClick"
    />

    <Input
      v-model="fen"
      label="FEN"
      name="fen"
      select-all />

    <div class="flex gap-x-6 mt-4">
      <button
        class="flex gap-x-1.5 items-center text-sm tracking-wide hover:text-(--vp-code-color)!"
        @click="onResetClick">
        <svg xmlns="http://www.w3.org/2000/svg" class="size-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/><path d="M3 3v5h5"/></svg>
        
        Reset
      </button>

      <button
        class="flex gap-x-1.5 items-center text-sm tracking-wide hover:text-(--vp-code-color)!"
        @click="onClearClick">
        <svg xmlns="http://www.w3.org/2000/svg" class="size-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 21H8a2 2 0 0 1-1.42-.587l-3.994-3.999a2 2 0 0 1 0-2.828l10-10a2 2 0 0 1 2.829 0l5.999 6a2 2 0 0 1 0 2.828L12.834 21"/><path d="m5.082 11.09 8.828 8.828"/></svg>
      
        Clear
      </button>

      <button
        class="flex gap-x-1.5 items-center text-sm tracking-wide hover:text-(--vp-code-color)!"
        @click="onFlipClick">
        <svg xmlns="http://www.w3.org/2000/svg" class="size-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21 16-4 4-4-4"/><path d="M17 20V4"/><path d="m3 8 4-4 4 4"/><path d="M7 4v16"/></svg>
      
        Flip
      </button>
    </div>
  </div>
</template>

<script lang="ts" setup>
import Input from '../components/Input.vue'
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

const fen = computed({
  get: () => hexchess.value.toString(),
  set: (value) => {
    if (value) {
      try {
        const next = Hexchess.parse(value)
        hexchess.value = next
      } catch { }
    }
  }
})

const targets = computed(() => selected.value ? hexchess.value.movesFrom(selected.value) : [])

//
// lifecycle
//

onMounted(() => {
  useEventListener(document.body, 'click', deselect)

  useEventListener(window, 'keydown', (evt) => {
    if (evt.key === 'Escape') {
      deselect()
    } else if (evt.key === 'Delete' && selected.value !== null) {
      hexchess.value.board[selected.value] = null
      deselect()
    } else if (selected.value !== null && ['p', 'b', 'n', 'r', 'q', 'k', 'P', 'B', 'N', 'R', 'Q', 'K'].includes(evt.key)) {
      hexchess.value.board[selected.value] = evt.key as any
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

function onClearClick() {
  hexchess.value = new Hexchess()
}

function onFlipClick() {
  flipped.value = !flipped.value
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

function onResetClick() {
  hexchess.value = Hexchess.init()
}
</script>
