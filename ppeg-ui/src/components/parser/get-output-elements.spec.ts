import { describe, expect, it } from 'vitest'

import { getOutputElements } from './get-output-elements'

describe('getOutputElements', () => {
  it('should be defined', () => {
    expect(getOutputElements).toBeDefined()
  })

  it('should return an empty array when cst is undefined', () => {
    const result = getOutputElements(undefined)
    expect(result).toEqual({
      nodes: [],
      edges: [],
    })
  })
})
