import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),

  kit: {
    // adapter-static gera HTML/JS/CSS puro — ideal para Tauri
    adapter: adapter({
      fallback: 'index.html', // SPA mode
    }),

    // Tauri espera o build em dist/
    outDir: 'build',
  },
};

export default config;
