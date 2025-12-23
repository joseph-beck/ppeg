import { InputGroupButton } from '@shadcn/input-group'
import { Tooltip, TooltipContent, TooltipTrigger } from '@shadcn/tooltip'
import { P } from '@shadcn/typography'
import { IconCheck, IconCopy } from '@tabler/icons-react'
import { useStore } from '@tanstack/react-form'
import { ReactElement, useState } from 'react'

import { parserOutputStore } from './parser-output-store'

interface ParserOutputCopyProps {
  nil?: never
}

const ParserOutputCopy = ({ nil: _nil }: ParserOutputCopyProps): ReactElement => {
  const store = useStore(parserOutputStore)

  const [copied, setCopied] = useState<boolean>(false)

  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <InputGroupButton
          className="ml-auto"
          size="icon-xs"
          onClick={() => {
            navigator.clipboard.writeText(JSON.stringify(store.cst))

            setCopied(true)

            setTimeout(() => {
              setCopied(false)
            }, 1000)
          }}
        >
          {copied ? <IconCheck /> : <IconCopy />}
        </InputGroupButton>
      </TooltipTrigger>
      <TooltipContent>
        <P>Copy to Clipboard</P>
      </TooltipContent>
    </Tooltip>
  )
}

export type { ParserOutputCopyProps }

export { ParserOutputCopy }
