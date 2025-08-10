import { fileURLToPath } from 'url'
import { parse } from 'yaml'
import fs from 'node:fs'
import path from 'node:path'

export function json(file: string) {
  return JSON.parse(read('tests', file))
}

export function read(...parts: string[]) {
  return fs.readFileSync(resolve(...parts), 'utf-8')
}

export function resolve(...parts: string[]) {
  const __filename = fileURLToPath(import.meta.url)
  const __dirname = path.dirname(__filename)

  return path.resolve(__dirname, '../..', ...parts)
}

export function yaml<T>(file: string) {
  return parse(read('yaml-tests', file)) as T
}
