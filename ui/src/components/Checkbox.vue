<template>
  <fieldset>
    <legend v-text="label" class="sr-only"></legend>
    <div class="space-y-5">
      <div class="flex gap-2">
        <div class="flex h-6 shrink-0 items-center">
          <div class="group grid size-4 grid-cols-1">
            <input v-model="model" type="checkbox" class="col-start-1 row-start-1 appearance-none rounded-sm border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 checked:border-indigo-600 checked:bg-indigo-600 indeterminate:border-indigo-600 indeterminate:bg-indigo-600 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 disabled:border-gray-300 dark:disabled:border-gray-600 disabled:bg-gray-100 dark:disabled:bg-gray-800 disabled:checked:bg-gray-100 dark:disabled:checked:bg-gray-800 forced-colors:appearance-auto" :aria-describedby="descriptionId" :id="id" :name="id" />
            <svg class="pointer-events-none col-start-1 row-start-1 size-3.5 self-center justify-self-center stroke-white group-has-disabled:stroke-gray-950/25" viewBox="0 0 14 14" fill="none">
              <path class="opacity-0 group-has-checked:opacity-100" d="M3 8L6 11L11 3.5" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
              <path class="opacity-0 group-has-indeterminate:opacity-100" d="M3 7H11" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
            </svg>
          </div>
        </div>
        <div class="text-sm/6">
          <label v-text="label" class="font-medium text-gray-900 dark:text-gray-100" :for="id"></label>
          <p v-text="description" class="text-gray-500 dark:text-gray-400" :id="descriptionId"></p>
        </div>
      </div>
    </div>
  </fieldset>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(
  defineProps<{
    id?: string
    label?: string
    description?: string
  }>(),
  {
    id: () => crypto.randomUUID(),
  }
)

const model = defineModel<boolean>({ default: false })

const id = computed(() => props.id ?? crypto.randomUUID())
const descriptionId = computed(() => `${id.value}-description`)
</script>