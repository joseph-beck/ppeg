import { formOptions } from '@tanstack/react-form'

const defaultParserFormOpts = formOptions({
  defaultValues: {
    grammar: {
      rules: [],
    },
    input: 'input',
    rule: '',
  },
})

export { defaultParserFormOpts }
