import { Button } from '@shadcn/button'
import { Card, CardDescription, CardFooter, CardHeader, CardTitle } from '@shadcn/card'
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@shadcn/dialog'
import { InputGroupButton } from '@shadcn/input-group'
import { ReactElement, useState } from 'react'

import { isNonEmptyArray } from '@/lib/is/is-non-empty-array'
import { Parse } from '@/types/ppeg/parse'

import { getPresets } from './get-presets'
import { parserOutputStore } from './parser-output-store'
import { useParserForm } from './use-parser-form'

interface ParserGrammarPresetsProps {
  form: ReturnType<typeof useParserForm>
}

const ParserGrammarPresets = ({ form }: ParserGrammarPresetsProps): ReactElement => {
  const [open, setOpen] = useState(false)

  const presets = getPresets()

  const loadPreset = (preset: Parse) => {
    if (form.getFieldValue('grammarType') === 'json' && preset.grammarType === 'json') {
      form.setFieldValue('grammar', JSON.stringify(preset.grammarObject, null, 2))
    }

    form.setFieldValue('rule', '')
    form.setFieldValue('input', '')

    parserOutputStore.setState(() => ({ remaining: '', cst: undefined }))

    setOpen(false)
  }

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <InputGroupButton variant="default">presets</InputGroupButton>
      </DialogTrigger>
      <DialogContent className="sm:max-w-3/4 md:w-1/2 lg:w-1/3 h-[80%]">
        <DialogHeader>
          <DialogTitle>grammar presets</DialogTitle>
        </DialogHeader>
        <div className="w-full space-y-4 overflow-y-auto py-4">
          {isNonEmptyArray(presets)
            ? presets.map((preset, index) => (
                <Card key={index} size="sm" className="mx-auto w-full max-w-sm">
                  <CardHeader>
                    <CardTitle>{preset.label}</CardTitle>
                    <CardDescription>{preset.description}</CardDescription>
                  </CardHeader>
                  <CardFooter>
                    <Button variant="outline" size="sm" className="w-full" onClick={() => loadPreset(preset.parse)}>
                      load
                    </Button>
                  </CardFooter>
                </Card>
              ))
            : undefined}
        </div>
        <DialogFooter>
          <DialogClose asChild>
            <Button variant="default">close</Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}

export type { ParserGrammarPresetsProps }

export { ParserGrammarPresets }
