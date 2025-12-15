import * as z from 'zod'

import { rule } from './rule'

const grammar = z.object({
  rules: z.array(rule),
})

type Grammar = z.infer<typeof grammar>

export type { Grammar }

export { grammar }
