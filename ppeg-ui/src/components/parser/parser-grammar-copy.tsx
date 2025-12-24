import { useStore } from '@tanstack/react-form'
import { ReactElement } from 'react'

import { CopyGroup } from '../shared/copy-group'
import { useParserForm } from './use-parser-form'

interface ParserGrammarCopyProps {
  form: ReturnType<typeof useParserForm>
}

const ParserGrammarCopy = ({ form }: ParserGrammarCopyProps): ReactElement => {
  const input = useStore(form.store, (state) => state.values.grammar)

  return <CopyGroup input={input} />
}

export type { ParserGrammarCopyProps }

export { ParserGrammarCopy }
