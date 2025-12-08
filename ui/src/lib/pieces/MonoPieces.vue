<template>
  <svg v-if="type === 'p'" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"></svg>
  <svg v-else-if="type === 'n'" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"></svg>
  <svg v-else-if="type === 'b'" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"></svg>
  <svg v-else-if="type === 'r'" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"></svg>
  <svg v-else-if="type === 'q'" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"></svg>
  <svg v-else-if="type === 'k'" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"></svg>
  <svg v-else-if="type === 'P'" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"></svg>
  <svg v-else-if="type === 'N'" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"></svg>
  <svg v-else-if="type === 'B'" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"></svg>
  <svg v-else-if="type === 'R'" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"></svg>
  <svg v-else-if="type === 'Q'" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"></svg>
  <svg v-else-if="type === 'K'" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"></svg>
</template>

<script setup lang="ts">
import type { Piece } from '@bedard/hexchess'

defineProps<{
  type: Piece | null
}>()
</script>