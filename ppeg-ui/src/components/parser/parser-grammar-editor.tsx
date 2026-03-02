import { ReactElement } from 'react'

import { JsonEditor } from '../shared/json-editor'
import { TextEditor } from '../shared/text-editor'
import { useParserForm } from './use-parser-form'

interface ParserGrammarEditorProps {
  form: ReturnType<typeof useParserForm>
}

const ParserGrammarEditor = ({ form }: ParserGrammarEditorProps): ReactElement => {
  const type = form.getFieldValue('grammarType')

  return (
    <form.Field
      name="grammar"
      children={(field) => (
        <>
          {type === 'json' ? (
            <JsonEditor
              id="parse-grammar"
              value={String(field.state.value)}
              onChange={(value) => field.handleChange(value)}
            />
          ) : (
            <TextEditor
              id="parse-grammar"
              value={String(field.state.value)}
              onChange={(value) => field.handleChange(value)}
            />
          )}
        </>
      )}
    />
  )
}

export type { ParserGrammarEditorProps }

export { ParserGrammarEditor }
