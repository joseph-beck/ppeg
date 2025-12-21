import { InputGroupButton } from '@shadcn/input-group'
import { Tooltip, TooltipContent, TooltipTrigger } from '@shadcn/tooltip'
import { P } from '@shadcn/typography'
import { IconCheck, IconCopy } from '@tabler/icons-react'
import { ReactElement, useState } from 'react'

import { useParserForm } from './use-parser-form'

const ParserInputCopy = (): ReactElement => {
  const form = useParserForm()

  const [copied, setCopied] = useState<boolean>(false)

  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <InputGroupButton
          className="ml-auto"
          size="icon-xs"
          onClick={() => {
            navigator.clipboard.writeText(form.getFieldValue('input'))

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

export { ParserInputCopy }
