import { createLazyFileRoute } from '@tanstack/react-router'

import { ParserInputForm } from '@/components/parser/parser-input-form'

const Route = createLazyFileRoute('/')({
  component: Page,
})

function Page() {
  return (
    <div className="p-2">
      <ParserInputForm />
    </div>
  )
}

export { Route }
