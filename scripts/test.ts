import { dim, execAsync, green, resolve } from './utils'
import ora from 'ora'

async function run() {
  const startAt = Date.now()
  const js = ora('JavaScript...').start()

  await execAsync('pnpm test', {
    cwd: resolve('js'),
    encoding: 'utf-8',
  })

  js.succeed(`JavaScript ${dim(`(${Date.now() - startAt}ms)`)}`)

  const rust = ora('Rust...').start()
  const rustStart = Date.now()

  await execAsync('cargo test --all-features', {
    cwd: resolve('rust'),
    encoding: 'utf-8'
  })

  rust.succeed(`Rust ${dim(`(${Date.now() - rustStart}ms)`)}`)

  console.log()
  console.log(`${green('Done')} ${dim(`(${Date.now() - startAt}ms)`)}`)
}

run()
