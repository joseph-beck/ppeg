import { createLazyFileRoute } from '@tanstack/react-router'

import { PPEGTitle } from '@/components/shared/ppeg-title'

export const Route = createLazyFileRoute('/docs')({
  component: RouteComponent,
})

function RouteComponent() {
  return (
    <div className="p-2 flex flex-col items-center ">
      <PPEGTitle />
    </div>
  )
}
