import { H1 } from '@shadcn/typography'
import { createLazyFileRoute } from '@tanstack/react-router'

import { ParserInputForm } from '@/components/parser/parser-input-form'
import { ParserOutput } from '@/components/parser/parser-output'

const Route = createLazyFileRoute('/')({
  component: Page,
})

function Page() {
  return (
    <div className="p-2 flex flex-col items-center ">
      <H1 className="mt-24 mb-12">ppeg</H1>
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
