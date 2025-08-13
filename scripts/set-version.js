import { execAsync, read, write, green, dim } from './utils.js'
import pkg from '../js/package.json' with { type: 'json' }

export async function setVersion(options) {
  const startAt = Date.now()
  let version = options.version || pkg.version

  if (!version) {
    console.error('Missing version argument')
    process.exit(1)
  }

  // validate version format (semver)
  const semverRegex = /^(\d+)\.(\d+)\.(\d+)$/

  if (!semverRegex.test(version)) {
    console.error('Invalid version format. Please use semantic versioning.')
    process.exit(1)
  }

  let [major, minor, patch] = version.split('.').map(Number)

  if (!options.version) {
    if (options?.major) {
      major++
    } else if (options?.minor) {
      minor++
    } else if (options?.patch) {
      patch++
    }

    version = `${major}.${minor}.${patch}`
  }

  // composer.json
  try {
    const composerPath = 'composer.json'
    const composer = JSON.parse(read(composerPath))
    composer.version = version
    write(composerPath, JSON.stringify(composer, null, 2) + '\n')
    console.log(`composer.json: ${green(version)}`)
  } catch (error) {
    console.error('Failed to update php/composer.json:', error)
    process.exit(1)
  }

  // package.json
  try {
    const jsPackagePath = 'js/package.json'
    const jsPackage = JSON.parse(read(jsPackagePath))
    jsPackage.version = version
    write(jsPackagePath, JSON.stringify(jsPackage, null, 2) + '\n')
    console.log(`js/package.json: ${green(version)}`)
  } catch (error) {
    console.error('Failed to update js/package.json:', error)
    process.exit(1)
  }

  // pnpm-lock.yaml
  try {
    await execAsync('pnpm', ['install'], {
      cwd: 'js',
    })
  } catch (error) {
    console.error('Failed to generate pnpm-lock.yaml', error)
    process.exit(1)
  }

  // Cargo.toml
  try {
    const cargoPath = 'rust/Cargo.toml'
    const cargoContent = read(cargoPath)
    const oldVersion = cargoContent.match(/version = "([^"]+)"/)?.[1]

    if (!oldVersion) {
      console.error('Could not find version in Cargo.toml')
      process.exit(1)
    }

    const updatedContent = cargoContent.replace(
      /version = "([^"]+)"/,
      `version = "${version}"`
    )

    write(cargoPath, updatedContent)
    console.log(`rust/Cargo.toml: ${green(version)}`)
  } catch (error) {
    console.error('Failed to update rust/Cargo.toml:', error)
    process.exit(1)
  }

  // Cargo.lock
  try {
    const cargoPath = 'rust/Cargo.lock'
    const cargoContent = read(cargoPath)
    const oldVersion = cargoContent.match(/hexchess"\nversion = "([^"]+)"/)?.[1]

    if (!oldVersion) {
      console.error('Could not find version in Cargo.lock')
      process.exit(1)
    }

    const updatedContent = cargoContent.replace(
      /hexchess"\nversion = "([^"]+)"/,
      `hexchess"\nversion = "${version}"`
    )

    write(cargoPath, updatedContent)
    console.log(`rust/Cargo.lock: ${green(version)}`)
  } catch (error) {
    console.error('Failed to update rust/Cargo.lock:', error)
    process.exit(1)
  }

  console.log()
  console.log(`${green('Done')} ${dim(`(${Date.now() - startAt}ms)`)}`)
}
