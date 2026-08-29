import { sveltekit } from '@sveltejs/kit/vite'
import tailwindcss from '@tailwindcss/vite'
import { defineConfig } from 'vite'

export default defineConfig({
  plugins: [tailwindcss(), sveltekit()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: process.env.TAURI_DEV_HOST ? '0.0.0.0' : '127.0.0.1',
    hmr: process.env.TAURI_DEV_HOST
      ? {
          protocol: 'ws',
          host: process.env.TAURI_DEV_HOST,
          port: 1421,
        }
      : {
          host: '127.0.0.1',
          port: 1421,
        },
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
})
