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

      <button
        class="flex gap-x-1.5 items-center text-sm tracking-wide hover:text-(--vp-code-color)!"
        :disabled="loading"
        @click="onPlayClick">
        <svg xmlns="http://www.w3.org/2000/svg" class="size-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 8V4H8"/><rect width="16" height="12" x="4" y="8" rx="2"/><path d="M2 14h2"/><path d="M20 14h2"/><path d="M15 13v2"/><path d="M9 13v2"/></svg>

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
import { useEngine, type SearchResult } from './use-engine'
import { useEventListener } from '@vueuse/core'
import EvaluationResult from './EvaluationResult.vue'
import Hexboard from '../components/hexboard/Hexboard.vue'
import Input from '../components/Input.vue'

const { evaluate, loading } = useEngine()

//
// state
//

const depth = ref(3)

const flipped = ref(false)

const highlighted = ref<number[]>([])

const hexchess = ref(Hexchess.init())

const selected = ref<number | null>(null)

const evaluation = ref<SearchResult | null>(null)

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

function handleMove(from: number, to: number) {
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
  if (loading.value) {
    return
  }

  console.log({ depth: depth.value, fen: hexchess.value })

  const result = await evaluate({
    depth: depth.value,
    fen: hexchess.value,
  })

  evaluation.value = result

  console.log({ result })
  
  if (result.sans.length > 0) {
    const best = result.sans[0]

    const next = hexchess.value.clone()

    try {
      next.applyMoveUnsafe(best.san)
      hexchess.value = next
      highlighted.value = [best.san.from, best.san.to]
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
