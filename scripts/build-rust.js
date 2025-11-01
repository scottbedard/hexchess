import { copy, execAsync } from './utils.js'

/** build rust library */
export async function buildRust(options) {
  copy('README.md', 'rust/README.md')

  await execAsync('cargo', ['build', '--release'], {
    cwd: 'rust',
    silent: options?.silent,
  })
}
