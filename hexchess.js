import { dim, green } from './scripts/utils.js'
import { program } from 'commander'
import { testJs, testPhp, testRust } from './scripts/test.js'
import ora from 'ora'

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
  .option('-f, --filter <filter>', 'Filter tests by name')
  .option('-s, --silent', 'Run tests silently')
  .action(async (options) => {
    await testJs({
      filter: options?.filter,
      silent: options?.silent,
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
  .command('test:rust')
  .description('Run Rust tests')
  .option('-s, --silent', 'Run tests silently')
  .action(async (options) => {
    await testRust({
      silent: options?.silent,
    })
  })

program.parse()
  