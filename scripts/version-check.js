import { dim, green, read } from './utils.js'
import toml from 'smol-toml'

export function versionCheck(options) {
  const cargo = toml.parse(read('rust/Cargo.toml'))
  const composer = JSON.parse(read('composer.json'))
  const engine = JSON.parse(read('engine/package.json'))
  const npm = JSON.parse(read('js/package.json'))
  const cargoVersion = cargo.package.version
  const macrosVersion = toml.parse(read('rust/macros/Cargo.toml')).package.version

  // Validate version formats
  const semverRegex = /^(\d+)\.(\d+)\.(\d+)$/
  const engineVersionRegex = /^(\d+\.\d+\.\d+)\+engine\.(\d+)$/

  if (!semverRegex.test(npm.version)) {
    throw new Error(`Invalid NPM version format: ${npm.version} (expected x.y.z)`)
  }

  if (!semverRegex.test(composer.version)) {
    throw new Error(`Invalid Composer version format: ${composer.version} (expected x.y.z)`)
  }

  if (!semverRegex.test(cargoVersion)) {
    throw new Error(`Invalid Cargo version format: ${cargoVersion} (expected x.y.z)`)
  }

  if (!semverRegex.test(macrosVersion)) {
    throw new Error(`Invalid Cargo (macros) version format: ${macrosVersion} (expected x.y.z)`)
  }

  if (!engineVersionRegex.test(engine.version)) {
    throw new Error(`Invalid Engine version format: ${engine.version} (expected x.y.z+engine.n)`)
  }

  console.log('Checking versions...')
  console.log()
  console.log(`NPM:             ${npm.version}`)
  console.log(`Composer:        ${composer.version}`)
  console.log(`Cargo:           ${cargoVersion}`)
  console.log(`Cargo (macros):  ${macrosVersion}`)
  console.log(`Engine:          ${engine.version}`)

  const versions = [
    composer.version,
    npm.version,
    cargoVersion,
    macrosVersion,
    engine.version.split('+').shift(),
  ]

  if (!versions.every(val => val === versions[0])) {
    throw new Error(`Version mismatch [npm: ${npm.version}, cargo: ${cargoVersion}, composer: ${composer.version}, engine: ${engine.version}]`)
  }

  if (options?.release) {
    if (options.release !== npm.version) {
      throw new Error(`Release version mismatch [${options.release}]`)
    }

    console.log(`Release:         ${green(options.release)}`)
  } else {
    console.log(`Release:         ${dim('None')}`)
  }

  console.log()
  console.log(green('Success'))
}

