import { ReactElement } from 'react'

import { TextEditor } from '../shared/text-editor'
import { useParserForm } from './use-parser-form'

interface ParserInputEditorProps {
  form: ReturnType<typeof useParserForm>
}

const ParserInputEditor = ({ form }: ParserInputEditorProps): ReactElement => {
  return (
    <form.Field
      name="input"
      children={(field) => (
        <TextEditor
          id="parser-input"
          value={String(field.state.value)}
          onChange={field.handleChange}
          miniumHeightPx={100}
          heightPercent={0}
        />
      )}
    />
  )
}

export type { ParserInputEditorProps }

export { ParserInputEditor }
