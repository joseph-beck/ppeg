import { Link } from '@tanstack/react-router'
import { ReactElement } from 'react'

const NavigationBar = (): ReactElement => {
  return (
    <div className="p-2 flex gap-2">
      <Link to="/" className="[&.active]:font-bold">
        Home
      </Link>{' '}
      <Link to="/about" className="[&.active]:font-bold">
        About
      </Link>
      <Link to="/docs" className="[&.active]:font-bold">
        Docs
      </Link>
    </div>
  )
}

export { NavigationBar }
