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
      v-model:selected="selected"
      active
      autoselect
      ignore-turn
      :flipped
      :hexchess
      :highlight
      :playing="true"
      @move="onMove"
      @click-position="onClickPosition">
      <template #promotion="{ b, cancel,  n, promote, q, r }">
        <div
          class="absolute bottom-full flex flex-row left-1/2 shadow-lg rounded-lg -translate-x-1/2 dark:bg-gray-700"
        >
          <PromotionPiece :piece="n" @click="promote('n')" />
          <PromotionPiece :piece="b" @click="promote('b')" />
          <PromotionPiece :piece="q" @click="promote('q')" />
          <PromotionPiece :piece="r" @click="promote('r')" />
          <button
            @click="cancel"
            class="cursor-pointer flex items-center justify-center h-14 rounded-r-lg w-14 dark:hover:bg-gray-600!">
            <X class="size-8" />
          </button>
        </div>
      </template>
    </Hexboard>
  </div>
</template>

<script lang="ts" setup>
import { computed, onMounted, ref } from 'vue'
import { Hexboard } from '@bedard/hexboard'
import { Hexchess, San } from '../../js/src'
import { useEngine } from './use-engine'
import { useEventListener } from '@vueuse/core'
import EvaluationResult from './EvaluationResult.vue'
import Input from '../components/Input.vue'
import PromotionPiece from '../components/PromotionPiece.vue'
import Spinner from '../components/Spinner.vue'
import type { EvaluateResponse } from '../../engine/index'
import X from '../components/icons/X.vue'

const { evaluate, loading } = useEngine()

//
// state
//

const depth = ref(3)

const flipped = ref(false)

const highlight = ref<number[]>([])

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

//
// lifecycle
//

onMounted(() => {
  useEventListener(window, 'keydown', (evt) => {
    if (evt.key === 'Escape') {
      deselect()
    } else if ((evt.key === 'Delete' || evt.key === 'Backspace') && selected.value !== null) {
      hexchess.value.board[selected.value] = null
      deselect()
    } else if (selected.value !== null && 'pbnrqkPBNRQK'.includes(evt.key)) {
      evt.preventDefault()
      hexchess.value.board[selected.value] = evt.key as any
    }
  })
})

//
// methods
//

function deselect() {
  highlight.value = []
  selected.value = null
}

function onClearClick() {
  evaluation.value = null
  hexchess.value = new Hexchess()
  highlight.value = []
}

function onFlipClick() {
  flipped.value = !flipped.value
}

function onMove(san: San) {
  hexchess.value.applyMoveUnsafe(san)
  highlight.value = [san.from, san.to]
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
      highlight.value = [san.from, san.to]
    } catch {
      return
    }
  }
}

function onClickPosition(position: number) {
  selected.value = position
}

function onResetClick() {
  hexchess.value = Hexchess.init()
  highlight.value = []
}
</script>
