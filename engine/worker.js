import * as engine from './pkg/hexchess_engine.js'

await engine.default()

onmessage = evt => {
  const { command, token, options } = evt.data

  if (
    typeof command === 'string' &&
    typeof token === 'number'
  ) {
    const post = (response = {}) => postMessage({ response, options, token })

    switch (command) {
      case 'hexchess/ping':
        post({ now: Date.now() })
        break
    }
  }

  

  // if (command === 'hexchess/evaluate') {
  //   const { fen, depth } = evt.data.options

  //   postMessage({
  //     id,
  //     key: 'hexchess/evaluate',
  //     result: engine.evaluate(fen, depth),
  //   })
  // }
}
