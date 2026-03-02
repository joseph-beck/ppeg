import { useForm } from '@tanstack/react-form'

import { parserSchemaTransform } from '@/lib/parser/parser-schema-transform'
import { Rule } from '@/types/ppeg/rule'
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
    onSubmit: async ({ value, meta: _meta }) => {
      const data = parserSchemaTransform(value)

      console.log('test0')

      if (data?.grammarType === 'json' && data?.grammarObject.rules) {
        console.log('test1')
        parserRuleStore.setState(() => {
          return [...data.grammarObject.rules]
        })
      }

      if (data?.grammarType === 'ppeg') {
        console.log('test2')
        // this is a pretty dirty trick to extra this information...
        // TODO: make this more robust.
        const rules = data.grammarMeta
          .split('\n')
          .filter((line) => line.includes(':='))
          .map((line) => {
            const [name, _] = line.split(':=')

            return {
              name: name.trim(),
              expression: { type: 'Empty' },
            } satisfies Rule
          })

        parserRuleStore.setState(() => {
          return [...rules]
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
