import { describe, expect, it } from 'vitest'

import { defaultParserFormOpts } from './default-parser-form-opts'

describe('defaultParserFormOpts', () => {
  it('should be defined', () => {
    expect(defaultParserFormOpts).toBeDefined()
  })

  it('should have the correct default input value', () => {
    expect(defaultParserFormOpts.defaultValues.input).toBe('')
  })

  it('should have the correct default rule value', () => {
    expect(defaultParserFormOpts.defaultValues.rule).toBe('')
  })

  it('should have the correct default grammar value', () => {
    expect(defaultParserFormOpts.defaultValues.grammar).toBe('')
  })
})
