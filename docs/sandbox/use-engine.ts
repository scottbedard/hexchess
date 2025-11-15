import { ref } from 'vue'
import {
  evaluate as evaluateCommand,
  type EvaluateOptions
} from '../../engine/index'

export function useEngine() {
  const loading = ref(false)

  const evaluate = async (options: EvaluateOptions) => {
    if (loading.value) {
      return
    }

    loading.value = true

    try {
      const result = await evaluateCommand(options)
      loading.value = false
      return result
    } catch (error) {
      console.error(error)
      loading.value = false
    }
  }

  return {
    evaluate,
    loading,
  }
}