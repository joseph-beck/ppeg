import { createFileRoute } from '@tanstack/react-router'

import { ParserInputForm } from '@/components/parser/parser-input-form'
import { ParserOutput } from '@/components/parser/parser-output-form'
import { PPEGTitle } from '@/components/shared/ppeg-title'

const Page = () => {
  return (
    <div className="p-2 flex flex-col items-center ">
      <PPEGTitle />
      <div
        className="
          w-full
          max-w-7xl
          flex
          flex-col
          gap-6
          md:flex-row
          md:items-start
        "
      >
        <ParserInputForm />
        <ParserOutput />
      </div>
    </div>
  )
}

const Route = createFileRoute('/')({
  component: Page,
})

export { Route }
