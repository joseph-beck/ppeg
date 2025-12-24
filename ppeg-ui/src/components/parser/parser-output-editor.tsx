import { useStore } from '@tanstack/react-store'
import { ReactElement } from 'react'

import { JsonEditor } from '../shared/json-editor'
import { parserOutputStore } from './parser-output-store'

const ParserOutputEditor = (): ReactElement => {
  const store = useStore(parserOutputStore)

  return <JsonEditor id="parser-output-editor" value={JSON.stringify(store.cst, null, 2)} readonly />
}

export { ParserOutputEditor }
