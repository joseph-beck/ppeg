import { formOptions } from '@tanstack/react-form'

import { parserInputRuleCharMock } from '@/mocks/parser-input-rule-char-mock'

const defaultParserFormOpts = formOptions({
  defaultValues: {
    grammar: JSON.stringify(parserInputRuleCharMock.grammar, null, 2),
    input: '',
    rule: '',
  },
})

export { defaultParserFormOpts }
