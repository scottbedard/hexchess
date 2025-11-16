import { execAsync, read, write, green, dim } from './utils.js'
import pkg from '../js/package.json' with { type: 'json' }

export async function version(options) {
  const startAt = Date.now()
  let version = options.version || pkg.version

  if (!version) {
    console.error('Missing version argument')
    process.exit(1)
  }

  // validate version format (semver)
  const semverRegex = /^(\d+)\.(\d+)\.(\d+)$/

  if (!semverRegex.test(version)) {
    console.error('Invalid version format')
    process.exit(1)
  }

  let [major, minor, patch] = version.split('.').map(Number)
  const versionIncremented = options?.major || options?.minor || options?.patch
  const explicitVersion = !!options.version

  if (!options.version) {
    if (options?.major) {
      patch = 0
      minor = 0
      major++
    } else if (options?.minor) {
      patch = 0
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

  // Cargo.toml (macros)
  try {
    const cargoPath = 'rust/macros/Cargo.toml'
    const cargoContent = read(cargoPath)
    const oldVersion = cargoContent.match(/version = "([^"]+)"/)?.[1]

    if (!oldVersion) {
      console.error('Could not find version in macros/Cargo.toml')
      process.exit(1)
    }

    const updatedContent = cargoContent.replace(
      /version = "([^"]+)"/,
      `version = "${version}"`
    )

    write(cargoPath, updatedContent)
    console.log(`rust/macros/Cargo.toml: ${green(version)}`)
  } catch (error) {
    console.error('Failed to update rust/macros/Cargo.toml:', error)
    process.exit(1)
  }

  // engine/package.json
  try {
    const enginePackagePath = 'engine/package.json'
    const enginePackage = JSON.parse(read(enginePackagePath))
    const currentEngineVersion = enginePackage.version

    // update if explicit version provided (resets dot version to 0, ignores --engine)
    // if major/minor/patch were incremented (resets dot version to 0, ignores --engine)
    // if --engine flag is set (increments dot version)
    if (explicitVersion || versionIncremented || options?.engine) {
      // parse version format: MAJOR.MINOR.PATCH+engine.N
      const versionRegex = /^(\d+\.\d+\.\d+)\+engine\.(\d+)$/
      const match = currentEngineVersion.match(versionRegex)

      let newEngineVersion

      if (explicitVersion || versionIncremented) {
        // explicit version provided or major/minor/patch were incremented, reset dot version to 0 (--engine flag is ignored)
        newEngineVersion = `${version}+engine.0`
      } else if (match) {
        // version already in +engine.N format, increment the dot number (--engine flag was used)
        const [, baseVersion, engineNumber] = match
        const newEngineNumber = parseInt(engineNumber, 10) + 1
        newEngineVersion = `${baseVersion}+engine.${newEngineNumber}`
      } else {
        // version not in +engine.N format, create it with the current base version
        newEngineVersion = `${currentEngineVersion}+engine.0`
      }

      enginePackage.version = newEngineVersion
      write(enginePackagePath, JSON.stringify(enginePackage, null, 2) + '\n')
      console.log(`engine/package.json: ${green(newEngineVersion)}`)
    } else {
      // output the current engine version without updating
      console.log(`engine/package.json: ${green(currentEngineVersion)}`)
    }
  } catch (error) {
    console.error('Failed to read/update engine/package.json:', error)
    process.exit(1)
  }

  console.log()
  console.log(`${green('Done')} ${dim(`(${Date.now() - startAt}ms)`)}`)
}

