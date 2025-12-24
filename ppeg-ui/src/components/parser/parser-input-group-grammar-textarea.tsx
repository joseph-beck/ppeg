import { ReactElement } from 'react'

import { JsonEditor } from '../shared/json-editor'
import { useParserForm } from './use-parser-form'

interface ParserInputGroupGrammarTextareaProps {
  form: ReturnType<typeof useParserForm>
}

const ParserInputGroupGrammarTextarea = ({ form }: ParserInputGroupGrammarTextareaProps): ReactElement => {
  return (
    <form.Field
      name="grammar"
      children={(field) => (
        <JsonEditor
          id="parse-grammar"
          value={String(field.state.value)}
          onChange={(value) => field.handleChange(value)}
        />
      )}
    />
  )
}

export type { ParserInputGroupGrammarTextareaProps }

export { ParserInputGroupGrammarTextarea }
