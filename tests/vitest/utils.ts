import { fileURLToPath } from 'url'
import fs from 'node:fs'
import path from 'node:path'

export function json(file: string) {
  return JSON.parse(read(file))
}

export function read(file: string) {
  return fs.readFileSync(resolve(file), 'utf-8')
}

export function resolve(...parts: string[]) {
  const __filename = fileURLToPath(import.meta.url)
  const __dirname = path.dirname(__filename)

  return path.resolve(__dirname, '..', ...parts)
}
