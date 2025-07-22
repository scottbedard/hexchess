import { copy, dim, green, read, resolve, write } from './utils'
import { execSync } from 'child_process'

function run() {
  const startAt = Date.now()

  // cleanup
  execSync(`rm -rf ${resolve('dist')}`)

  // generate types
  execSync('tsc --project ./tsconfig.json --emitDeclarationOnly')

  // build npm package
  execSync('rollup --config rollup.config.ts --configPlugin @rollup/plugin-typescript')
  copy('../LICENSE', 'dist/LICENSE')
  copy('package.json', 'dist/package.json')
  copy('README.md', 'dist/README.md')

  // set version numbers
  const pkg = JSON.parse(read('package.json'))
  write('dist/index.mjs', read('dist/index.mjs').replace('x.y.z', pkg.version))

  console.log()
  console.log(`${green('Done')} ${dim(`${(Date.now() - startAt) / 1000}s`)}`)
}

run()
