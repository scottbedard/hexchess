import { buildEngine } from './scripts/build-engine.js'
import { buildJs } from './scripts/build-js.js'
import { buildRust } from './scripts/build-rust.js'
import { dim, execAsync, green } from './scripts/utils.js'
import { program } from 'commander'
import { testEngine, testJs, testPhp, testRust } from './scripts/test.js'
import { versionCheck } from './scripts/version-check.js'
import { version } from './scripts/version.js'
import ora from 'ora'

//
// build
//

program
  .command('build')
  .description('Build all projects')
  .action(async () => {
    const startAt = Date.now()

    const js = ora('Building NPM package...').start()
    await buildJs({ silent: true })
    js.succeed(`NPM package ${dim(`(${Date.now() - startAt}ms)`)}`)

    const rust = ora('Building Rust crate...').start()
    const rustStart = Date.now()
    await buildRust({ silent: true })
    rust.succeed(`Rust crate ${dim(`(${Date.now() - rustStart}ms)`)}`)

    console.log()
    console.log(green('Done'), dim(`(${Date.now() - startAt}ms)`))
  })

program
  .command('build:engine')
  .description('Build engine package')
  .action(buildEngine)

program
  .command('build:js')
  .description('Build NPM package')
  .action(buildJs)

program
  .command('build:rs')
  .description('Build Rust crate')
  .action(buildRust)

//
// docs
//

program
  .command('docs:dev')
  .description('Develop docs')
  .action(async () => {
    await execAsync('./node_modules/.bin/vitepress', ['dev', 'docs'])
  })

program
  .command('docs:build')
  .description('Build docs')
  .action(async () => {
    await execAsync('./node_modules/.bin/vitepress', ['build', 'docs'])
  })

program
  .command('docs:preview')
  .description('Preview docs')
  .action(async () => {
    await execAsync('./node_modules/.bin/vitepress', ['preview', 'docs'])
  })

//
// lint
//

program
  .command('lint:php')
  .description('Run linting')
  .action(async () => {
    await execAsync('./vendor/bin/php-cs-fixer', ['fix', '.', '--dry-run', '--diff', '--format=txt'])
  })

//
// test
//

program
  .command('test')
  .description('Run all tests')
  .action(async () => {
    const startAt = Date.now()
  
    const rustStartAt = Date.now()
    const rust = ora('Rust...').start()
    await testRust({ silent: true })
    rust.succeed(`Rust ${dim(`(${Date.now() - rustStartAt}ms)`)}`)

    const phpStartAt = Date.now()
    const php = ora('PHP...').start()
    await testPhp({ silent: true })
    php.succeed(`PHP ${dim(`(${Date.now() - phpStartAt}ms)`)}`)

    const jsStartAt = Date.now()
    const js = ora('JavaScript...').start()
    await testJs({ silent: true })
    js.succeed(`JavaScript ${dim(`(${Date.now() - jsStartAt}ms)`)}`)

    const engineStartAt = Date.now()
    const engine = ora('Engine...').start()
    await testEngine({ silent: true })
    engine.succeed(`Engine ${dim(`(${Date.now() - engineStartAt}ms)`)}`)

    console.log()
    console.log(green('Done'), dim(`(${Date.now() - startAt}ms)`))
  })

program
  .command('test:engine')
  .description('Run engine tests')
  .option('-c, --coverage', 'Generate coverage report')
  .option('-f, --filter <filter>', 'Filter tests by name')
  .option('-s, --silent', 'Run tests silently')
  .action(async (options) => {
    await testEngine({
      coverage: options?.coverage,
      filter: options?.filter,
      silent: options?.silent,
    })
  })

program
  .command('test:js')
  .description('Run JavaScript tests')
  .option('-c, --coverage', 'Generate coverage report')
  .option('-f, --filter <filter>', 'Filter tests by name')
  .option('-s, --silent', 'Run tests silently')
  .action(async (options) => {
    await testJs({
      coverage: options?.coverage,
      filter: options?.filter,
      silent: options?.silent,
      watch: options?.watch,
    })
  })

program
  .command('test:php')
  .description('Run PHP tests')
  .option('-c, --coverage', 'Generate coverage report')
  .option('-f, --filter <filter>', 'Filter tests by name')
  .option('-s, --silent', 'Run tests silently')
  .action(async (options) => {
    await testPhp({
      coverage: options?.coverage,
      filter: options?.filter,
      silent: options?.silent,
    })
  })

program
  .command('test:rs')
  .description('Run Rust tests')
  .option('-c, --coverage', 'Generate coverage report')
  .option('-s, --silent', 'Run tests silently')
  .action(async (options) => {
    await testRust({
      coverage: options?.coverage,
      silent: options?.silent,
    })
  })

//
// version
//

program
  .command('version [version]')
  .option('-M, --major', 'Increment the major version')
  .option('-m, --minor', 'Increment the minor version')
  .option('-p, --patch', 'Increment the patch version')
  .option('-e, --engine', 'Increment the engine version')
  .description('Set the version of the project')
  .action((versionArg, options) => {
    version({
      major: options?.major,
      minor: options?.minor,
      patch: options?.patch,
      engine: options?.engine,
      version: versionArg,
    })
  })

program
  .command('version:check')
  .description('Check the versions of the dependencies')
  .option('-r, --release <release>', 'Release version')
  .action((options) => {
    versionCheck({
      release: options?.release,
    })
  })

program.parse()
  