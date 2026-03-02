import { parserInputRuleArithmeticExpressionMock } from '@/mocks/parser-input-rule-arithmetic-expression.mock'
import { parserInputRuleCharMock } from '@/mocks/parser-input-rule-char.mock'
import { parserInputRuleDirectLeftRecursion } from '@/mocks/parser-input-rule-direct-left-recursion'
import { parserInputRuleIndirectLeftRecursion } from '@/mocks/parser-input-rule-indirect-left-recursion'
import { parserInputRuleNumberMock } from '@/mocks/parser-input-rule-number.mock'
import { parserInputRuleWordMock } from '@/mocks/parser-input-rule-word.mock'
import { GrammarType } from '@/types/ppeg/grammar-type'
import { Parse } from '@/types/ppeg/parse'

interface GetPresetsResult {
  label: string
  description?: string
  parse: Parse
  grammarType: GrammarType
}

const getPresets = (): GetPresetsResult[] => {
  return [
    {
      label: 'character rule',
      description: 'grammar for parsing the character "a"',
      parse: parserInputRuleCharMock,
      grammarType: 'json',
    },
    {
      label: 'number rule',
      description: 'grammar for parsing numbers, such as "123"',
      parse: parserInputRuleNumberMock,
      grammarType: 'json',
    },
    {
      label: 'word rule',
      description: 'grammar for parsing a word',
      parse: parserInputRuleWordMock,
      grammarType: 'json',
    },
    {
      label: 'arithmetic expression rule',
      description: 'grammar for parsing arithmetic expressions, such as "1+(2*3)"',
      parse: parserInputRuleArithmeticExpressionMock,
      grammarType: 'json',
    },
    {
      label: 'indirect left recursion rule',
      description: 'grammar demonstrating indirect left recursion',
      parse: parserInputRuleDirectLeftRecursion,
      grammarType: 'json',
    },
    {
      label: 'direct left recursion rule',
      description: 'grammar demonstrating direct left recursion',
      parse: parserInputRuleIndirectLeftRecursion,
      grammarType: 'json',
    },
  ]
}

export type { GetPresetsResult }

export { getPresets }
