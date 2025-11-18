---
layout: doc
---

# Game engine

The game engine is designed for [Web Workers](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API), with a NPM package to facilitate communication with the worker. To get started, install [`@bedard/hexchess-engine`](https://www.npmjs.com/package/@bedard/hexchess-engine).

```sh
npm install @bedard/hexchess-engine
```

This documentation assumes a Vite environment, you may need to adjust worker construction for other bundlers. You may also need to [configure Vite for `es` workers](https://v3.vitejs.dev/config/worker-options.html#worker-format).

```js
import { evaluate } from '@bedard/hexchess-engine'
import EngineWorker from '@bedard/hexchess-engine/worker.js?worker'

const worker = new EngineWorker()

await evaluate(worker, {
  depth: 3,
  position: 'b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1',
})
```