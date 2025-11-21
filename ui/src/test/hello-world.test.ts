import { expect, test } from 'vitest'
import { render } from 'vitest-browser-vue'
import App from '../App.vue'

test('properly handles form inputs', async () => {
  const screen = render(App)

  expect(screen.container.querySelector('svg')).not.toBeNull()
})