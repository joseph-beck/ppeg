import { createRootRoute, Outlet } from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools'
import init, { add } from 'ppeg-wasm'
import { useEffect } from 'react'

import { ThemeProvider } from '@/components/theme/theme-provider'

const Page = () => {
  useEffect(() => {
    init().then(() => {
      console.log('WASM loaded')

      console.log(add(2, 2))
    })
  }, [])

  return (
    <ThemeProvider defaultTheme="light" storageKey="vite-ui-theme">
      <Outlet />
      <TanStackRouterDevtools />
    </ThemeProvider>
  )
}

const Route = createRootRoute({
  component: Page,
})

export { Route }
