import { useForm } from '@tanstack/react-form'

import { schema } from '@/types/ppeg/schema'

import { defaultParserFormOpts } from './default-parser-form-opts'
import { defaultParserFormMeta, ParserFormMeta } from './parser-form-meta'
import { parserOutputStore } from './parser-output-store'
import { parserRuleStore } from './parser-rule-store'
import { useParser } from './use-parser'

const useParserForm = () => {
  const { parse, getRules } = useParser()

  return useForm({
    ...defaultParserFormOpts,
    // still requires a manual cast even though it is of type ParserFormMeta
    onSubmitMeta: defaultParserFormMeta as ParserFormMeta,
    onSubmit: async ({ value, meta: _meta }) => {
      console.log(value)

      try {
        const rules = getRules(value.grammar)
        parserRuleStore.setState(() => rules)
      } catch (_) {
        parserRuleStore.setState(() => [])
      }

      try {
        // until we have some data no transformations should occur in the mutation or parser output store
        if (!value || !value.rule || value.input === '') {
          return
        }

        const result = parse(value.input, value.grammar, value.rule)

        parserOutputStore.setState((state) => {
          return {
            ...state,
            remaining: result?.remaining ?? '',
            cst: result?.cst,
          }
        })
      } catch (error) {
        console.error('failed to parse input', error)

        parserOutputStore.setState((state) => {
          return {
            ...state,
            remaining: '',
            cst: undefined,
          }
        })
      }
    },
    onSubmitInvalid: ({ value, meta }) => {
      console.error(value, meta)
    },
    validators: {
      onChange: ({ value }) => {
        const result = schema.safeParse(value)

        if (!result.success) {
          console.error('Form is invalid', result.error)
          return result.error.issues.map((issue) => ({
            path: issue.path,
            message: issue.message,
          }))
        }

        return undefined
      },
    },
    listeners: {
      onChange: ({ formApi }) => {
        if (formApi.state.isValid) {
          formApi.handleSubmit({ submitAction: 'autosave' })
        }
      },
      onChangeDebounceMs: 250,
    },
  })
}

export { useParserForm }
