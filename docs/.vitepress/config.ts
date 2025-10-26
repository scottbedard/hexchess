import { defineConfig } from 'vitepress'
import tailwindcss from '@tailwindcss/vite'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: 'Hexchess',
  description: 'The brain of hexchess.club',
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Libraries', link: '/libraries' }
    ],

    sidebar: [
      {
        text: 'Libraries',
        items: [
          { text: 'Overview', link: '/libraries' },
          { text: 'TypeScript', link: '/typescript' },
          { text: 'PHP', link: '/php' },
          { text: 'Rust', link: '/rust' },
        ]
      },

      {
        text: 'Engine',
        items: [
          { text: 'Sandbox', link: '/sandbox' }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/scottbedard/hexchess' }
    ]
  },
  vite: {
    plugins: [
      tailwindcss(),
    ]
  }
})
