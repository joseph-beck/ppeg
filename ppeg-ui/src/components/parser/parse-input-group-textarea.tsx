import { InputGroupTextarea } from '@shadcn/input-group'
import { ReactElement } from 'react'

import { useParserForm } from './use-parser-form'

const ParserInputGroupTextarea = (): ReactElement => {
  const form = useParserForm()

  return (
    <form.Field
      name="input"
      children={(field) => (
        <InputGroupTextarea
          id="parse-input"
          className="min-h-50"
          value={String(field.state.value)}
          onBlur={field.handleBlur}
          onChange={(e) => field.handleChange(e.target.value)}
        />
      )}
    />
  )
}

export { ParserInputGroupTextarea }
