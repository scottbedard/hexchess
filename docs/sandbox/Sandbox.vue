<template>
  <div>
    <Input
      v-model="fen"
      name="fen"
      select-all />

    <div class="flex gap-x-6 mt-4">
      <button
        class="flex gap-x-1.5 items-center text-sm tracking-wide hover:text-(--vp-code-color)!"
        @click="onResetClick">
        <svg class="size-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/><path d="M3 3v5h5"/></svg>
        
        Reset
      </button>

      <button
        class="flex gap-x-1.5 items-center text-sm tracking-wide hover:text-(--vp-code-color)!"
        @click="onClearClick">
        <svg class="size-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 21H8a2 2 0 0 1-1.42-.587l-3.994-3.999a2 2 0 0 1 0-2.828l10-10a2 2 0 0 1 2.829 0l5.999 6a2 2 0 0 1 0 2.828L12.834 21"/><path d="m5.082 11.09 8.828 8.828"/></svg>
      
        Clear
      </button>

      <button
        class="flex gap-x-1.5 items-center text-sm tracking-wide hover:text-(--vp-code-color)!"
        @click="onFlipClick">
        <svg class="size-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21 16-4 4-4-4"/><path d="M17 20V4"/><path d="m3 8 4-4 4 4"/><path d="M7 4v16"/></svg>
      
        Flip
      </button>

      <button
        :class="[
          'flex gap-x-1.5 items-center text-sm tracking-wide hover:text-(--vp-code-color)!',
          loading && 'opacity-50 pointer-events-none',
        ]"
        :disabled="loading"
        @click="onPlayClick">
        <Spinner
          v-if="loading"
          class="size-4" />
          
        <svg
          v-else
          class="size-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 20v2"/><path d="M12 2v2"/><path d="M17 20v2"/><path d="M17 2v2"/><path d="M2 12h2"/><path d="M2 17h2"/><path d="M2 7h2"/><path d="M20 12h2"/><path d="M20 17h2"/><path d="M20 7h2"/><path d="M7 20v2"/><path d="M7 2v2"/><rect x="4" y="4" width="16" height="16" rx="2"/><rect x="8" y="8" width="8" height="8" rx="1"/></svg>
        
        Play
      </button>
    </div>

    <div class="h-12 relative">
      <EvaluationResult
        class="absolute top-2"
        v-model:depth="depth"
        :evaluation />
    </div>

    <Hexboard
      :flipped
      :hexchess
      :highlighted
      :selected
      :targets
      @position-click="onPositionClick"
    />
  </div>
</template>

<script lang="ts" setup>
import { computed, onMounted, ref } from 'vue'
import { Hexchess, San } from '../../js/src'
import { useEngine } from './use-engine'
import { useEventListener } from '@vueuse/core'
import EvaluationResult from './EvaluationResult.vue'
import Hexboard from '../components/hexboard/Hexboard.vue'
import Input from '../components/Input.vue'
import Spinner from '../components/Spinner.vue'
import type { EvaluateResponse } from '../../engine/index'

const { evaluate, loading } = useEngine()

//
// state
//

const depth = ref(3)

const flipped = ref(false)

const highlighted = ref<number[]>([])

const hexchess = ref(Hexchess.init())

const selected = ref<number | null>(null)

const evaluation = ref<EvaluateResponse | null>(null)

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
    } else if ((evt.key === 'Delete' || evt.key === 'Backspace') && selected.value !== null) {
      hexchess.value.board[selected.value] = null
      deselect()
    } else if (selected.value !== null && 'pbnrqkPBNRQK'.includes(evt.key)) {
      hexchess.value.board[selected.value] = evt.key as any
    }
  })
})

//
// methods
//

function deselect() {
  highlighted.value = []
  selected.value = null
}

async function handleMove(from: number, to: number) {
  const san = new San({ from, to })

  evaluation.value = null
  hexchess.value.applyMoveUnsafe(san)
  highlighted.value = []
}

function onClearClick() {
  evaluation.value = null
  hexchess.value = new Hexchess()
  highlighted.value = []
}

function onFlipClick() {
  flipped.value = !flipped.value
}

async function onPlayClick() {
  const data = await evaluate({
    depth: depth.value,
    position: hexchess.value.toString(),
  })

  if (!data) {
    return
  }

  evaluation.value = data?.response
  
  if (data.response.sans.length > 0) {
    const best = data.response.sans[0]
    const san = San.from(best.san)
    const next = hexchess.value.clone()

    try {
      next.applyMoveUnsafe(san)
      hexchess.value = next
      highlighted.value = [san.from, san.to]
    } catch {
      return
    }
  }
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
  highlighted.value = []
}
</script>
