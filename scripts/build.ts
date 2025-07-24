import { copy, dim, execAsync, green, read, resolve, write } from './utils'
import ora from 'ora'

async function buildJs() {
  // cleanup
  await execAsync(`rm -rf ${resolve('js/dist')}`, {
    cwd: 'js',
  })

  // generate types
  await execAsync('tsc --project ./tsconfig.json --emitDeclarationOnly', {
    cwd: 'js',
  })

  // build npm package
  await execAsync('npx rollup --config rollup.config.ts --configPlugin @rollup/plugin-typescript', {
    cwd: 'js',
  })

  copy('LICENSE', 'js/dist/LICENSE')
  copy('js/package.json', 'js/dist/package.json')
  copy('js/README.md', 'js/dist/README.md')

  // set version numbers
  const pkg = JSON.parse(read('js/package.json'))
  write('js/dist/index.mjs', read('js/dist/index.mjs').replace('x.y.z', pkg.version))
}

async function buildRust() {
  await execAsync('cargo build --release', {
    cwd: 'rust',
  })
}

async function run() {
  const args = process.argv.slice(2)
  const startAt = Date.now()
 
  if (args.includes('--js-only') || args.length === 0) {
    const js = ora('Building JavaScript...').start()
    await buildJs()
    js.succeed(`JavaScript ${dim(`(${Date.now() - startAt}ms)`)}`)
  }

  if (args.includes('--rs-only') || args.length === 0) {
    const rust = ora('Rust...').start()
    const rustStart = Date.now()
    await buildRust()
    rust.succeed(`Rust ${dim(`(${Date.now() - rustStart}ms)`)}`)
  }

  console.log()
  console.log(`${green('Done')} ${dim(`(${Date.now() - startAt}ms)`)}`)
}

run()
