import * as z from 'zod'

const empty = z.object({
  type: z.literal('Empty'),
})

const char = z.object({
  type: z.literal('Char'),
  value: z.string(),
})

const sequence = z.object({
  type: z.literal('Sequence'),
  expressions: z.lazy(() => z.array(expression)),
})

const choice = z.object({
  type: z.literal('Choice'),
  expressions: z.lazy(() => z.array(expression)),
})

const zeroOrMore = z.object({
  type: z.literal('ZeroOrMore'),
  expression: z.lazy(() => expression),
})

const oneOrMore = z.object({
  type: z.literal('OneOrMore'),
  expression: z.lazy(() => expression),
})

const namedRule = z.object({
  type: z.literal('NamedRule'),
  name: z.string(),
})

const expression: z.ZodType<Expression> = z.discriminatedUnion('type', [
  empty,
  char,
  sequence,
  choice,
  zeroOrMore,
  oneOrMore,
  namedRule,
])

type Expression =
  | z.infer<typeof empty>
  | z.infer<typeof char>
  | { type: 'Sequence'; expressions: Expression[] }
  | { type: 'Choice'; expressions: Expression[] }
  | { type: 'ZeroOrMore'; expression: Expression }
  | { type: 'OneOrMore'; expression: Expression }
  | z.infer<typeof namedRule>

type Empty = z.infer<typeof empty>
type Char = z.infer<typeof char>
type Sequence = z.infer<typeof sequence>
type Choice = z.infer<typeof choice>
type ZeroOrMore = z.infer<typeof zeroOrMore>
type OneOrMore = z.infer<typeof oneOrMore>
type NamedRule = z.infer<typeof namedRule>

export type { Char, Choice, Empty, Expression, NamedRule, OneOrMore, Sequence, ZeroOrMore }

export { char, choice, empty, expression, namedRule, oneOrMore, sequence, zeroOrMore }
