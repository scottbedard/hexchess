import { exec } from 'child_process'
import { fileURLToPath } from 'url'
import fs from 'node:fs'
import path from 'node:path'

/** copy a file from one location to another */
export function copy(from, to) {
  fs.copyFileSync(resolve(from), resolve(to))
}

/** colorize text in dim */
export function dim(text) {
  return `\x1b[2m${text}\x1b[0m`
}

/** execute a command asynchronously */
export function execAsync(cmd: string, options: Parameters<typeof exec>[1]) {
  return new Promise((resolve, reject) => {
    exec(cmd, options, (error, stdout, stderr) => {
      if (error) {
        reject(error);
        return;
      }
      resolve({ stdout, stderr });
    });
  });
}

/** colorize text in green */
export function green(text) {
  return `\x1b[32m${text}\x1b[0m`
}

/** read a file from the project root */
export function read(file) {
  return fs.readFileSync(resolve(file), 'utf-8')
}

/** resolve a path from the project root */
export function resolve(file) {
  const __filename = fileURLToPath(import.meta.url)
  const __dirname = path.dirname(__filename)

  return path.resolve(__dirname, '..', file)
}

/** write a file to the project root */
export function write(file, content) {
  fs.writeFileSync(resolve(file), content)
}
