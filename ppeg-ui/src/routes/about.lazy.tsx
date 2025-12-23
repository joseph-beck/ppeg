import { createLazyFileRoute } from '@tanstack/react-router'

import { JsonEditor } from '@/components/shared/json-editor'

const Route = createLazyFileRoute('/about')({
  component: Page,
})

function Page() {
  return (
    <>
      <div className="p-2">Hello from About!</div>
      <JsonEditor />
    </>
  )
}

export { Route }
