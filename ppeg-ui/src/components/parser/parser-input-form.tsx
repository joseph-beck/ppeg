import { InputGroup, InputGroupAddon, InputGroupText } from '@shadcn/input-group'
import { ReactElement } from 'react'

import { ParserOutputGroupTextarea } from '../output/parse-output-group-textarea'
import { ParserInputGroupTextarea } from './parse-input-group-textarea'
import { ParserInputRuleSelect } from './parse-input-rule-select'
import { ParserInputCopy } from './parser-input-copy'
import { ParserInputValidation } from './parser-input-validation'

interface ParserInputFormProps {
  _?: never
}

const ParserInputForm = (props: ParserInputFormProps): ReactElement => {
  void props

  return (
    <div className="w-full max-w-md mb-6">
      <InputGroup>
        <InputGroupAddon align="block-start" className="border-b">
          <InputGroupText className="font-mono font-medium">editor</InputGroupText>
          <ParserInputCopy />
        </InputGroupAddon>
        <ParserInputGroupTextarea />
        <InputGroupAddon align="block-end" className="border-t">
          <ParserInputValidation />
          <ParserInputRuleSelect />
        </InputGroupAddon>
        <InputGroupAddon align="block-end" className="border-t">
          <ParserOutputGroupTextarea />
        </InputGroupAddon>
      </InputGroup>
    </div>
  )
}

export type { ParserInputFormProps }

export { ParserInputForm }
