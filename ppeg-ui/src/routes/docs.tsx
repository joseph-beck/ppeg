import { createFileRoute } from '@tanstack/react-router'

import { PPEGTitle } from '@/components/shared/ppeg-title'

const Page = () => {
  return (
    <div className="p-2 flex flex-col items-center ">
      <PPEGTitle />
    </div>
  )
}

const Route = createFileRoute('/docs')({
  component: Page,
})

export { Route }
