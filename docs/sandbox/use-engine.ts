import { onUnmounted, ref } from 'vue'
import {
  evaluate as evaluateCommand,
  type EvaluateOptions
} from '../../engine/index'

export function useEngine() {
  const worker = new Worker(new URL('/engine/worker.js', location.origin), { type: 'module' })

  const loading = ref(false)

  const evaluate = async (options: EvaluateOptions) => {
    if (loading.value) {
      return
    }

    loading.value = true

    try {
      const result = await evaluateCommand(worker, options)
      loading.value = false
      return result
    } catch (err) {
      console.error(err)
      loading.value = false
    }
  }

  onUnmounted(() => {
    worker.terminate()
  })

  return {
    evaluate,
    loading,
  }
}