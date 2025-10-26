<template>
  <div>
    <label
      v-if="label"
      v-text="label"
      class="cursor-pointer font-bold text-sm tracking-wide"
      :for="name"
      @click="onLabelClick" />

    <input
      v-model="model"
      class="block h-10 outline-1 -outline-offset-1 outline-(--vp-c-divider) rounded-md px-3! w-full focus:outline-1! focus:-outline-offset-2 focus:outline-(--vp-code-color)! sm:text-sm/6"
      ref="inputEl"
      :aria-label="label"
      :autocomplete="normalAutocomplete"
      :autofocus
      :disabled
      :id
      :max
      :maxlength
      :min
      :minlength
      :name
      :placeholder
      :readonly
      :required
      :selectAll
      :type
      @click="onClick"
    />
  </div>
</template>

<script lang="ts" setup>
import { computed, useTemplateRef } from 'vue'

const props = defineProps<{
  autocomplete?: boolean | string
  autofocus?: boolean
  disabled?: boolean
  id?: string
  label?: string
  max?: string | number
  maxlength?: string | number
  min?: string | number
  minlength?: string | number
  name?: string
  placeholder?: string
  readonly?: boolean
  required?: boolean
  selectAll?: boolean
  type?: string
}>()

const model = defineModel<string>({ default: '' })

const inputEl = useTemplateRef('inputEl')

const normalAutocomplete = computed(() => {
  if (typeof props.autocomplete === 'string') {
    return props.autocomplete
  }

  if (typeof props.autocomplete === 'boolean') {
    return props.autocomplete ? 'on' : 'off'
  }

  return undefined
})

function selectAllInput() {
  inputEl.value?.select()
}

function onClick() {
  if (props.selectAll) {
    selectAllInput()
  }
}

function onLabelClick() {
  if (props.selectAll) {
    selectAllInput()
  } else {
    inputEl.value?.focus()
  }
}
</script>