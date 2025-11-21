/** @jsxImportSource vue */
import { expect, test } from 'vitest'
import { defineComponent } from 'vue'
import { Hexboard } from '../lib'
import { render } from 'vitest-browser-vue'

test('properly handles form inputs', async () => {
  const TestComponent = defineComponent({
    setup() {
      return () => (
        <div>
          <Hexboard />
        </div>
      )
    }
  })

  const screen = render(<TestComponent />)

  expect(screen.container.querySelector('svg')).not.toBeNull()
})