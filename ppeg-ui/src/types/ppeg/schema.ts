import * as z from 'zod'

// Similar to parse but used as the form input schema.
// Grammar is stored as a string here, in the form of JSON.
// Grammar is later parsed into the Grammar type in the parserSchemaTransform function.
const schema = z.object({
  grammar: z.string(),
  input: z.string(),
  rule: z.string(),
})

type Schema = z.infer<typeof schema>

export type { Schema }

export { schema }
