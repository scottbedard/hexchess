import { execAsync, resolve } from './utils.js'

export async function testJs(options) {
  const args = [
    'test',
  ]

  // if (options?.coverage) {
  //   args.push('--coverage')
  // }

  if (options?.filter) {
    args.push(`--filter="${options.filter}"`)
  }

  await execAsync('pnpm', args, {
    cwd: resolve('js'),
    silent: options?.silent,
  })
}

export async function testPhp(options) {
  const args = [
    '--bootstrap=./vendor/autoload.php',
    '--log-junit=./php/junit.xml',
    '--cache-directory=./php/.pest/cache',
    '--test-directory=./php/tests',
  ]

  if (options?.filter) {
    args.push(`--filter="${options.filter}"`)
  }

  if (options?.coverage) {
    args.push('--coverage-clover=coverage.xml ')
  }

  await execAsync('./vendor/bin/pest', args, {
    silent: options?.silent,
  })
}

export async function testRust(options) {
  const args = [
    'test',
    '--all-features',
  ]

  await execAsync('cargo', args, {
    cwd: resolve('rust'),
    silent: options?.silent,
  })
}
