import * as z from 'zod'

import { expression } from './expression'

const rule = z.object({
  name: z.string(),
  expression: expression,
})

type Rule = z.infer<typeof rule>

export type { Rule }

export { rule }
