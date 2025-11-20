import { x, y } from './geometry'
import type { Vec } from './types'

/** create svg vector path */
export function d(arr: Vec<2>[]) {
  if (arr.length === 0) {
    return ''
  }

  const [origin, ...points] = arr

  let path = `M ${x(origin[0])} ${y(origin[1])} L `

  for (const point of points) {
    path += `${x(point[0])} ${y(point[1])} `
  }

  return `${path} Z`
}
