import { parserInputRuleCharMock } from '@/mocks/parser-input-rule-char-mock'
import { parserInputRuleNumberMock } from '@/mocks/parser-input-rule-number-mock'
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
  ]
}

export type { GetPresetsResult }

export { getPresets }
