import { createLazyFileRoute } from '@tanstack/react-router'

export const Route = createLazyFileRoute('/docs')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/docs"!</div>
}
