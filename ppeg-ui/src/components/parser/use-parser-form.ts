import { useForm } from '@tanstack/react-form'

import { parse } from '@/types/ppeg/parse'

import { defaultParserFormOpts } from './default-parser-form-opts'
import { defaultParserFormMeta, ParserFormMeta } from './parser-form-meta'
import { parserStore } from './parser-store'
import { useParserMutation } from './use-parser-mutation'

const useParserForm = () => {
  const mutation = useParserMutation()

  return useForm({
    ...defaultParserFormOpts,
    // still requires a manual cast even though it is of type ParserFormMeta
    onSubmitMeta: defaultParserFormMeta as ParserFormMeta,
    onSubmit: async ({ value, meta }) => {
      void meta

      const result = await mutation.mutateAsync(value)

      parserStore.setState((state) => {
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
        const result = parse.safeParse(value)

        if (!result.success) {
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
