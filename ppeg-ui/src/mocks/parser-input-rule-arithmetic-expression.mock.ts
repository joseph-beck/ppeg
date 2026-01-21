import { Parse } from '@/types/ppeg/parse'

const parserInputRuleArithmeticExpressionMock: Parse = {
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
        name: 'rule_factor',
        expression: {
          type: 'Choice',
          value: [
            {
              type: 'NamedRule',
              value: 'rule_number',
            },
            {
              type: 'Sequence',
              value: [
                {
                  type: 'Char',
                  value: '(',
                },
                {
                  type: 'NamedRule',
                  value: 'rule_arithmetic_expression',
                },
                {
                  type: 'Char',
                  value: ')',
                },
              ],
            },
          ],
        },
      },
      {
        name: 'rule_term',
        expression: {
          type: 'Sequence',
          value: [
            {
              type: 'NamedRule',
              value: 'rule_factor',
            },
            {
              type: 'ZeroOrMore',
              value: {
                type: 'Sequence',
                value: [
                  {
                    type: 'Char',
                    value: '*',
                  },
                  {
                    type: 'NamedRule',
                    value: 'rule_factor',
                  },
                ],
              },
            },
          ],
        },
      },
      {
        name: 'rule_arithmetic_expression',
        expression: {
          type: 'Sequence',
          value: [
            {
              type: 'NamedRule',
              value: 'rule_term',
            },
            {
              type: 'ZeroOrMore',
              value: {
                type: 'Sequence',
                value: [
                  {
                    type: 'Char',
                    value: '+',
                  },
                  {
                    type: 'NamedRule',
                    value: 'rule_term',
                  },
                ],
              },
            },
          ],
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
  input: '1+(2*3)',
  rule: 'rule_arithmetic_expression',
}

export { parserInputRuleArithmeticExpressionMock }
