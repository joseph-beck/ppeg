import { describe, expect, it } from 'vitest'

import { isNonEmptyArray } from './is-non-empty-array'

describe('isNonEmptyArray', () => {
  it('should be defined', () => {
    expect(isNonEmptyArray).toBeDefined()
  })

  it('should return true for non-empty arrays', () => {
    expect(isNonEmptyArray([1, 2, 3])).toBe(true)
    expect(isNonEmptyArray(['a', 'b'])).toBe(true)
    expect(isNonEmptyArray([{}])).toBe(true)
  })

  it('should return false for empty arrays', () => {
    expect(isNonEmptyArray([])).toBe(false)
  })

  it('should return false for non-array values', () => {
    expect(isNonEmptyArray(null)).toBe(false)
    expect(isNonEmptyArray(undefined)).toBe(false)
    expect(isNonEmptyArray(42)).toBe(false)
    expect(isNonEmptyArray('hello')).toBe(false)
    expect(isNonEmptyArray({})).toBe(false)
  })
})
