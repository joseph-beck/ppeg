import { Button } from '@shadcn/button'
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@shadcn/dialog'
import { InputGroup, InputGroupAddon, InputGroupButton, InputGroupText } from '@shadcn/input-group'
import { IconBinaryTree } from '@tabler/icons-react'
import { ReactElement } from 'react'

import { ParserOutputCopy } from './parser-output-copy'
import { ParserOutputEditor } from './parser-output-editor'
import { ParserOutputGraph } from './parser-output-graph'

const ParserOutput = (): ReactElement => {
  return (
    <div className="w-full sm:max-w-4/5 md:max-w-2/3 lg:max-w-1/2">
      <InputGroup>
        <InputGroupAddon align="block-start" className="border-b">
          <InputGroupText className="font-mono font-medium">output</InputGroupText>
          <div className="ml-auto">
            <ParserOutputCopy />
          </div>
        </InputGroupAddon>
        <ParserOutputEditor />
        <InputGroupAddon align="block-end" className="border-t justify-end">
          <Dialog>
            <DialogTrigger asChild>
              <InputGroupButton variant="default">
                <IconBinaryTree />
                view graph
              </InputGroupButton>
            </DialogTrigger>
            <DialogContent className="sm:max-w-3/4">
              <DialogHeader>
                <DialogTitle>parse tree</DialogTitle>
              </DialogHeader>
              <div className="w-full h-full">
                <ParserOutputGraph />
              </div>
              <DialogFooter>
                <DialogClose asChild>
                  <Button variant="default">close</Button>
                </DialogClose>
              </DialogFooter>
            </DialogContent>
          </Dialog>
        </InputGroupAddon>
      </InputGroup>
    </div>
  )
}

export { ParserOutput }
