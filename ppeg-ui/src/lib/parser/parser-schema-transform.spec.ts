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
      grammarType: 'json',
    }

    const result = parserSchemaTransform(schema)

    expect(result).toEqual({
      grammarObject: { rules: [] },
      grammarType: 'json',
      input: 'some input',
      rule: 'start',
    })
  })

  it('should transform a ppeg schema correctly', () => {
    const schema = {
      grammar: `a := { 'a' }`,
      input: 'some input',
      rule: 'a',
      grammarType: 'ppeg',
    }

    const result = parserSchemaTransform(schema)

    expect(result).toEqual({
      grammarMeta: "a := { 'a' }",
      grammarType: 'ppeg',
      input: 'some input',
      rule: 'a',
    })
  })

  it('should throw an error for invalid grammar JSON', () => {
    const schema = {
      grammar: 'invalid json',
      input: 'some input',
      rule: 'start',
      grammarType: 'json',
    }

    expect(parserSchemaTransform(schema)).toBeUndefined()
  })

  it('should throw an error for mismatch grammar JSON', () => {
    const schema = {
      grammar: '{"invalidKey":123}',
      input: 'some input',
      rule: 'start',
      grammarType: 'json',
    }

    expect(parserSchemaTransform(schema)).toBeUndefined()
  })

  it('should throw an error for invalid grammar grammarType', () => {
    const schema = {
      grammar: '{"rules":[]}',
      input: 'some input',
      rule: 'start',
      grammarType: 'other',
    }

    expect(parserSchemaTransform(schema)).toBeUndefined()
  })
})
