import { describe, expect, it } from 'vitest'

import { parserSchemaTransform } from './parser-schema-transform'

describe('parserSchemaTransform', () => {
  it('should be defined', () => {
    expect(parserSchemaTransform).toBeDefined()
  })

  it('should transform a valid schema correctly', () => {
    const schema = {
      grammar: '{"rules":[]}',
      input: 'some input',
      rule: 'start',
    }

    const result = parserSchemaTransform(schema)

    expect(result).toEqual({
      grammar: { rules: [] },
      input: 'some input',
      rule: 'start',
    })
  })

  it('should throw an error for invalid grammar JSON', () => {
    const schema = {
      grammar: 'invalid json',
      input: 'some input',
      rule: 'start',
    }

    expect(parserSchemaTransform(schema)).toBeUndefined()
  })

  it('should throw an error for mismatch grammar JSON', () => {
    const schema = {
      grammar: '{"invalidKey":123}',
      input: 'some input',
      rule: 'start',
    }

    expect(parserSchemaTransform(schema)).toBeUndefined()
  })
})
