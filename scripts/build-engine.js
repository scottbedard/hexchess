import { copy, copyDir, execAsync, hashDir, read, resolve, write } from './utils.js'
import { existsSync, rmSync } from 'node:fs'

/** build engine package */
export async function buildEngine() {
  const pkg = resolve('./engine/pkg')
  const wasmPack = resolve('./node_modules/.bin/wasm-pack')
  const fingerprint = hashDir('engine/src').slice(0, 7)
  const workerPath = resolve('docs/public/engine/worker.js')

  // rebuild wasm package
  rmSync(pkg, { recursive: true, force: true })

  await execAsync(wasmPack, ['build', '--release', '--target', 'web'], { cwd: 'engine' })

  // remove obsolete files
  for (const file of [
    resolve('docs/public/engine'),
    resolve(pkg, '.gitignore'),
    resolve(pkg, 'package.json'),
    workerPath,
  ]) {
    const filePath = resolve(pkg, file)

    if (existsSync(filePath)) {
      rmSync(filePath, { recursive: true, force: true })
    }
  }
  
  // create vitepress public files
  copyDir('engine/pkg', resolve(`docs/public/engine/${fingerprint}`))

  copy('engine/worker.js', workerPath)

  write(workerPath, read(workerPath).replace(`'./pkg/hexchess_engine.js'`, `'./${fingerprint}/hexchess_engine.js'`))

  // copy license
  copy('LICENSE', resolve(pkg, 'LICENSE'))
}
