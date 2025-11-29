<template>
  <Listbox as="div" v-model="model">
    <ListboxLabel v-text="label" class="block text-sm/6 font-medium text-gray-900 dark:text-gray-100"></ListboxLabel>
    <div class="relative mt-2">
      <ListboxButton class="grid w-full cursor-default grid-cols-1 rounded-md bg-white dark:bg-gray-800 py-1.5 pr-2 pl-3 text-left text-gray-900 dark:text-gray-100 outline-1 -outline-offset-1 outline-gray-300 dark:outline-gray-600 focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-indigo-600 sm:text-sm/6">
        <span v-text="display" class="col-start-1 row-start-1 truncate pr-6"></span>
        <ChevronUpDownIcon class="col-start-1 row-start-1 size-5 self-center justify-self-end text-gray-500 dark:text-gray-400 sm:size-4" aria-hidden="true" />
      </ListboxButton>

      <transition leave-active-class="transition ease-in duration-100" leave-from-class="" leave-to-class="opacity-0">
        <ListboxOptions class="absolute z-10 mt-1 max-h-60 w-full overflow-auto rounded-md bg-white dark:bg-gray-800 py-1 text-base shadow-lg outline-1 outline-black/5 dark:outline-gray-700 sm:text-sm">
          <ListboxOption as="template" v-for="item in items" :key="item.display" :value="item.value" v-slot="{ active, selected }">
            <li :class="[active ? 'bg-indigo-600 text-white outline-hidden' : 'text-gray-900 dark:text-gray-100', 'relative cursor-default py-2 pr-9 pl-3 select-none']">
              <span v-text="item.display" :class="[selected ? 'font-semibold' : 'font-normal', 'block truncate']"></span>

              <span v-if="selected" :class="[active ? 'text-white' : 'text-indigo-600 dark:text-indigo-400', 'absolute inset-y-0 right-0 flex items-center pr-4']">
                <CheckIcon class="size-5" aria-hidden="true" />
              </span>
            </li>
          </ListboxOption>
        </ListboxOptions>
      </transition>
    </div>
  </Listbox>
</template>

<script setup lang="ts" generic="T extends string | number | boolean | object | null | undefined">
import { computed } from 'vue'
import { Listbox, ListboxButton, ListboxLabel, ListboxOption, ListboxOptions } from '@headlessui/vue'
import { ChevronUpDownIcon } from '@heroicons/vue/16/solid'
import { CheckIcon } from '@heroicons/vue/20/solid'

const model = defineModel<T>({ required: true })

const props = defineProps<{
  items: Array<{ display: number | string; value: T }>
  label?: string
}>()

const display = computed(() => {
  return props.items.find(item => item.value === model.value)?.display
})
</script>