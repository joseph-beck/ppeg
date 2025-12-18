import * as z from 'zod'

import { grammar } from './grammar'

const parse = z.object({
  grammar: grammar,
  input: z.string(),
  rule: z.string(),
})

type Parse = z.infer<typeof parse>

export type { Parse }

export { parse }
