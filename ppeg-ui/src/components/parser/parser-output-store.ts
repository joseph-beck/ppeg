import { Store } from '@tanstack/react-store'

import { Output } from '@/types/ppeg/output'

const defaultParserOutputStoreState: Output = {
  remaining: '',
  cst: undefined,
}

const parserOutputStore = new Store<Output>({
  ...defaultParserOutputStoreState,
})

export { parserOutputStore }
