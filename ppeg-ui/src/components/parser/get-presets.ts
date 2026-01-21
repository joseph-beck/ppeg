import { parserInputRuleArithmeticExpressionMock } from '@/mocks/parser-input-rule-arithmetic-expression.mock'
import { parserInputRuleCharMock } from '@/mocks/parser-input-rule-char.mock'
import { parserInputRuleNumberMock } from '@/mocks/parser-input-rule-number.mock'
import { parserInputRuleWordMock } from '@/mocks/parser-input-rule-word.mock'
import { Parse } from '@/types/ppeg/parse'

interface GetPresetsResult {
  label: string
  description?: string
  parse: Parse
}

const getPresets = (): GetPresetsResult[] => {
  return [
    {
      label: 'character rule',
      description: 'grammar for parsing the character "a"',
      parse: parserInputRuleCharMock,
    },
    {
      label: 'number rule',
      description: 'grammar for parsing numbers, such as "123"',
      parse: parserInputRuleNumberMock,
    },
    {
      label: 'word rule',
      description: 'grammar for parsing a word',
      parse: parserInputRuleWordMock,
    },
    {
      label: 'arithmetic expression rule',
      description: 'grammar for parsing arithmetic expressions, such as "1+(2*3)"',
      parse: parserInputRuleArithmeticExpressionMock,
    },
  ]
}

export type { GetPresetsResult }

export { getPresets }
