import { describe, expect, it, vi } from 'vitest'

import { useParseMutation } from './use-parse-mutation'

vi.mock('axios')

describe('useParseMutation', () => {
  it('should be defined', () => {
    expect(useParseMutation).toBeDefined()
  })
})
