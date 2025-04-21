import { describe, expect, test } from 'vitest'
import { fileURLToPath } from 'url'
import { Hexchess } from '../../src'
import { index, walk } from '../../src/utils'
import fs from 'node:fs'
import path from 'node:path'

function read(file) {
  return fs.readFileSync(resolve(file), 'utf-8')
}

function resolve(file) {
  const __filename = fileURLToPath(import.meta.url)
  const __dirname = path.dirname(__filename)

  return path.resolve(__dirname, '..', file)
}

describe('graph', () => {
  const hexchess = new Hexchess()

  JSON.parse(read('../tests/graph.json')).forEach(spec => {
    test(`${spec.from} -> ${spec.direction}`, () => {
      expect(walk(hexchess, index(spec.from), spec.direction, 'w')).toEqual(spec.result.map(index))
    })
  })
})
