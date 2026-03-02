import { grammar } from '@/types/ppeg/grammar'
import { grammarType } from '@/types/ppeg/grammar-type'
import { Parse } from '@/types/ppeg/parse'
import { Schema } from '@/types/ppeg/schema'

const parserSchemaTransform = (schema: Schema): Parse | undefined => {
  try {
    const grammarTypeParsed = grammarType.safeParse(schema.grammarType)
    if (!grammarTypeParsed.success) {
      console.error('error parsing grammar type', grammarTypeParsed.error)
      throw new Error(`error: ${grammarTypeParsed.error.message}`)
    }

    const baseParse = {
      input: schema.input,
      rule: schema.rule,
    }

    if (grammarTypeParsed.data === 'json') {
      const grammarFromJSON = JSON.parse(schema.grammar)

      const grammarParsed = grammar.safeParse(grammarFromJSON)
      if (!grammarParsed.success) {
        console.error('error parsing grammar', grammarParsed.error)
        throw new Error(`error: ${grammarParsed.error.message}`)
      }

      return {
        grammarObject: grammarParsed.data,
        grammarType: grammarTypeParsed.data,
        ...baseParse,
      }
    } else {
      return {
        grammarMeta: schema.grammar,
        grammarType: grammarTypeParsed.data,
        ...baseParse,
      }
    }
  } catch {
    return undefined
  }
}

export { parserSchemaTransform }
