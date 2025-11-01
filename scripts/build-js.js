import { copy, execAsync, read, resolve, write } from './utils.js'

/** build js library for npm */
export async function buildJs(options) {
  // cleanup
  await execAsync('rm', ['-rf', resolve('js/dist')], {
    cwd: 'js',
    silent: options?.silent,
  })

  // generate types
  await execAsync('tsc', ['--project', './tsconfig.json', '--emitDeclarationOnly'], {
    cwd: 'js',
  })

  // build npm package
  await execAsync('npx', ['rollup', '--config', 'rollup.config.ts', '--configPlugin', '@rollup/plugin-typescript'], {
    cwd: 'js',
  })

  copy('LICENSE', 'js/dist/LICENSE')
  copy('js/package.json', 'js/dist/package.json')
  copy('README.md', 'js/dist/README.md')

  // set version numbers
  const pkg = JSON.parse(read('js/package.json'))
  write('js/dist/index.mjs', read('js/dist/index.mjs').replace('x.y.z', pkg.version))
}