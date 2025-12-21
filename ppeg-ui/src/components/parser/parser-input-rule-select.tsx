import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectTrigger, SelectValue } from '@shadcn/select'
import { ReactElement } from 'react'

import { useParserForm } from './use-parser-form'

interface ParserInputRuleSelectProps {
  form: ReturnType<typeof useParserForm>
}

const ParserInputRuleSelect = ({ form }: ParserInputRuleSelectProps): ReactElement => {
  return (
    <div className="ml-auto">
      <form.Field
        name="rule"
        children={(field) => (
          <Select
            value={field.state.value ?? ''}
            onValueChange={field.handleChange}
            onOpenChange={(open) => {
              if (!open) field.handleBlur()
            }}
          >
            <SelectTrigger id="parse-rule">
              <SelectValue placeholder="Rule" />
            </SelectTrigger>
            <SelectContent>
              <SelectGroup>
                <SelectLabel>Rules</SelectLabel>
                <SelectItem value="rule">rule</SelectItem>
              </SelectGroup>
            </SelectContent>
          </Select>
        )}
      />
    </div>
  )
}

export type { ParserInputRuleSelectProps }

export { ParserInputRuleSelect }
