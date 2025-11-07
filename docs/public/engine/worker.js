import * as engine from './6108a31/hexchess_engine.js'

await engine.default()

onmessage = evt => {
  const key = evt.data.key
  const token = evt.data.token

  if (typeof key !== 'string' || typeof token !== 'number') {
    return
  }

  if (key === 'hexchess/evaluate') {
    const { fen, depth } = evt.data.options

    postMessage({
      key: 'hexchess/evaluate/response',
      result: engine.evaluate(fen, depth),
      token,
    })
  }
}
