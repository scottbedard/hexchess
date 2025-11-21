<template>
  <Combobox as="div" v-model="model" @update:modelValue="query = ''">
    <ComboboxLabel v-if="label" v-text="label" class="block text-sm/6 font-medium text-gray-900"></ComboboxLabel>
    <div class="relative mt-2">
      <ComboboxInput
        class="block w-full rounded-md bg-white py-1.5 pr-12 pl-3 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6"
        @change="query = $event.target.value"
        @blur="query = ''"
      />
      <ComboboxButton class="absolute inset-y-0 right-0 flex items-center rounded-r-md px-2 focus:outline-hidden">
        <ChevronDownIcon class="size-5 text-gray-400" aria-hidden="true" />
      </ComboboxButton>

      <transition leave-active-class="transition ease-in duration-100" leave-from-class="" leave-to-class="opacity-0">
        <ComboboxOptions v-if="filteredItems.length > 0 || query.length > 0" class="absolute z-10 mt-1 max-h-60 w-full overflow-auto rounded-md bg-white py-1 text-base shadow-lg outline outline-black/5 sm:text-sm">
          <ComboboxOption v-if="queryItem" :value="queryItem" as="template" v-slot="{ active }">
            <li :class="['relative cursor-default px-3 py-2 select-none', active ? 'bg-indigo-600 text-white outline-hidden' : 'text-gray-900']">
              <span class="block truncate" v-text="query"></span>
            </li>
          </ComboboxOption>
          <ComboboxOption v-for="item in filteredItems" :key="item" :value="item" as="template" v-slot="{ active }">
            <li :class="['relative cursor-default px-3 py-2 select-none', active ? 'bg-indigo-600 text-white outline-hidden' : 'text-gray-900']">
              <span class="block truncate" v-text="item"></span>
            </li>
          </ComboboxOption>
        </ComboboxOptions>
      </transition>
    </div>
  </Combobox>
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue'
import { ChevronDownIcon } from '@heroicons/vue/20/solid'
import {
  Combobox,
  ComboboxButton,
  ComboboxInput,
  ComboboxLabel,
  ComboboxOption,
  ComboboxOptions,
} from '@headlessui/vue'

// props and v-model
const props = withDefaults(
  defineProps<{
    items?: string[]
    label?: string
  }>(),
  {
    items: () => [],
  }
)

const model = defineModel<string>({ default: '' })

// Local query for filtering/search
const query = ref('')

// Computed filtered item list
const filteredItems = computed(() =>
  query.value === ''
    ? props.items
    : props.items.filter((item) =>
        item.toLowerCase().includes(query.value.toLowerCase())
      )
)

const queryItem = computed(() => {
  if (query.value === '') return null
  // Only allow custom entry if not matching existing item
  return props.items.includes(query.value) ? null : query.value
})
</script>
