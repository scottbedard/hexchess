import { createApp } from 'vue'
import { Hexboard } from './lib'
import App from './App.vue'

customElements.define('hexchess-board', Hexboard)

createApp(App).mount('#app')
