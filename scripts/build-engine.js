import { copy, execAsync, resolve } from './utils.js'
import { existsSync, rmSync } from 'node:fs'

/** build engine package */
export async function buildEngine() {
  const pkg = resolve('./engine/pkg')
  const wasmPack = resolve('./node_modules/.bin/wasm-pack')

  // rebuild wasm package
  rmSync(pkg, { recursive: true, force: true })

  await execAsync(wasmPack, ['build', '--release'], { cwd: 'engine' })

  // remove unnecessary files
  for (const file of [
    '.gitignore',
    'package.json',
  ]) {
    const filePath = resolve(pkg, file)

    if (existsSync(filePath)) {
      rmSync(filePath)
    }
  }

  // copy license
  copy('LICENSE', resolve(pkg, 'LICENSE'))
}
