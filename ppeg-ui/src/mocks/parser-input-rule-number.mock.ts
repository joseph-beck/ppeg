import { Parse } from '@/types/ppeg/parse'

const parserInputRuleNumberMock: Parse = {
  grammarObject: {
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
            {
              type: 'Char',
              value: '0',
            },
            {
              type: 'Char',
              value: '1',
            },
            {
              type: 'Char',
              value: '2',
            },
            {
              type: 'Char',
              value: '3',
            },
            {
              type: 'Char',
              value: '4',
            },
            {
              type: 'Char',
              value: '5',
            },
            {
              type: 'Char',
              value: '6',
            },
            {
              type: 'Char',
              value: '7',
            },
            {
              type: 'Char',
              value: '8',
            },
            {
              type: 'Char',
              value: '9',
            },
          ],
        },
      },
    ],
  },
  grammarType: 'json',
  input: '4132',
  rule: 'rule_number',
}

export { parserInputRuleNumberMock }
