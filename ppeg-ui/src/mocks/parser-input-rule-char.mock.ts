import { Parse } from '@/types/ppeg/parse'

const parserInputRuleCharMock: Parse = {
  grammarObject: {
    rules: [
      {
        name: 'rule_char',
        expression: {
          type: 'Char',
          value: 'a',
        },
      },
    ],
  },
  grammarType: 'json',
  input: 'a',
  rule: 'rule_char',
}

export { parserInputRuleCharMock }
