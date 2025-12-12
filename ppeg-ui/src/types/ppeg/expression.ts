interface Empty {
  type: 'Empty'
}

interface Char {
  type: 'Char'
  value: string
}

interface Sequence {
  type: 'Sequence'
  expressions: Expression[]
}

interface Choice {
  type: 'Choice'
  expressions: Expression[]
}

interface ZeroOrMore {
  type: 'ZeroOrMore'
  expression: Expression
}

interface OneOrMore {
  type: 'OneOrMore'
  expression: Expression
}

interface NamedRule {
  type: 'NamedRule'
  name: string
}

type Expression = Empty | Char | Sequence | Choice | ZeroOrMore | OneOrMore | NamedRule

export type { Char, Choice, Empty, Expression, NamedRule, OneOrMore, Sequence, ZeroOrMore }
