import { useForm } from '@tanstack/react-form'

import { parserSchemaTransform } from '@/lib/parser/parser-schema-transform'
import { schema } from '@/types/ppeg/schema'

import { defaultParserFormOpts } from './default-parser-form-opts'
import { defaultParserFormMeta, ParserFormMeta } from './parser-form-meta'
import { parserOutputStore } from './parser-output-store'
import { parserRuleStore } from './parser-rule-store'
import { useParserMutation } from './use-parser-mutation'

const useParserForm = () => {
  const mutation = useParserMutation()

  return useForm({
    ...defaultParserFormOpts,
    // still requires a manual cast even though it is of type ParserFormMeta
    onSubmitMeta: defaultParserFormMeta as ParserFormMeta,
    onSubmit: async ({ value, meta }) => {
      void meta

      const data = parserSchemaTransform(value)

      if (data?.grammar.rules) {
        parserRuleStore.setState(() => {
          return [...data.grammar.rules]
        })
      }

      // until we have some data no transformations should occur in the mutation or parser output store
      if (!data || !data.rule || data.input === '') {
        return
      }

      const result = await mutation.mutateAsync(data)

      parserOutputStore.setState((state) => {
        return {
          ...state,
          remaining: result?.remaining ?? '',
          cst: result?.cst,
        }
      })
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
