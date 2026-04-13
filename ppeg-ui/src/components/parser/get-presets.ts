interface PresetInput {
  grammar: string
  input: string
  rule: string
}

interface GetPresetsResult {
  label: string
  description?: string
  parse: PresetInput
}

const getPresets = (): GetPresetsResult[] => {
  return [
    {
      label: 'character',
      description: 'grammar for parsing the character "a".',
      parse: {
        grammar: ["rule_char := { 'a' }", ''].join('\n'),
        input: 'a',
        rule: 'rule_char',
      },
    },
    {
      label: 'number',
      description: 'grammar for parsing numbers, such as "123".',
      parse: {
        grammar: ["rule_number := { { '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' }+ }", ''].join('\n'),
        input: '123',
        rule: 'rule_number',
      },
    },
    {
      label: 'word',
      description: 'grammar for parsing a word.',
      parse: {
        grammar: [
          "rule_word := { { 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' }+ }",
          '',
        ].join('\n'),
        input: 'hello',
        rule: 'rule_word',
      },
    },
    {
      label: 'arithmetic expression',
      description: 'grammar for parsing arithmetic expressions, such as "1+2".',
      parse: {
        grammar: ["rule_num := { { '1' | '2' }+ }", "rule_expr := { rule_expr, '+', rule_num | rule_num }", ''].join(
          '\n',
        ),
        input: '1+2',
        rule: 'rule_expr',
      },
    },
    {
      label: 'arithmetic expression with left recursion',
      description: 'grammar for parsing arithmetic expressions, such as "1+2+3", using left recursion.',
      parse: {
        grammar: [
          "n := { { '1' | '2' | '3' }+ }",
          "ae := { { ae, '+', ae } | { ae, '*', ae } | n | { '(', ae, ')' } }",
          '',
        ].join('\n'),
        input: '1+2+3',
        rule: 'ae',
      },
    },
    {
      label: 'direct left recursion',
      description: 'grammar demonstrating direct left recursion.',
      parse: {
        grammar: ["n := { { '1' | '2' | '3' }+ }", "expr := { expr, '+', expr | n }", ''].join('\n'),
        input: '1+2+3',
        rule: 'expr',
      },
    },
    {
      label: 'indirect left recursion',
      description: 'grammar demonstrating indirect left recursion.',
      parse: {
        grammar: ["n := { { '1' | '2' | '3' }+ }", 'x := { rule_expr }', "expr := { expr, '+', expr | n }", ''].join(
          '\n',
        ),
        input: '1+2+3',
        rule: 'expr',
      },
    },
    {
      label: 'structured data',
      description: 'grammar for a minimal structured data format.',
      parse: {
        grammar: [
          "open := { '{' }",
          "close := { '}' }",
          "char := { 'a' | 'b' }",
          'string := { char+ }',
          'key := { string }',
          `value := { '"', string, '"' }`,
          "object := { key, ':', value }",
          "objects := { { objects, ',', objects } | object? }",
          'root := { open, objects, close }',
          '',
        ].join('\n'),
        input: '{a:"b"}',
        rule: 'root',
      },
    },
  ]
}

export type { GetPresetsResult, PresetInput }

export { getPresets }
