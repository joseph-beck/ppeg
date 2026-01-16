import { Parse } from '@/types/ppeg/parse'

const parserInputRuleNumberMock: Parse = {
  grammar: {
    rules: [
      {
        name: 'rule_number',
        expression: {
          type: 'OneOrMore',
          value: {
            type: 'NamedRule',
            value: 'digit',
          },
        },
      },
      {
        name: 'digit',
        expression: {
          type: 'Choice',
          value: [
            { type: 'Char', value: '1' },
            { type: 'Char', value: '2' },
            { type: 'Char', value: '3' },
            { type: 'Char', value: '4' },
          ],
        },
      },
    ],
  },
  input: '4132',
  rule: 'rule_number',
}

export { parserInputRuleNumberMock }
