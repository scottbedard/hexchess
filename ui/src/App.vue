<template>
  <div class="min-h-screen bg-white dark:bg-gray-900">
    <div class="gap-3 grid p-3">
      <div class="flex flex-wrap gap-6">
        <Select
          v-model="selectedPieces"
          class="max-w-32 w-full"
          label="Pieces"
          :items="pieceItems" />

        <Select
          v-model="playing"
          class="max-w-32 w-full"
          label="Playing"
          :items="playingItems"/>
      </div>

      <div class="flex flex-wrap gap-6">
        <Checkbox
          v-model="active"
          label="Active" />

        <Checkbox
          v-model="flipped"
          label="Flipped" />
      </div>
    </div>

    <Hexboard
      class="max-w-3xl mx-auto"
      :active
      :flipped
      :pieces="selectedPieces"
      :options="{
        // ...
      }"
      @click-position="onClickPosition"
    />
  </div>
</template>

<script lang="ts" setup>
import type { Color } from '@bedard/hexchess'
import { ref, shallowRef, type Component } from 'vue'
import { Checkbox, Select } from './components'
import {
  AlphaPieces,
  AnarcandyPieces,
  CalientePieces,
  CaliforniaPieces,
  CardinalPieces,
  CburnettPieces,
  // CelticPieces,
  Chess7Pieces,
  ChessnutPieces,
  CompanionPieces,
  // CookePieces,
  DisguisedPieces,
  DubrovnyPieces,
  // FantasyPieces,
  FiriPieces,
  FrescaPieces,
  GiocoPieces,
  GovernorPieces,
  HorseyPieces,
  IcpiecesPieces,
  KiwenSuwiPieces,
  // KosalPieces,
  LeipzigPieces,
  LetterPieces,
  MaestroPieces,
  MeridaPieces,
  MonarchyPieces,
  MonoPieces,
  MpchessPieces,
  PirouettiPieces,
  PixelPieces,
  ReillycraigPieces,
  // RhosgfxPieces,
  RiohachaPieces,
  ShapesPieces,
  // SpatialPieces,
  StauntyPieces,
  TatianaPieces,
  XkcdPieces,
  Hexboard
} from './lib'

const pieces = {
  alpha: AlphaPieces,
  anarcandy: AnarcandyPieces,
  caliente: CalientePieces,
  california: CaliforniaPieces,
  cardinal: CardinalPieces,
  cburnett: CburnettPieces,
  // celtic: CelticPieces,
  chess7: Chess7Pieces,
  chessnut: ChessnutPieces,
  companion: CompanionPieces,
  // cooke: CookePieces,
  // disguised: DisguisedPieces,
  dubrovny: DubrovnyPieces,
  // fantasy: FantasyPieces,
  firi: FiriPieces,
  fresca: FrescaPieces,
  gioco: GiocoPieces,
  governor: GovernorPieces,
  horsey: HorseyPieces,
  icpieces: IcpiecesPieces,
  'kiwen-suwi': KiwenSuwiPieces,
  // kosal: KosalPieces,
  leipzig: LeipzigPieces,
  letter: LetterPieces,
  maestro: MaestroPieces,
  merida: MeridaPieces,
  monarchy: MonarchyPieces,
  mono: MonoPieces,
  mpchess: MpchessPieces,
  pirouetti: PirouettiPieces,
  pixel: PixelPieces,
  reillycraig: ReillycraigPieces,
  // rhosgfx: RhosgfxPieces,
  riohacha: RiohachaPieces,
  shapes: ShapesPieces,
  // spatial: SpatialPieces,
  staunty: StauntyPieces,
  tatiana: TatianaPieces,
  xkcd: XkcdPieces,
}

const playingItems: Array<{ display: string; value: Color | boolean }> = [
  { display: 'Black', value: 'b' },
  { display: 'White', value: 'w' },
  { display: 'Both', value: true },
  { display: 'None', value: false },
] as const

const active = ref(true)

const flipped = ref(false)

const pieceItems: Array<{
  display: string
  value: Component
}> = Object.entries(pieces).map(([display, value]) => ({ display, value }))

const selectedPieces = shallowRef<Component>(pieces.gioco)

const playing = ref<Color | boolean>(true)

// const pieces = computed(() => pieces[selectedSet.value.value as keyof typeof pieces])

function onClickPosition(position: number) {
  // ...
}
</script>