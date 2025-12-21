import { InputGroupButton } from '@shadcn/input-group'
import { Tooltip, TooltipContent, TooltipTrigger } from '@shadcn/tooltip'
import { P } from '@shadcn/typography'
import { IconCheck, IconCopy } from '@tabler/icons-react'
import { useStore } from '@tanstack/react-form'
import { ReactElement, useState } from 'react'

import { useParserForm } from './use-parser-form'

interface ParserInputCopyProps {
  form: ReturnType<typeof useParserForm>
}

const ParserInputCopy = ({ form }: ParserInputCopyProps): ReactElement => {
  const input = useStore(form.store, (state) => state.values.input)

  const [copied, setCopied] = useState<boolean>(false)

  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <InputGroupButton
          className="ml-auto"
          size="icon-xs"
          onClick={() => {
            navigator.clipboard.writeText(input)

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

export type { ParserInputCopyProps }

export { ParserInputCopy }
