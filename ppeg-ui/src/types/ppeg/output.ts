import * as z from 'zod'

import { cst } from './cst'

const output = z.object({
  remaining: z.string(),
  cst: cst.optional(),
})

type Output = z.infer<typeof output>

export type { Output }

export { output }
