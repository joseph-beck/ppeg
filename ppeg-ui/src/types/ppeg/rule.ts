import * as z from 'zod'

import { expression } from './expression'

const rule = z.object({
  name: z.string(),
  expression: expression,
})

type Rule = z.infer<typeof rule>

const rules = z.array(rule)

type Rules = z.infer<typeof rules>

const ruleNames = z.array(z.string())

type RuleNames = z.infer<typeof ruleNames>

export type { Rule, RuleNames,Rules }

export { rule, ruleNames,rules }
