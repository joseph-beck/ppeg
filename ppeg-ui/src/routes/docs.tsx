import { createFileRoute } from '@tanstack/react-router'
import { add } from 'ppeg-wasm'

import { PPEGTitle } from '@/components/shared/ppeg-title'

const Page = () => {
  console.log(add(2, 2))

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
