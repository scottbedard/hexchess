import { onMounted, onUnmounted, shallowRef, type TemplateRef } from 'vue'

export function useDomRect(el: TemplateRef<Element>) {
  const rect = shallowRef(new DOMRect())

  function measure() {
    rect.value = el.value?.getBoundingClientRect() ?? new DOMRect()
  }

  function reset() {
    rect.value = new DOMRect()
  }

  onMounted(() => {
    measure()
    window.addEventListener('resize', measure)
    window.addEventListener('scroll', measure)
  })

  onUnmounted(() => {
    window.removeEventListener('resize', measure)
    window.removeEventListener('scroll', measure)
  })

  return {
    measure,
    rect,
    reset,
  }
}