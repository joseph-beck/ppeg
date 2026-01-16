import * as z from 'zod'

import { label } from './label'

const cst = z.object({
  value: z.string(),
  get children() {
    return z.array(cst)
  },
  label: label.optional(),
})

type CST = z.infer<typeof cst>

export type { CST }

export { cst }
