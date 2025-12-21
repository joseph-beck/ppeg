import { H1 } from '@shadcn/typography'
import { createLazyFileRoute } from '@tanstack/react-router'

import { ParserInputForm } from '@/components/parser/parser-input-form'

const Route = createLazyFileRoute('/')({
  component: Page,
})

function Page() {
  return (
    <div className="p-2 flex flex-col items-center sm:min-w-screen md:min-w-7/12 lg:min-w-2/3">
      <H1 className="mt-24 mb-12">ppeg</H1>
      <ParserInputForm />
    </div>
  )
}

export { Route }
