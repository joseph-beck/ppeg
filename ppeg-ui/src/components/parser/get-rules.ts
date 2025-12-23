import { grammar } from '@/types/ppeg/grammar'
import { Rule } from '@/types/ppeg/rule'

const getRules = (grammarString: string): Rule[] => {
  try {
    const grammarFromJSON = JSON.parse(grammarString)

    const grammarParsed = grammar.safeParse(grammarFromJSON)

    if (!grammarParsed.success) {
      return []
    }

    return grammarParsed.data.rules
  } catch {
    return []
  }
}

export { getRules }
