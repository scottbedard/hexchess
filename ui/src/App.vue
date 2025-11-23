<template>
  <div class="min-h-screen bg-white dark:bg-gray-900">
    <div class="gap-3 grid p-3">
      <Select
        v-model="selectedSet"
        class="max-w-40 w-full"
        label="Pieces"
        :items="pieceItems" />

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
      :pieces
      :options="{
        // ...
      }"
      @click-position="onClickPosition"
    />
  </div>
</template>

<script lang="ts" setup>
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
import { computed, ref } from 'vue'

const allPieces = {
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
  disguised: DisguisedPieces,
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

const active = ref(true)

const flipped = ref(false)

const pieceItems = computed(() =>
  Object.keys(allPieces).map(key => ({
    display: key,
    value: key,
  }))
)

const selectedSet = ref<{ display: string; value: string }>({ display: 'gioco', value: 'gioco' })

const pieces = computed(() => allPieces[selectedSet.value.value as keyof typeof allPieces])

function onClickPosition(position: number) {
  console.log('onClickPosition', position)
}
</script>