import { copy, copyDir, execAsync, hashDir, read, resolve, write } from './utils.js'
import { existsSync, rmSync } from 'node:fs'

/** build engine packages */
export async function buildEngine(options) {
  const fingerprint = hashDir('engine/src').slice(0, 7)
  const pkg = resolve('./engine/pkg')
  const tsc = resolve('./node_modules/.bin/tsc')
  const wasmPack = resolve('./node_modules/.bin/wasm-pack')
  const workerPath = resolve('docs/public/engine/worker.js')

  // rebuild wasm package
  rmSync(pkg, { recursive: true, force: true })

  await execAsync(wasmPack, ['build', '--release', '--target', 'web'], { cwd: 'engine' })

  // remove unnecessary files
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

  // build worker files
  copyDir(pkg, resolve(`docs/public/engine/${fingerprint}`))
  copy('engine/worker.js', workerPath)
  write(workerPath, read(workerPath).replace(`'./pkg/hexchess_engine.js'`, `'./${fingerprint}/hexchess_engine.js'`))
  write('engine/pkg/worker.js', read('engine/worker.js').replace(`'./pkg/hexchess_engine.js'`, `'./hexchess_engine.js'`))

  // copy package files
  copy('engine/package.json', resolve(pkg, 'package.json'))
  copy('LICENSE', resolve(pkg, 'LICENSE'))

  // build library interface and copy a local version for docs to use
  await execAsync(tsc, ['--project', resolve('engine/tsconfig.json')], { cwd: 'engine' })
}
