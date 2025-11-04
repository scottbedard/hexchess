import * as engine from './pkg/hexchess_engine.js'

await engine.default()

onmessage = evt => {
  const key = evt.data.key

  if (key === '@bedard/hexchess::evaluate') {
    console.log({
      key: 'evaluate',
      result: engine.init(),
    })
  }
}
