import * as engine from './18b2bc36/hexchess_engine.js'

await engine.default()

onmessage = evt => {
  const {
    key,
  } = evt.data

  console.log({ key })

  if (evt.data.key === '@bedard/hexchess::evaluate') {
    console.log({
      key: 'evaluate',
      result: engine.init(),
    })
  }
}
