import { useMutation } from '@tanstack/react-query'
import axios from 'axios'

import { parserInputRuleWordMock } from '@/mocks/parser-input-rule-word-mock'
import { Output } from '@/types/ppeg/output'
import { Parse } from '@/types/ppeg/parse'

const useParserMutation = () => {
  // placeholder impl with hardcoded values for now
  return useMutation<Output, unknown, Parse>({
    mutationKey: ['parser-mutation'],
    mutationFn: async (parse) => {
      void parse

      return axios.post('http://127.0.0.1:8080/v1/parse', parserInputRuleWordMock).then((res) => res.data)
    },
    onSuccess: (data) => {
      void data
    },
  })
}

export { useParserMutation }
