<template>
  <svg v-if="type === 'p'" v-once xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"></svg>
  <svg v-else-if="type === 'n'" v-once xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"></svg>
  <svg v-else-if="type === 'b'" v-once xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"></svg>
  <svg v-else-if="type === 'r'" v-once xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"></svg>
  <svg v-else-if="type === 'q'" v-once xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"></svg>
  <svg v-else-if="type === 'k'" v-once xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"></svg>
  <svg v-else-if="type === 'P'" v-once xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"></svg>
  <svg v-else-if="type === 'N'" v-once xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"></svg>
  <svg v-else-if="type === 'B'" v-once xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"></svg>
  <svg v-else-if="type === 'R'" v-once xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"></svg>
  <svg v-else-if="type === 'Q'" v-once xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"></svg>
  <svg v-else-if="type === 'K'" v-once xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"></svg>
</template>

<script setup lang="ts">
import type { Piece } from '@bedard/hexchess'

defineProps<{
  type: Piece | null
}>()
</script>