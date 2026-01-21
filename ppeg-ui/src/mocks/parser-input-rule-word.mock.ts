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
            {
              type: 'Char',
              value: 'c',
            },
            {
              type: 'Char',
              value: 'd',
            },
            {
              type: 'Char',
              value: 'e',
            },
            {
              type: 'Char',
              value: 'f',
            },
            {
              type: 'Char',
              value: 'g',
            },
            {
              type: 'Char',
              value: 'h',
            },
            {
              type: 'Char',
              value: 'i',
            },
            {
              type: 'Char',
              value: 'j',
            },
            {
              type: 'Char',
              value: 'k',
            },
            {
              type: 'Char',
              value: 'l',
            },
            {
              type: 'Char',
              value: 'm',
            },
            {
              type: 'Char',
              value: 'n',
            },
            {
              type: 'Char',
              value: 'o',
            },
            {
              type: 'Char',
              value: 'p',
            },
            {
              type: 'Char',
              value: 'q',
            },
            {
              type: 'Char',
              value: 'r',
            },
            {
              type: 'Char',
              value: 's',
            },
            {
              type: 'Char',
              value: 't',
            },
            {
              type: 'Char',
              value: 'u',
            },
            {
              type: 'Char',
              value: 'v',
            },
            {
              type: 'Char',
              value: 'w',
            },
            {
              type: 'Char',
              value: 'x',
            },
            {
              type: 'Char',
              value: 'y',
            },
            {
              type: 'Char',
              value: 'z',
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
