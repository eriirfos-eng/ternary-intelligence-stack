import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  server: {
    fs: {
      // Security override: Allow Vite to reach outside its root into the monorepo
      allow: ['..']
    }
  },
  resolve: {
    alias: {
      // Map the absolute path used in studio.js to the actual sibling folder
      '/playground': '../playground'
    }
  }
});
