import { InputGroupButton } from '@shadcn/input-group'
import { IconCopy } from '@tabler/icons-react'
import { ReactElement } from 'react'

const ParserInputCopy = (): ReactElement => {
  return (
    <InputGroupButton className="ml-auto" size="icon-xs">
      <IconCopy />
    </InputGroupButton>
  )
}

export { ParserInputCopy }
