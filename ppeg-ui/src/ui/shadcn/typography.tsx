import * as React from 'react'

import { cn } from '@/lib/utils'

function H1({ className, ...props }: React.ComponentProps<'h1'>) {
  return <h1 className={cn('scroll-m-20 text-4xl font-bold tracking-tight text-balance', className)} {...props} />
}

function H2({ className, ...props }: React.ComponentProps<'h2'>) {
  return <h2 className={cn('scroll-m-20 text-3xl font-semibold tracking-tight text-balance', className)} {...props} />
}

function H3({ className, ...props }: React.ComponentProps<'h3'>) {
  return <h3 className={cn('scroll-m-20 text-2xl font-semibold tracking-tight text-balance', className)} {...props} />
}

function H4({ className, ...props }: React.ComponentProps<'h4'>) {
  return <h4 className={cn('scroll-m-20 text-xl font-semibold tracking-tight text-balance', className)} {...props} />
}

function H5({ className, ...props }: React.ComponentProps<'h5'>) {
  return <h5 className={cn('scroll-m-20 text-lg font-semibold tracking-tight text-balance', className)} {...props} />
}

function H6({ className, ...props }: React.ComponentProps<'h6'>) {
  return <h6 className={cn('scroll-m-20 text-base font-semibold tracking-tight text-balance', className)} {...props} />
}

function P({ className, ...props }: React.ComponentProps<'p'>) {
  return <p className={cn('leading-7 not-first:mt-6', className)} {...props} />
}

function Blockquote({ className, ...props }: React.ComponentProps<'blockquote'>) {
  return <blockquote className={cn('border-l-2 pl-6 italic', className)} {...props} />
}

function List({ className, ...props }: React.ComponentProps<'ul'>) {
  return <ul className={cn('ml-6 list-disc', className)} {...props} />
}

function Code({ className, ...props }: React.ComponentProps<'code'>) {
  return (
    <code
      className={cn('bg-muted relative rounded px-[0.3rem] py-[0.2rem] font-mono text-sm font-semibold', className)}
      {...props}
    />
  )
}

function Lead({ className, ...props }: React.ComponentProps<'p'>) {
  return <p className={cn('text-muted-foreground text-xl', className)} {...props} />
}

function Large({ className, ...props }: React.ComponentProps<'div'>) {
  return <div className={cn('text-lg font-semibold', className)} {...props} />
}

function Small({ className, ...props }: React.ComponentProps<'small'>) {
  return <small className={cn('text-sm leading-none font-medium', className)} {...props} />
}

function Muted({ className, ...props }: React.ComponentProps<'p'>) {
  return <p className={cn('text-muted-foreground text-sm', className)} {...props} />
}

export { Blockquote, Code, H1, H2, H3, H4, H5, H6, Large, Lead, List, Muted, P, Small }
