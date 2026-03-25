import { resolve } from 'path'
import { defineConfig } from 'electron-vite'
import react from '@vitejs/plugin-react'

// NOTE: Dharma is a domain library package, not an Electron application.
// This config is maintained for development/testing purposes only.
// Consumers (Dhi, Vidhan) provide the actual Electron build configuration.

export default defineConfig({
  main: {
    // Dharma does not have a main process entry point
    resolve: {
      alias: {
        '@dharma': resolve('.')
      }
    }
  },

  preload: {
    resolve: {
      alias: {
        '@dharma': resolve('.')
      }
    }
  },

  renderer: {
    resolve: {
      alias: {
        '@dharma': resolve('.')
      }
    },
    plugins: [react()]
  }
})
