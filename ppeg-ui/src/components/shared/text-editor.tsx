import { xcodeDark, xcodeLight } from '@uiw/codemirror-theme-xcode'
import ControlledEditor from '@uiw/react-codemirror'
import { basicSetup } from 'codemirror'
import { ReactElement, useEffect, useState } from 'react'

import { useTheme } from '../theme/use-theme'

interface TextEditorProps {
  id?: string
  heightPercent?: number
  miniumHeightPx?: number
  value: string
  onChange?: (value: string) => void
  readonly?: boolean
}

const TextEditor = ({ heightPercent = 0.55, miniumHeightPx = 300, ...props }: TextEditorProps): ReactElement => {
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
      extensions={[basicSetup]}
      lang="txt"
      height={height}
      theme={editorTheme}
    />
  )
}

export type { TextEditorProps }

export { TextEditor }
