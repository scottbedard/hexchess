export interface EvaluateOptions {
  depth: number
  position: string
}

export interface EvaluateResponse {
  depth: number
  evaluations: number
  sans: { san: string, score: number }[]
}

export interface PingResponse {
  now: number
}

export interface ExecuteResponse<T extends Record<string, any> = {}> {
  command: string
  options: Record<string, unknown>
  response: T
}

/** execute a command with the engine worker */
export function execute<T extends Record<string, any> = {}>(command: string, options: Record<string, any> = {}) {
  const token = Math.random()

  return new Promise<ExecuteResponse<T>>((resolve, reject) => {
    const listener = (evt: MessageEvent) => {
      if (evt.data.token === token && typeof evt.data.response === 'object') {
        globalThis.removeEventListener('message', listener)

        resolve({
          command,
          options: evt.data.options,
          response: evt.data.response as T,
        })
      }
    }

    globalThis.addEventListener('message', listener)
    globalThis.postMessage({ command, token, options })
  });
}

/** evaluate a position */
export function evaluate(options: EvaluateOptions) {
  return execute<EvaluateResponse>('hexchess/evaluate', options)
}

/** test for a connection with the engine worker */
export function ping() {
  return execute<PingResponse>('hexchess/ping')
}