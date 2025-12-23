import { InputGroup, InputGroupAddon, InputGroupText } from '@shadcn/input-group'
import { ReactElement } from 'react'

import { ParserOutputCopy } from './parser-output-copy'
import { ParserOutputGroupTextarea } from './parser-output-group-textarea'

const ParserOutput = (): ReactElement => {
  return (
    <div className="w-full sm:max-w-4/5 md:max-w-2/3 lg:max-w-1/2">
      <InputGroup>
        <InputGroupAddon align="block-start" className="border-b">
          <InputGroupText className="font-mono font-medium">output</InputGroupText>
          <ParserOutputCopy />
        </InputGroupAddon>
        <ParserOutputGroupTextarea />
      </InputGroup>
    </div>
  )
}

export { ParserOutput }
