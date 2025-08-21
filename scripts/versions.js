import { dim, green, read } from './utils.js'
import toml from 'smol-toml'

export function versions(options) {
  const cargo = toml.parse(read('rust/Cargo.toml'))
  const npm = JSON.parse(read('js/package.json'))
  const composer = JSON.parse(read('composer.json'))
  const cargoVersion = cargo.package.version
  const macrosVersion = toml.parse(read('rust/macros/Cargo.toml')).package.version

  console.log('Checking versions...')
  console.log()
  console.log(`NPM:             ${npm.version}`)
  console.log(`Composer:        ${composer.version}`)
  console.log(`Cargo:           ${cargoVersion}`)
  console.log(`Cargo (macros):  ${macrosVersion}`)

  const versions = [composer.version, npm.version, cargoVersion, macrosVersion]

  if (!versions.every(val => val === versions[0])) {
    throw new Error(`Version mismatch [npm: ${npm.version}, cargo: ${cargoVersion}, composer: ${composer.version}]`)
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
