import { createRootRoute, Outlet } from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools'

import { ParserProvider } from '@/components/parser/parser-provider'
import { ThemeProvider } from '@/components/theme/theme-provider'

const Page = () => {
  return (
    <ThemeProvider defaultTheme="light" storageKey="vite-ui-theme">
      <ParserProvider>
        <Outlet />
      </ParserProvider>
      <TanStackRouterDevtools />
    </ThemeProvider>
  )
}

const Route = createRootRoute({
  component: Page,
})

export { Route }
