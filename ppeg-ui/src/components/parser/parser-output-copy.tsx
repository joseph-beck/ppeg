import { useStore } from '@tanstack/react-store'
import { ReactElement } from 'react'

import { CopyGroup } from '../shared/copy-group'
import { parserOutputStore } from './parser-output-store'

interface ParserOutputCopyProps {
  nil?: never
}

const ParserOutputCopy = ({ nil: _nil }: ParserOutputCopyProps): ReactElement => {
  const store = useStore(parserOutputStore, (state) => state)

  return <CopyGroup input={JSON.stringify(store.cst, null, 2)} />
}

export type { ParserOutputCopyProps }

export { ParserOutputCopy }
