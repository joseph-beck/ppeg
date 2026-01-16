import { json } from '@codemirror/lang-json'
import { xcodeDark, xcodeLight } from '@uiw/codemirror-theme-xcode'
import ControlledEditor from '@uiw/react-codemirror'
import { basicSetup } from 'codemirror'
import { ReactElement, useEffect, useState } from 'react'

import { useTheme } from '../theme/use-theme'

interface JsonEditorProps {
  id?: string
  heightPercent?: number
  miniumHeightPx?: number
  value: string
  onChange?: (value: string) => void
  readOnly?: boolean
}

const JsonEditor = ({ heightPercent = 0.55, miniumHeightPx = 300, ...props }: JsonEditorProps): ReactElement => {
  const { theme } = useTheme()

  const editorTheme = theme === 'dark' ? xcodeDark : xcodeLight

  const [height, setHeight] = useState(`${miniumHeightPx}px`)

  useEffect(() => {
    const calculateHeight = () => {
      const viewportHeight = typeof window !== 'undefined' ? window.innerHeight : 0
      const calculatedHeight = Math.max(viewportHeight * heightPercent, miniumHeightPx)
      setHeight(`${calculatedHeight}px`)
    }

    calculateHeight()
    window.addEventListener('resize', calculateHeight)

    return () => {
      window.removeEventListener('resize', calculateHeight)
    }
  }, [heightPercent, miniumHeightPx])

  return (
    <ControlledEditor
      {...props}
      className="w-full overflow-auto"
      extensions={[basicSetup, json()]}
      lang="json"
      height={height}
      theme={editorTheme}
    />
  )
}

export type { JsonEditorProps }

export { JsonEditor }
