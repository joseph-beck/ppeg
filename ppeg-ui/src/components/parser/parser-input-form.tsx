import { InputGroup, InputGroupAddon, InputGroupText } from '@shadcn/input-group'
import { ReactElement } from 'react'

import { ParserInputCopy } from './parser-input-copy'
import { ParserInputGroupGrammarTextarea } from './parser-input-group-grammar-textarea'
import { ParserInputGroupInputTextarea } from './parser-input-group-input-textarea'
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
      >
        <InputGroup>
          <InputGroupAddon align="block-start" className="border-b">
            <InputGroupText className="font-mono font-medium">editor</InputGroupText>
            <ParserInputCopy form={form} />
          </InputGroupAddon>
          <ParserInputGroupGrammarTextarea form={form} />
          <InputGroupAddon align="block-end" className="border-t">
            <ParserInputValidation />
            <ParserInputRuleSelect form={form} />
          </InputGroupAddon>
          <InputGroupAddon align="block-end" className="border-t">
            <ParserInputGroupInputTextarea form={form} />
          </InputGroupAddon>
        </InputGroup>
      </form>
    </div>
  )
}

export type { ParserInputFormProps }

export { ParserInputForm }
