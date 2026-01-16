import { describe, expect, it } from 'vitest'

import { isRoot } from './is-root'

describe('isRoot', () => {
  it('should be defined', () => {
    expect(isRoot).toBeDefined()
  })

  it('should return true with an undefined parentId', () => {
    expect(isRoot(undefined)).toBe(true)
  })

  it('should return false with a defined parentId', () => {
    expect(isRoot('0')).toBe(false)
  })
})
