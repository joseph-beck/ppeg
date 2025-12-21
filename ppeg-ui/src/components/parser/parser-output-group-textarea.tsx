import { InputGroupTextarea } from '@shadcn/input-group'
import { useStore } from '@tanstack/react-store'
import { ReactElement } from 'react'

import { parserStore } from './parser-store'

const ParserOutputGroupTextarea = (): ReactElement => {
  const store = useStore(parserStore)

  return (
    <InputGroupTextarea
      readOnly
      id="parser-output"
      placeholder="output"
      className="min-h-50"
      value={JSON.stringify(store.cst)}
    />
  )
}

export { ParserOutputGroupTextarea }
