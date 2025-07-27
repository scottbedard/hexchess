import { buildJs, buildRust } from './scripts/build.js'
import { setVersion } from './scripts/set-version.js'
import { dim, execAsync, green } from './scripts/utils.js'
import { program } from 'commander'
import { testJs, testPhp, testRust } from './scripts/test.js'
import { versions } from './scripts/versions.js'
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
  .command('build:js')
  .description('Build NPM package')
  .action(buildJs)

program
  .command('build:rs')
  .description('Build Rust crate')
  .action(buildRust)

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
// set version
//

program
  .command('set-version <version>')
  .description('Set the version of the project')
  .action((version) => {
    setVersion({ version })
  })

//
// test
//

program
  .command('test')
  .description('Run all tests')
  .action(async () => {
    const startAt = Date.now()

    const rust = ora('Rust...').start()
    await testRust({ silent: true })
    rust.succeed(`Rust ${dim(`(${Date.now() - startAt}ms)`)}`)

    const php = ora('PHP...').start()
    await testPhp({ silent: true })
    php.succeed(`PHP ${dim(`(${Date.now() - startAt}ms)`)}`)

    const js = ora('JavaScript...').start()
    await testJs({ silent: true })
    js.succeed(`JavaScript ${dim(`(${Date.now() - startAt}ms)`)}`)

    console.log()
    console.log(green('Done'), dim(`(${Date.now() - startAt}ms)`))
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
// versions
//

program
  .command('versions')
  .description('Check the versions of the dependencies')
  .action(versions)

program.parse()
  