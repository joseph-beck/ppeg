import { json } from '@codemirror/lang-json'
import { xcodeDark } from '@uiw/codemirror-theme-xcode'
import ControlledEditor from '@uiw/react-codemirror'
import { basicSetup } from 'codemirror'
import { ReactElement, useEffect, useState } from 'react'

interface JsonEditorProps {
  id?: string
  value: string
  onChange?: (value: string) => void
  readonly?: boolean
}

const JsonEditor = (props: JsonEditorProps): ReactElement => {
  const [height, setHeight] = useState('300px')

  useEffect(() => {
    const calculateHeight = () => {
      const viewportHeight = typeof window !== 'undefined' ? window.innerHeight : 0
      const calculatedHeight = Math.max(viewportHeight * 0.55, 300)
      setHeight(`${calculatedHeight}px`)
    }

    calculateHeight()
    window.addEventListener('resize', calculateHeight)

    return () => {
      window.removeEventListener('resize', calculateHeight)
    }
  }, [])

  return (
    <ControlledEditor
      {...props}
      className="w-full overflow-auto"
      extensions={[basicSetup, json()]}
      lang="json"
      height={height}
      theme={xcodeDark}
    />
  )
}

export type { JsonEditorProps }

export { JsonEditor }
