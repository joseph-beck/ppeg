import { InputGroupTextarea } from '@shadcn/input-group'
import { useStore } from '@tanstack/react-store'
import { ReactElement } from 'react'

import { parserOutputStore } from './parser-output-store'

const ParserOutputGroupTextarea = (): ReactElement => {
  const store = useStore(parserOutputStore)

  return (
    <InputGroupTextarea
      readOnly
      id="parser-output"
      placeholder="output"
      className="min-h-50 p-3 font-mono"
      value={JSON.stringify(store.cst)}
    />
  )
}

export { ParserOutputGroupTextarea }
