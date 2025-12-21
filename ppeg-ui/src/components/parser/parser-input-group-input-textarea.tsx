import { InputGroupTextarea } from '@shadcn/input-group'
import { ReactElement } from 'react'

import { useParserForm } from './use-parser-form'

interface ParserInputGroupInputTextareaProps {
  form: ReturnType<typeof useParserForm>
}

const ParserInputGroupInputTextarea = ({ form }: ParserInputGroupInputTextareaProps): ReactElement => {
  return (
    <form.Field
      name="input"
      children={(field) => (
        <InputGroupTextarea
          id="parse-input"
          className="min-h-50 p-3 font-mono"
          placeholder="input"
          value={String(field.state.value)}
          onBlur={field.handleBlur}
          onChange={(e) => {
            field.handleChange(e.target.value)
          }}
        />
      )}
    />
  )
}

export type { ParserInputGroupInputTextareaProps }

export { ParserInputGroupInputTextarea }
