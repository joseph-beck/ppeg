import { formOptions } from '@tanstack/react-form'

const defaultParserFormOpts = formOptions({
  defaultValues: {
    grammar: '',
    input: '',
    rule: '',
  },
})

export { defaultParserFormOpts }
