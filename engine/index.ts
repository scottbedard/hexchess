/**
 * Trigger a message to the hexfish worker
 */
function trigger(command: string, options: Record<string, any> = {}) {
  const token = Math.random()

  return new Promise((resolve, reject) => {
    const listener = (evt: MessageEvent) => {
      if (evt.data.token === token && typeof evt.data.response === 'object') {
        globalThis.removeEventListener('message', listener)
        resolve({
          command,
          options: evt.data.options,
          response: evt.data.response,
        })
      }
    }

    globalThis.addEventListener('message', listener)
    globalThis.postMessage({ command, token, options })
  });
}

/** test for a connection with the engine worker */
export function ping() {
  return trigger('hexchess/ping')
}