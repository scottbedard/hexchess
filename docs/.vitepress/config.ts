import { defineConfig } from 'vitepress'

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
        text: 'Game Libraries',
        items: [
          { text: 'Overview', link: '/libraries' },
          { text: 'Rust', link: '/rust' },
          { text: 'JavaScript', link: '/javascript' },
          { text: 'PHP', link: '/php' }
        ]
      },

      {
        text: 'Engine Development',
        items: [
          { text: 'Sandbox', link: '/sandbox' }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/scottbedard/hexchess' }
    ]
  }
})
