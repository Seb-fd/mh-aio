import adapter from '@sveltejs/adapter-static'
import { sveltekit } from '@sveltejs/kit/vite'
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte'
import tailwindcss from '@tailwindcss/vite'
import { defineConfig } from 'vite'

export default defineConfig({
  // SvelteKit 3: kit config lives here (svelte.config.js is no longer used).
  plugins: [
    tailwindcss(),
    sveltekit({
      adapter: adapter({
        fallback: 'index.html',
      }),
      preprocess: vitePreprocess(),
      // `$lib` was removed as a built-in in SvelteKit 3; re-add it explicitly
      // (deprecated `alias` API, still honored) to avoid renaming ~400 imports.
      alias: { $lib: 'src/lib' },
    }),
  ],
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
