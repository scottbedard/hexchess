import { dim, green, read } from './utils'
import toml from 'smol-toml'

function run() {
  const cargo = toml.parse(read('rust/Cargo.toml'))
  const npm = JSON.parse(read('js/package.json'))
  const composer = JSON.parse(read('php/composer.json'))

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const cargoVersion = (cargo.package as any).version

  console.log('Checking versions...')
  console.log()
  console.log(`NPM:      ${npm.version}`)
  console.log(`Cargo:    ${cargoVersion}`)
  console.log(`Composer: ${composer.version}`)

  const versions = [composer.version, npm.version, cargoVersion]

  if (!versions.every(val => val === versions[0])) {
    throw new Error(`Version mismatch [npm: ${npm.version}, cargo: ${cargoVersion}, composer: ${composer.version}]`)
  }

  let releasing = false

  for (const arg of process.argv) {
    if (arg.startsWith('release=')) {
      const release = arg.slice(8)

      if (release !== npm.version) {
        throw new Error(`Release version mismatch [${release}]`)
      }

      console.log(`Release: ${release}`)
      releasing = true
    }
  }

  if (!releasing) {
    console.log(`Release: ${dim('None')}`)
  }

  console.log()
  console.log(green('Success'))
}

run()
