import { Textarea } from '@shadcn/textarea'
import { ReactElement } from 'react'

const ParserOutput = (): ReactElement => {
  return (
    <div className="w-full max-w-md mb-12">
      <Textarea readOnly={true} placeholder="output"></Textarea>
    </div>
  )
}

export { ParserOutput }
