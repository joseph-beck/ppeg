import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectTrigger, SelectValue } from '@shadcn/select'
import { isNonEmptyArray } from '@tanstack/react-form'
import { useStore } from '@tanstack/react-store'
import { ReactElement } from 'react'

import { parserRuleStore } from './parser-rule-store'
import { useParserForm } from './use-parser-form'

interface ParserInputRuleSelectProps {
  form: ReturnType<typeof useParserForm>
}

const ParserInputRuleSelect = ({ form }: ParserInputRuleSelectProps): ReactElement => {
  const rules = useStore(parserRuleStore, (state) => state)

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
            disabled={!isNonEmptyArray(rules)}
          >
            <SelectTrigger id="parse-rule">
              <SelectValue placeholder="Rule" />
            </SelectTrigger>
            <SelectContent>
              <SelectGroup>
                {isNonEmptyArray(rules) ? (
                  <>
                    <SelectLabel>Rules</SelectLabel>
                    {rules.map((ruleName) => (
                      <SelectItem key={ruleName} value={ruleName}>
                        {ruleName}
                      </SelectItem>
                    ))}
                  </>
                ) : undefined}
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
