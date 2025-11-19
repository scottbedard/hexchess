import { x, y } from './geometry'
import type { Vec } from './types'

/** svg namespace */
const ns = 'http://www.w3.org/2000/svg'

/** create svg vector path */
export function d(arr: [number, number][]) {
  if (arr.length === 0) return ''

  const [origin, ...points] = arr

  let path = `M ${x(origin[0])} ${y(origin[1])} L `

  for (const point of points) {
    path += `${x(point[0])} ${y(point[1])} `
  }

  return `${path} Z`
}

/** create svg path element */
export function path(points: Vec<2>[]) {
  const el = svgEl('path')
  el.setAttribute('d', d(points))
  return el
}

/** create slot element */
export function slot(name: string) {
  const slotEl = document.createElement('slot')
  slotEl.name = name
  return slotEl
}

/** create svg element */
export function svg() {
  const el = svgEl('svg')
  el.setAttribute('xmlns', ns)
  return el
}

function svgEl<T extends keyof SVGElementTagNameMap>(tag: T) {
  return document.createElementNS(ns, tag)
}
