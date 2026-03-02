import { Parse } from '@/types/ppeg/parse'

const parserInputRuleDirectLeftRecursion: Parse = {
  grammarObject: {
    rules: [
      {
        name: 'rule_number',
        expression: {
          type: 'Choice',
          value: [
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
          ],
        },
      },
      {
        name: 'rule_x',
        expression: {
          type: 'NamedRule',
          value: 'rule_expr',
        },
      },
      {
        name: 'rule_expr',
        expression: {
          type: 'Choice',
          value: [
            {
              type: 'Sequence',
              value: [
                {
                  type: 'NamedRule',
                  value: 'rule_x',
                },
                {
                  type: 'Char',
                  value: '+',
                },
                {
                  type: 'NamedRule',
                  value: 'rule_number',
                },
              ],
            },
            {
              type: 'NamedRule',
              value: 'rule_number',
            },
          ],
        },
      },
    ],
  },
  grammarType: 'json',
  input: '1+1',
  rule: 'rule_expr',
}

export { parserInputRuleDirectLeftRecursion }
