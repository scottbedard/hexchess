import * as engine from './4e1482b/hexchess_engine.js'

await engine.default()

onmessage = evt => {
  const { command, token, options } = evt.data

  if (
    typeof command === 'string' &&
    typeof token === 'number'
  ) {
    const post = (response = {}) => postMessage({ response, options, token })

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
