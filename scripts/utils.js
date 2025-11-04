import { fileURLToPath } from 'url'
import { spawn } from 'child_process'
import crypto from 'node:crypto'
import fs from 'node:fs'
import path from 'node:path'

/** copy a file from one location to another */
export function copy(from, to) {
  fs.copyFileSync(resolve(from), resolve(to))
}

/** copy a directory from one location to another */
export function copyDir(from, to) {
  fs.cpSync(resolve(from), resolve(to), { recursive: true })
}

/** colorize text in dim */
export function dim(text) {
  return `\x1b[2m${text}\x1b[0m`
}

/** execute a command asynchronously */
export function execAsync(cmd, args, options) {
  return new Promise((resolve, reject) => {
    const child = spawn(cmd, args, options);

    if (!options?.silent) {
      child.stdout?.on('data', (data) => process.stdout.write(data));
      child.stderr?.on('data', (data) => process.stderr.write(data));
    }

    child.on('close', (code) => {
      if (code === 0) {
        resolve(code);
      } else {
        const err = new Error(`Process exited with code ${code}`);
        err.code = code;
        reject(err);
      }
    });

    child.on('error', (err) => {
      reject(err);
    });
  });
}

/** colorize text in green */
export function green(text) {
  return `\x1b[32m${text}\x1b[0m`
}

/**
 * Recursively hash all files in a directory
 */
export function hashDir(dir) {
  const read = dir => {
    const entries = fs.readdirSync(dir, { withFileTypes: true })
    let files = []
    for (const entry of entries) {
      const fullPath = path.join(dir, entry.name)
      if (entry.isDirectory()) {
        files = files.concat(read(fullPath))
      } else if (entry.isFile()) {
        files.push(fullPath)
      }
    }
    return files
  }

  const files = read(resolve(dir)).sort()

  return crypto.createHash('sha256').update(files.join('')).digest('hex')
}




/** read a file from the project root */
export function read(file) {
  return fs.readFileSync(resolve(file), 'utf-8')
}

/** resolve a path from the project root */
export function resolve(...paths) {
  const __filename = fileURLToPath(import.meta.url)
  const __dirname = path.dirname(__filename)

  return path.resolve(__dirname, '..', ...paths)
}

/** write a file to the project root */
export function write(file, content) {
  fs.writeFileSync(resolve(file), content)
}
