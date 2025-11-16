import * as engine from './pkg/hexchess_engine.js'

await engine.default()

onmessage = evt => {
  const { command, id, options } = evt.data

  if (typeof command === 'string' && typeof id === 'string') {
    const post = (response = {}) => postMessage({ id, response, options })

    switch (command) {
      case 'hexchess/evaluate':
        post(engine.evaluate(options))
        break
      case 'hexchess/ping':
        post({ now: Date.now() })
        break
    }
  }
}
