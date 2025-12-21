import { Store } from '@tanstack/react-store'

import { Output } from '@/types/ppeg/output'

const defaultParserStoreState: Output = {
  remaining: '',
  cst: undefined,
}

const parserStore = new Store<Output>({
  ...defaultParserStoreState,
})

export { parserStore }
