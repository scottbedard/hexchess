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
export function execute<T extends Record<string, any> = {}>(
  worker: Worker,
  command: string,
  options: Record<string, any> = {}
) {
  const id = crypto.randomUUID()

  return new Promise<ExecuteResponse<T>>((resolve) => {
    const listener = (evt: MessageEvent) => {
      if (evt.data.id === id && typeof evt.data.response === 'object') {
        worker.removeEventListener('message', listener)

        resolve({
          command,
          options: evt.data.options,
          response: evt.data.response as T,
        })
      }
    }

    worker.addEventListener('message', listener)
    worker.postMessage({ command, id, options })
  });
}

/** evaluate a position */
export function evaluate(worker: Worker, options: EvaluateOptions) {
  return execute<EvaluateResponse>(worker, 'hexchess/evaluate', options)
}

/** test for a connection with the engine worker */
export function ping(worker: Worker) {
  return execute<PingResponse>(worker, 'hexchess/ping')
}