import { ReactElement } from 'react'

import { JsonEditor } from '../shared/json-editor'
import { useParserForm } from './use-parser-form'

interface ParserGrammarEditorProps {
  form: ReturnType<typeof useParserForm>
}

const ParserGrammarEditor = ({ form }: ParserGrammarEditorProps): ReactElement => {
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

export type { ParserGrammarEditorProps }

export { ParserGrammarEditor }
