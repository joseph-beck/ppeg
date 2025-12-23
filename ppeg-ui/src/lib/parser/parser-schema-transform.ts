import { grammar } from '@/types/ppeg/grammar'
import { Parse } from '@/types/ppeg/parse'
import { Schema } from '@/types/ppeg/schema'

const parserSchemaTransform = (schema: Schema): Parse | undefined => {
  try {
    const grammarFromJSON = JSON.parse(schema.grammar)

    const grammarParsed = grammar.safeParse(grammarFromJSON)
    if (!grammarParsed.success) {
      throw new Error(`error: ${grammarParsed.error.message}`)
    }

    return {
      grammar: grammarParsed.data,
      input: schema.input,
      rule: schema.rule,
    }
  } catch {
    return undefined
  }
}

export { parserSchemaTransform }
