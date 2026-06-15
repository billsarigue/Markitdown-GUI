import { sveltekit } from '@sveltejs/vite-plugin-svelte';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [sveltekit()],

  // Tauri espera o dev server nesta porta
  server: {
    port: 1420,
    strictPort: true,
    host: '127.0.0.1',
    watch: {
      // hot-reload mais eficiente em projetos Tauri
      ignored: ['**/src-tauri/**'],
    },
  },

  // Permite imports relativos do Tauri no build
  clearScreen: false,
  envPrefix: ['VITE_', 'TAURI_'],

  build: {
    // Chromium moderno embutido no Tauri — sem necessidade de polyfills legados
    target: 'chrome105',
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    sourcemap: !!process.env.TAURI_DEBUG,
    outDir: 'dist',
  },
});
