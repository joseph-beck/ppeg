import { Label } from './label'

interface CST {
  value: string
  children: CST[]
  label?: Label
}

export type { CST }
