import { describe, expect, it } from 'vitest'

import { CST } from '@/types/ppeg/cst'

import { isLeaf } from './is-leaf'

describe('isLeaf', () => {
  it('should be defined', () => {
    expect(isLeaf).toBeDefined()
  })

  it('should return true with a leaf node input', () => {
    const node: CST = {
      value: 'a',
      children: [],
    }

    expect(isLeaf(node)).toBe(true)
  })

  it('should return false with a non-leaf node input', () => {
    const node: CST = {
      value: 'a',
      children: [
        {
          value: 'b',
          children: [],
        },
      ],
    }

    expect(isLeaf(node)).toBe(false)
  })
})
