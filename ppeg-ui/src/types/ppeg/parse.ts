import * as z from 'zod'

import { grammar } from './grammar'

const parseBase = z.object({
  input: z.string().default(''),
  rule: z.string().optional(),
})

const parse = z.discriminatedUnion('grammar_type', [
  parseBase.extend({
    grammarType: z.literal('ppeg'),
    grammarMeta: z.string(),
  }),
  parseBase.extend({
    grammarType: z.literal('json'),
    grammarObject: grammar,
  }),
])

type Parse = z.infer<typeof parse>

export type { Parse }

export { parse }
