import { grammar } from '@/types/ppeg/grammar'
import { Parse } from '@/types/ppeg/parse'
import { Schema } from '@/types/ppeg/schema'

const parserSchemaTransform = (schema: Schema): Parse => {
  const grammarParsed = grammar.safeParse(JSON.parse(schema.grammar))
  if (!grammarParsed.success) {
    throw new Error(`error: ${grammarParsed.error.message}`)
  }

  return {
    grammar: grammarParsed.data,
    input: schema.input,
    rule: schema.rule,
  }
}

export { parserSchemaTransform }
