import { useForm } from '@tanstack/react-form'

import { parse } from '@/types/ppeg/parse'

import { defaultParserFormOpts } from './default-parser-form-opts'

const useParserForm = () =>
  useForm({
    ...defaultParserFormOpts,
    onSubmit: async ({ value }) => {
      console.log(value)
    },
    validators: {
      onChange: parse,
    },
  })

export { useParserForm }
