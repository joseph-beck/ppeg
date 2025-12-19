import { createContext } from 'react'

import { Theme } from './theme'

interface ThemeProviderState {
  theme: Theme
  setTheme: (theme: Theme) => void
}

const initialState: ThemeProviderState = {
  theme: 'system',
  setTheme: () => undefined,
}

const ThemeProviderContext = createContext<ThemeProviderState>(initialState)

export type { ThemeProviderState }

export { ThemeProviderContext }
