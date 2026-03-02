import { formOptions } from '@tanstack/react-form'

const defaultParserFormOpts = formOptions({
  defaultValues: {
    grammar: '',
    grammarType: 'ppeg',
    input: '',
    rule: '',
  },
})

export { defaultParserFormOpts }
