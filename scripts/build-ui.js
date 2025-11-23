import { execAsync, resolve } from './utils.js'

/** build ui library */
export async function buildUi(options) {
  // build ui package
  await execAsync('npx', ['vite', 'build'], {
    cwd: resolve('ui'),
    silent: options?.silent,
  })
}

