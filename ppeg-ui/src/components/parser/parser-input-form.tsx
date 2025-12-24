import { InputGroup, InputGroupAddon, InputGroupText } from '@shadcn/input-group'
import { ReactElement } from 'react'

import { ParserGrammarCopy } from './parser-grammar-copy'
import { ParserGrammarEditor } from './parser-grammar-editor'
import { ParserInputCopy } from './parser-input-copy'
import { ParserInputEditor } from './parser-input-editor'
import { ParserInputRuleSelect } from './parser-input-rule-select'
import { ParserInputValidation } from './parser-input-validation'
import { useParserForm } from './use-parser-form'

interface ParserInputFormProps {
  nil?: never
}

const ParserInputForm = ({ nil: _nil }: ParserInputFormProps): ReactElement => {
  const form = useParserForm()

  return (
    <div className="sm:min-w-4/5 md:min-w-2/3 lg:min-w-1/2 mb-12">
      <form
        onSubmit={(e) => {
          e.preventDefault()
          e.stopPropagation()
        }}
        className=""
      >
        <InputGroup className="mb-4">
          <InputGroupAddon align="block-start" className="border-b">
            <InputGroupText className="font-mono font-medium">editor</InputGroupText>
            <ParserGrammarCopy form={form} />
          </InputGroupAddon>
          <ParserGrammarEditor form={form} />
          <InputGroupAddon align="block-end" className="border-t">
            <ParserInputValidation />
            <ParserInputRuleSelect form={form} />
          </InputGroupAddon>
        </InputGroup>
        <InputGroup>
          <InputGroupAddon align="block-start" className="border-b">
            <InputGroupText className="font-mono font-medium">input</InputGroupText>
            <ParserInputCopy form={form} />
          </InputGroupAddon>
          <ParserInputEditor form={form} />
          <InputGroupAddon align="block-end" className="border-t">
            <InputGroupText className="font-mono font-medium">input</InputGroupText>
          </InputGroupAddon>
        </InputGroup>
      </form>
    </div>
  )
}

export type { ParserInputFormProps }

export { ParserInputForm }
