import {
  Field,
  FieldContent,
  FieldDescription,
  FieldError,
  FieldGroup,
  FieldLabel,
  FieldLegend,
  FieldSet,
} from '@shadcn/field'
import { Textarea } from '@shadcn/textarea'
import { ReactElement } from 'react'

import { useParserForm } from './use-parser-form'

interface ParserInputFormProps {
  _?: never
}

const ParserInputForm = (props: ParserInputFormProps): ReactElement => {
  void props

  const form = useParserForm()

  return (
    <div className="w-full max-w-md">
      <FieldGroup>
        <FieldLegend>Parser</FieldLegend>
        <FieldDescription>Generate a parse tree</FieldDescription>
        <FieldSet>
          <Field>
            <FieldLabel htmlFor="parse-input">Input</FieldLabel>
            <FieldContent>
              <form.Field
                name="input"
                children={(field) => (
                  <>
                    <Textarea
                      id="parse-input"
                      value={String(field.state.value)}
                      onBlur={field.handleBlur}
                      onChange={(e) => field.handleChange(e.target.value)}
                    />
                    <FieldDescription>What do you want to parse?</FieldDescription>
                    <FieldError>Validation message</FieldError>
                  </>
                )}
              />
            </FieldContent>
          </Field>
        </FieldSet>
      </FieldGroup>
    </div>
  )
}

export type { ParserInputFormProps }

export { ParserInputForm }
