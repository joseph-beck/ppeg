import { useMutation, UseMutationResult } from '@tanstack/react-query'
import axios from 'axios'

import { Output } from '@/types/ppeg/output'
import { Parse } from '@/types/ppeg/parse'

const useParseMutation = (): UseMutationResult<Output, unknown, Parse> => {
  // placeholder impl with hardcoded values for now
  return useMutation<Output, unknown, Parse>({
    mutationFn: (parse) => {
      return axios.post('localhost:8080/v1/parse', parse)
    },
  })
}

export { useParseMutation }
