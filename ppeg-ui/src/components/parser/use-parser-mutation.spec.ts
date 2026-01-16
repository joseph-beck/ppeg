import { describe, expect, it, vi } from 'vitest'

import { useParserMutation } from './use-parser-mutation'

vi.mock('axios')

describe('useParseMutation', () => {
  it('should be defined', () => {
    expect(useParserMutation).toBeDefined()
  })
})
