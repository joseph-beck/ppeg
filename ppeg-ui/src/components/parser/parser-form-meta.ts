type SubmitAction = 'input' | 'rule' | 'grammar' | 'autosave'

interface ParserFormMeta {
  submitAction?: SubmitAction
}

const defaultParserFormMeta: ParserFormMeta = {
  submitAction: undefined,
}

export type { ParserFormMeta }

export { defaultParserFormMeta }
