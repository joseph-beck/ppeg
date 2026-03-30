import { useContext } from 'react'

import { ParserContext, ParserContextValue } from './parser-provider'

const useParser = (): ParserContextValue => {
  const context = useContext(ParserContext)

  if (!context) {
    throw new Error('useParser must be used within ParserProvider')
  }

  return context
}

export { useParser }
