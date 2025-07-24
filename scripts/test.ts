import { dim, execAsync, green, resolve } from './utils'
import ora from 'ora'

async function testJs() {
  await execAsync('pnpm test', {
    cwd: resolve('js'),
    encoding: 'utf-8',
  })
}

async function testRust() {
  await execAsync('cargo test --all-features', {
    cwd: resolve('rust'),
    encoding: 'utf-8',
  })
}

async function run(args: string[]) {
  const startAt = Date.now()

  if (args.includes('--js-only') || args.length === 0) {
    const js = ora('JavaScript...').start()
    await testJs()
    js.succeed(`JavaScript ${dim(`(${Date.now() - startAt}ms)`)}`)
  }

  if (args.includes('--rs-only') || args.length === 0) {
    const rust = ora('Rust...').start()
    const rustStart = Date.now()
    await testRust()
    rust.succeed(`Rust ${dim(`(${Date.now() - rustStart}ms)`)}`)
  }

  console.log()
  console.log(`${green('Done')} ${dim(`(${Date.now() - startAt}ms)`)}`)
}

run(process.argv.slice(2))
