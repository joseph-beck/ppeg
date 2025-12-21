import { InputGroupTextarea } from '@shadcn/input-group'
import { ReactElement } from 'react'

import { useParserForm } from './use-parser-form'

interface ParserInputGroupGrammarTextareaProps {
  form: ReturnType<typeof useParserForm>
}

const ParserInputGroupGrammarTextarea = ({ form }: ParserInputGroupGrammarTextareaProps): ReactElement => {
  return (
    <form.Field
      name="grammar"
      children={(field) => (
        <InputGroupTextarea
          id="parse-grammar"
          className="min-h-50 p-3 font-mono"
          placeholder="input"
          value={String(field.state.value)}
          onBlur={field.handleBlur}
          onChange={(e) => field.handleChange(e.target.value)}
        />
      )}
    />
  )
}

export type { ParserInputGroupGrammarTextareaProps }

export { ParserInputGroupGrammarTextarea }
