import * as z from 'zod'

const grammarType = z.literal(['ppeg', 'json'])

type GrammarType = z.infer<typeof grammarType>

export type { GrammarType }

export { grammarType }
