<template>
  <div class="min-h-screen bg-white dark:bg-gray-900">
    <div class="gap-3 grid p-3">
      <div class="flex flex-wrap gap-x-6 gap-y-3">
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

    <div class="h-200" />

    <Hexboard
      v-model:hexchess="hexchess"
      v-model:selected="selected"
      class="max-w-3xl mx-auto"
      autoselect
      :active
      :flipped
      :pieces="selectedPieces"
      :options="{
        // ...
      }"
      :playing
      @click-position="onClickPosition"
      @move="onMove">
      <template #promotion="{ file, promote }">
        <div
          class="absolute inset-0 shadow-lg"
          ref="promotionEl">
          <div
            ref="promotionContentEl"
            :class="[
              'absolute',
              {
                'bottom-full': promotionTop,
                'left-0': 'ab'.includes(file),
                'left-1/2 -translate-x-1/2': 'cdefghi'.includes(file),
                'right-0': 'kl'.includes(file),
                'top-full': !promotionTop,
              }
            ]">
            <div
              class="bg-gray-200 flex my-2 rounded-lg dark:bg-gray-800">
              <button
                class="border-r border-gray-500/50 cursor-pointer rounded-l-lg size-14 dark:hover:bg-gray-700"
                @click="promote('q')">
                q
              </button>

              <button
                class="border-r border-gray-500/50 cursor-pointer size-14 dark:hover:bg-gray-700"
                @click="promote('r')">
                r
              </button>

              <button
                class="border-r border-gray-500/50 cursor-pointer size-14 dark:hover:bg-gray-700"
                @click="promote('b')">
                b
              </button>

              <button
                class="cursor-pointer rounded-r-lg size-14 dark:hover:bg-gray-700"
                @click="promote('n')">
                n
              </button>
            </div>
          </div>
        </div>
      </template>
    </Hexboard>

    <div class="h-200" />
  </div>
</template>

<script lang="ts" setup>
import { Hexchess, type Color, type San } from '@bedard/hexchess'
import { computed, nextTick, ref, shallowRef, type Component, useTemplateRef, watch } from 'vue'
import { Checkbox, Select } from './components'
import { useDomRect } from './composables/use-dom-rect'
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
  // DisguisedPieces,
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

const promotionEl = useTemplateRef('promotionEl')

const promotionContentEl = useTemplateRef('promotionContentEl')

const {
  measure: measurePromotionRect,
  rect: promotionRect
} = useDomRect(promotionEl)

const {
  measure: measurePromotionContentRect,
  rect: promotionContentRect
} = useDomRect(promotionContentEl)

const active = shallowRef(true)

const flipped = shallowRef(false)

const hexchess = ref(Hexchess.parse('b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1'))

const selected = shallowRef<number | null>(null)

const pieceItems: Array<{
  display: string
  value: Component
}> = Object.entries(pieces).map(([display, value]) => ({ display, value }))

const selectedPieces = shallowRef<Component>(pieces.gioco)

const playing = shallowRef<Color | boolean>(true)

const promotionTop = computed(() => {
  return promotionRect.value.top - promotionContentRect.value.height > 0
})

//
// watchers
//

watch(promotionEl, async () => {
  await nextTick()
  measurePromotionContentRect()
  measurePromotionRect()
})

//
// methods
//

/** handle click position */
function onClickPosition(index: number) {
  console.log('onClickPosition', index)
}

/** handle move */
function onMove(san: San) {
  console.log({ san })
  hexchess.value.applyMoveUnsafe(san)
}
</script>
