import init, { getRules as getRulesWasm, parse as parseWasm } from 'ppeg-wasm'
import { createContext, ReactNode, useCallback, useEffect, useMemo, useState } from 'react'

import { Output, output } from '@/types/ppeg/output'
import { ruleNames } from '@/types/ppeg/rule'

interface ParserContextValue {
  isReady: boolean
  parse: (input: string, grammar: string, rule: string) => Output
  getRules: (grammar: string) => string[]
}

const ParserContext = createContext<ParserContextValue | undefined>(undefined)

interface ParserProviderProps {
  children: ReactNode
}

const ParserProvider = ({ children }: ParserProviderProps) => {
  const [isReady, setIsReady] = useState(false)

  useEffect(() => {
    let mounted = true

    init()
      .then(() => {
        if (mounted) {
          setIsReady(true)
        }
      })
      .catch((error) => {
        console.error('failed to init ppeg wasm', error)
      })

    return () => {
      mounted = false
    }
  }, [])

  const parse = useCallback<ParserContextValue['parse']>(
    (input, grammar, rule) => {
      if (!isReady) {
        throw new Error('parser not yet inited')
      }

      const result = output.safeParse(parseWasm(input, grammar, rule))

      if (result.success) {
        return result.data
      }

      throw new Error(`failed to parse input: ${result.error}`)
    },
    [isReady],
  )

  const getRules = useCallback<ParserContextValue['getRules']>(
    (grammar) => {
      if (!isReady) {
        throw new Error('parser not yet inited')
      }

      const result = ruleNames.safeParse(getRulesWasm(grammar))

      if (result.success) {
        return result.data
      }

      throw new Error('failed to get rules')
    },
    [isReady],
  )

  const value = useMemo(
    () => ({
      isReady,
      parse,
      getRules,
    }),
    [isReady, getRules, parse],
  )

  return <ParserContext.Provider value={value}>{children}</ParserContext.Provider>
}

export type { ParserContextValue, ParserProviderProps }

export { ParserContext, ParserProvider }
