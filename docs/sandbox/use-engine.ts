import { computed, onMounted, shallowRef, toValue, type MaybeRef } from 'vue'
import { useEventListener } from '@vueuse/core'
import type { Hexchess, San } from '../../js/src'

export interface EvaluateOptions {
  depth?: MaybeRef<number>
  fen: MaybeRef<Hexchess>
}

export interface SearchResult {
  depth: number
  duration: number
  evaluations: number
  sans: { san: San, score: number }[]
}

export function useEngine() {
  let i = 0

  const loading = computed(() => reqs.value.length > 0)

  const reqs = shallowRef<[number, (result: SearchResult) => void][]>([])

  const evaluate = async (options: EvaluateOptions) => {
    const t = ++i

    const p = new Promise<SearchResult>((resolve) => {
      const startAt = performance.now()
    
      reqs.value.push([t, (r: SearchResult) => {
        r.duration = performance.now() - startAt
        return resolve(r)
      }])

      postMessage({
        key: 'hexchess/evaluate',
        options: {
          depth: toValue(options.depth) ?? 3,
          fen: toValue(options.fen).toString(),
        },
        token: t,
      })
    })

    return p
  }

  onMounted(() => {
    useEventListener(window, 'message', (evt: MessageEvent) => {
      const { key, token } = evt.data

      if (typeof key !== 'string' || typeof token !== 'number') {
        return
      }
      
      if (key === 'hexchess/evaluate/response') {
        const { result } = evt.data
        
        for (const [t, resolve] of reqs.value) {
          if (t === token) {
            reqs.value = reqs.value.filter(([val]) => val !== t)
            resolve(result)
            return
          }
        }
      }
    })
  })

  return {
    evaluate,
    loading,
  }
}