import * as z from 'zod'

const label = z.object({
  hidden: z.boolean(),
})

type Label = z.infer<typeof label>

export type { Label as LabelType }

export { label }
