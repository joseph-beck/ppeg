import { H1 } from '@shadcn/typography'
import { createLazyFileRoute } from '@tanstack/react-router'

import { ParserInputForm } from '@/components/parser/parser-input-form'

const Route = createLazyFileRoute('/')({
  component: Page,
})

function Page() {
  return (
    <div className="p-2">
      <H1>ppeg</H1>
      <ParserInputForm />
    </div>
  )
}

export { Route }
