import { createApp } from 'vue'
import { HexchessBoard } from './lib'
import App from './App.vue'

customElements.define('hexchess-board', HexchessBoard)

createApp(App).mount('#app')
