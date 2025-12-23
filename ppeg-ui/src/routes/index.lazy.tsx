import { createLazyFileRoute } from '@tanstack/react-router'

import { ParserInputForm } from '@/components/parser/parser-input-form'
import { ParserOutput } from '@/components/parser/parser-output'
import { PPEGTitle } from '@/components/shared/ppeg-title'

const Route = createLazyFileRoute('/')({
  component: Page,
})

function Page() {
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

export { Route }
