/** hexboard options */
export interface HexboardOptions {
  /** position colors */
  colors: [string, string, string]
  /** color of highlighted position */
  highlightColor: string
  /** color of active label relative to mouseover */
  labelActiveColor: string
  /** label color */
  labelColor: string
  /** fill color of inactive label relative to mouseover */
  labelInactiveColor: string
  /** show labels */
  labels: boolean
  /** color of target circles */
  targetColor: string
  /** color of selected position */
  selectedColor: string
}

/** uniform tuple of length `T`, `number` by default */
export type Vec<
  T extends number,
  U = number,
  V extends unknown[] = []
> = V['length'] extends T ? V : Vec<T, U, [U, ...V]>
