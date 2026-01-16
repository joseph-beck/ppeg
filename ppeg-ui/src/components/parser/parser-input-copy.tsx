import { useStore } from '@tanstack/react-form'
import { ReactElement } from 'react'

import { CopyGroup } from '../shared/copy-group'
import { useParserForm } from './use-parser-form'

interface ParserInputCopyProps {
  form: ReturnType<typeof useParserForm>
}

const ParserInputCopy = ({ form }: ParserInputCopyProps): ReactElement => {
  const input = useStore(form.store, (state) => state.values.input)

  return <CopyGroup input={input} />
}

export type { ParserInputCopyProps }

export { ParserInputCopy }
