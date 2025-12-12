import { Grammar } from './grammar'

interface Parse {
  grammar: Grammar
  input: string
  rule: string
}

export type { Parse }
