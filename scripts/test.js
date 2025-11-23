import { execAsync, resolve } from './utils.js'

export async function testEngine(options) {
  const args = [
    'test',
    '--all-features',
  ]
  
  if (options?.coverage) {
    args.push('--coverage')
  }

  if (options?.filter) {
    args.push(`--filter="${options.filter}"`)
  }
  
  if (options?.coverage) {
    args.length = 0

    args.push(
      '+nightly',
      'tarpaulin',
      '--verbose',
      '--all-features',
      '--workspace',
      '--timeout=120',
      '--out=xml'
    )
  }

  await execAsync('cargo', args, {
    cwd: resolve('engine'),
    silent: options?.silent,
  })
}

export async function testJs(options) {
  const args = [
    'vitest',
  ]

  if (options?.filter) {
    args.push(options.filter)
  }

  if (options?.coverage) {
    args.push('--coverage')
  }

  await execAsync('npx', args, {
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

  if (options?.coverage) {
    args.length = 0

    args.push(
      '+nightly',
      'tarpaulin',
      '--verbose',
      '--all-features',
      '--workspace',
      '--timeout=120',
      '--out=xml'
    )
  }

  await execAsync('cargo', args, {
    cwd: resolve('rust'),
    silent: options?.silent,
  })
}

export async function testUi(options) {
  const args = [
    'vitest',
  ]

  if (options?.filter) {
    args.push(options.filter)
  }

  if (options?.coverage) {
    args.push('--coverage')
  }

  if (options?.ui) {
    args.push('--ui')
  }

  await execAsync('npx', args, {
    cwd: resolve('ui'),
    silent: options?.silent,
  })
}
