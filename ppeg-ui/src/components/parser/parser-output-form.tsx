import { InputGroup, InputGroupAddon, InputGroupText } from '@shadcn/input-group'
import { ReactElement } from 'react'

import { ParserOutputCopy } from './parser-output-copy'
import { ParserOutputEditor } from './parser-output-editor'

const ParserOutput = (): ReactElement => {
  return (
    <div className="w-full sm:max-w-4/5 md:max-w-2/3 lg:max-w-1/2">
      <InputGroup>
        <InputGroupAddon align="block-start" className="border-b">
          <InputGroupText className="font-mono font-medium">output</InputGroupText>
          <ParserOutputCopy />
        </InputGroupAddon>
        <ParserOutputEditor />
        <InputGroupAddon align="block-end" className="border-t">
          <InputGroupText className="font-mono font-medium"></InputGroupText>
        </InputGroupAddon>
      </InputGroup>
    </div>
  )
}

export { ParserOutput }
