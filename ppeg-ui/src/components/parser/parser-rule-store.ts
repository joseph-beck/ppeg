import { Store } from '@tanstack/react-store'

const defaultParserRuleStoreState: string[] = []

const parserRuleStore = new Store<string[]>(defaultParserRuleStoreState)

export { parserRuleStore }
