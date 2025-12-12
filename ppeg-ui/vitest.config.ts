import { defineConfig } from 'vitest/config'

export default defineConfig({
  test: {
    exclude: ['src/**/*.gen.ts'],
    include: ['src/**/*.spec.ts'],
  },
})
