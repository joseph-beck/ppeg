import { Parse } from '@/types/ppeg/parse'

const parserInputRuleWordMock: Parse = {
  grammar: {
    rules: [
      {
        name: 'rule_word',
        expression: {
          type: 'OneOrMore',
          value: {
            type: 'NamedRule',
            value: 'alpha',
          },
        },
      },
      {
        name: 'alpha',
        expression: {
          type: 'Choice',
          value: [
            {
              type: 'Char',
              value: 'a',
            },
            {
              type: 'Char',
              value: 'b',
            },
          ],
        },
      },
    ],
  },
  input: 'abba',
  rule: 'rule_word',
}

export { parserInputRuleWordMock }
