import { useMutation } from '@tanstack/react-query'
import axios from 'axios'

import { Output } from '@/types/ppeg/output'
import { Parse } from '@/types/ppeg/parse'

const useParserMutation = () => {
  // placeholder impl with hardcoded values for now
  return useMutation<Output, unknown, Parse>({
    mutationKey: ['parser-mutation'],
    mutationFn: async (parse) => {
      const { grammarType, ...requestBody } = parse

      console.log('requestBody', requestBody)
      console.log('grammarType', grammarType)

      return axios
        .post(`http://localhost:8080/v1/parse?grammar_type=${grammarType}`, requestBody)
        .then((res) => res.data)
    },
    onSuccess: (data) => {
      void data
    },
  })
}

export { useParserMutation }
