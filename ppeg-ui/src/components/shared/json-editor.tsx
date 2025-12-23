import { json } from '@codemirror/lang-json'
import { xcodeDark } from '@uiw/codemirror-theme-xcode'
import ControlledEditor from '@uiw/react-codemirror'
import { basicSetup } from 'codemirror'
import { useState } from 'react'

export const JsonEditor = () => {
  const [value, setValue] = useState<string>(`{
  "name": "John",
  "age": 30
}`)

  return (
    <ControlledEditor
      readOnly
      value={value}
      height="300px"
      extensions={[basicSetup, json()]}
      theme={xcodeDark}
      onChange={(value: string) => setValue(value)}
    />
  )
}
