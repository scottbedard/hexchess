(function() {
  'use strict';

  importScripts('./hexchess_engine.js');

  (async function() {
    await wasm_bindgen.default();

    onmessage = evt => {
      const { command, id, options } = evt.data

      if (typeof command === 'string' && typeof id === 'string') {
        const post = (response = {}) => postMessage({ id, response, options })

        switch (command) {
          case 'hexchess/evaluate':
            post(wasm_bindgen.evaluate(options))
            break
          case 'hexchess/ping':
            post({ now: Date.now() })
            break
        }
      }
    }
  })();
})();
