import { InputGroupButton } from '@shadcn/input-group'
import { Tooltip, TooltipContent, TooltipTrigger } from '@shadcn/tooltip'
import { IconCheck, IconCopy } from '@tabler/icons-react'
import { ReactElement, useState } from 'react'

import { P } from '@/ui/shadcn/typography'

interface CopyGroupProps {
  input: string
}

/**
 * A reusable copy button component for copying text to clipboard.
 * This variant must be used within an `InputGroup` component.
 **/
const CopyGroup = ({ input }: CopyGroupProps): ReactElement => {
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

export type { CopyGroupProps }

export { CopyGroup }
