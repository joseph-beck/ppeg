import { Parse } from '@/types/ppeg/parse'

const parserInputRuleCharMock: Parse = {
  grammar: {
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
  input: 'a',
  rule: 'rule_char',
}

export { parserInputRuleCharMock }
