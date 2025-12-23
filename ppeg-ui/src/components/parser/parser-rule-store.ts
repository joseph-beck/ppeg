import { Store } from '@tanstack/react-store'

import { Rule } from '@/types/ppeg/rule'

const defaultParserRuleStoreState: Rule[] = []

const parserRuleStore = new Store<Rule[]>(defaultParserRuleStoreState)

export { parserRuleStore }
