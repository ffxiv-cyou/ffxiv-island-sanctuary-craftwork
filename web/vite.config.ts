import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import wasmPack from 'vite-plugin-wasm-pack';

// https://vitejs.dev/config/
export default defineConfig({
  build: {
    minify: true,
    assetsInlineLimit: 8192,
    rollupOptions: {
      input: {
        main: 'index.html',
        ngld: 'ngld.html',
      },
    },
  },
  plugins: [
    wasmPack('../mji-craftwork'),
    vue({
      template: {
        compilerOptions: {
          isCustomElement: tag => tag === "icon",
        }
      }
    })
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    }
  },
  define: {
    BUILD_TIMESTAMP: new Date().getTime(),
  }
})
