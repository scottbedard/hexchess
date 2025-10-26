import { box } from './constants'
import { sideLength } from './constants'
import type { Vec } from './types'

/** bilinearly interpolate between two points */
export function bilerp(p1: Vec<2>, p2: Vec<2>, t: number): Vec<2> {
  return [
    p1[0] + (t * (p2[0] - p1[0])),
    p1[1] + (t * (p2[1] - p1[1])),
  ]
}

/** create svg vector path */
export function d(arr: [number, number][]) {
  const p = 4 // <- svg rounding precision

  const [origin, ...points] = arr

  let path = `M ${x(origin[0]).toFixed(p)} ${y(origin[1]).toFixed(p)} L `

  for (const point of points) {
    path += `${x(point[0]).toFixed(p)} ${y(point[1]).toFixed(p)} `
  }

  return `${path} Z`
}

/** reflect a path of points across the origin */
export function flip<T extends Vec<2>[]>(path: T) {
  return path.map(reflect) as T
}

/** calculate hexagon vertices around a given point */
export function hexagon(origin: Vec<2>): Vec<6, Vec<2>> {
  return [
    pivot(origin, -300, sideLength),
    pivot(origin, 0, sideLength),
    pivot(origin, -60, sideLength),
    pivot(origin, -120, sideLength),
    pivot(origin, -180, sideLength),
    pivot(origin, -240, sideLength),
  ]
}

/** pivot counter-clockwise around a point */
export function pivot(point: Vec<2>, deg: number, distance: number): Vec<2> {
  const rad = radians(deg)

  return [
    (distance * Math.cos(rad)) + point[0],
    (distance * Math.sin(rad)) + point[1],
  ]
}

/** convert degrees to radians */
export function radians(deg: number) {
  return deg * (Math.PI / 180)
}

/** reflect a point across the origin */
export function reflect(v: Vec<2>): Vec<2> {
  return [-v[0], -v[1]]
}

/** sum two vectors */
export function sum(v1: Vec<2>, v2: Vec<2>): Vec<2> {
  return [v1[0] + v2[0], v1[1] + v2[1]]
}

/** convert cartesian X value to SVG coordinate */
export function x(n: number) {
  return (box / 2) + n
}

/** convert cartesian Y value to SVG coordinate */
export function y(n: number) {
  return (box / 2) - n
}
