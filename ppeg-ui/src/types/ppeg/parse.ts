import * as z from 'zod'

const parse = z.object({
  input: z.string().default(''),
  rule: z.string().optional(),
  grammar: z.string().optional(),
})

type Parse = z.infer<typeof parse>

export type { Parse }

export { parse }
