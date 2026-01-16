import { H1 } from '@shadcn/typography'
import { ReactElement } from 'react'

const PPEGTitle = (): ReactElement => {
  const letters = [
    {
      letter: 'p',
      color: 'text-orange-500',
    },
    {
      letter: 'p',
      color: 'dark:text-white light:text-black',
    },
    {
      letter: 'e',
      color: 'text-purple-600',
    },
    {
      letter: 'g',
      color: 'text-green-700',
    },
  ]

  return (
    <H1 className="mt-24 mb-12">
      {letters.map(({ letter, color }, index) => (
        <span key={index} className={color}>
          {letter}
        </span>
      ))}
    </H1>
  )
}

export { PPEGTitle }
