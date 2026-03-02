import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectTrigger, SelectValue } from '@shadcn/select'

import { isNonEmptyArray } from '@/lib/is/is-non-empty-array'
import { GrammarType } from '@/types/ppeg/grammar-type'

import { useParserForm } from './use-parser-form'

interface ParserGrammarTypeSelectProps {
  form: ReturnType<typeof useParserForm>
}

const ParserGrammarTypeSelect = ({ form }: ParserGrammarTypeSelectProps) => {
  const grammarTypes: GrammarType[] = ['json', 'ppeg']

  return (
    <form.Field
      name="grammarType"
      children={(field) => (
        <Select
          value={field.state.value ?? ''}
          onValueChange={field.handleChange}
          onOpenChange={(open) => {
            if (!open) field.handleBlur()
          }}
          disabled={!isNonEmptyArray(grammarTypes)}
        >
          <SelectTrigger id="parse-grammar-type">
            <SelectValue placeholder="Grammar Type" />
          </SelectTrigger>
          <SelectContent>
            <SelectGroup>
              {isNonEmptyArray(grammarTypes) ? (
                <>
                  <SelectLabel>Grammar Types</SelectLabel>
                  {grammarTypes.map((type) => (
                    <SelectItem key={type} value={type}>
                      {type}
                    </SelectItem>
                  ))}
                </>
              ) : undefined}
            </SelectGroup>
          </SelectContent>
        </Select>
      )}
    />
  )
}

export type { ParserGrammarTypeSelectProps }

export { ParserGrammarTypeSelect }
