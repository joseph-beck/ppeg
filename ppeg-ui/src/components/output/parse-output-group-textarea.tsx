import { InputGroupTextarea } from '@shadcn/input-group'
import { ReactElement } from 'react'

const ParserOutputGroupTextarea = (): ReactElement => {
  return <InputGroupTextarea readOnly id="parser-output" placeholder="output" className="min-h-50" />
}

export { ParserOutputGroupTextarea }
