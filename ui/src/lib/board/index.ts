import { board, box, colors, perimeter } from './constants'
import { path, slot, svg } from './dom'

export class HexchessBoard extends HTMLElement {
  private perimeterEl: SVGPathElement

  private positionEls: SVGPathElement[] = []

  private svgEl: SVGSVGElement

  /**
   * Create <hexchess-board /> element
   */
  constructor() {
    super();
    
    this.attachShadow({ mode: 'open' })

    // root <svg />
    this.svgEl = svg()
    this.svgEl.setAttribute('viewBox', `0 0 ${box} ${box}`)

    // perimeter <path />
    this.perimeterEl = path(perimeter)
    this.perimeterEl.setAttribute('fill', colors[1])

    // position <path />'s
    const points = this.isFlipped() ? 4 : 3

    for (const position of board) {
      const el = path(position[points])
      el.setAttribute('fill', colors[position[0]])
      this.positionEls.push(el)
    }

    // construct dom tree
    this.shadowRoot?.append(
      this.svgEl,
      slot('p'),
      slot('n'),
      slot('b'),
      slot('r'),
      slot('q'),
      slot('k'),
      slot('P'),
      slot('N'),
      slot('B'),
      slot('R'),
      slot('Q'),
      slot('K'),
    )

    this.svgEl.append(this.perimeterEl, ...this.positionEls)
  }

  connectedCallback() {
    // ...
  }

  isFlipped() {
    const val = this.getAttribute('flipped')
    return val === 'true' || val === ''
  }
}
