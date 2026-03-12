import '@xyflow/react/dist/style.css'

import { Button } from '@shadcn/button'
import { P } from '@shadcn/typography'
import { useStore } from '@tanstack/react-store'
import { Background, ConnectionLineType, Panel, ReactFlow, useEdgesState, useNodesState } from '@xyflow/react'
import { ReactElement, useCallback, useEffect, useMemo } from 'react'

import { Direction, getElementLayout } from './get-element-layout'
import { getOutputElements } from './get-output-elements'
import { parserOutputStore } from './parser-output-store'

const ParserOutputGraph = (): ReactElement => {
  const store = useStore(parserOutputStore)

  const elements = useMemo(() => getOutputElements(store.cst), [store.cst])

  const initialLayout = useMemo(() => getElementLayout(elements.nodes, elements.edges, 'LR'), [elements])

  const [nodes, setNodes, onNodesChange] = useNodesState(initialLayout.nodes)

  const [edges, setEdges, onEdgesChange] = useEdgesState(initialLayout.edges)

  useEffect(() => {
    const layout = getElementLayout(elements.nodes, elements.edges, 'LR')

    setNodes(layout.nodes)

    setEdges(layout.edges)
  }, [store.cst, elements, setEdges, setNodes])

  const onLayout = useCallback(
    (direction: Direction) => {
      const layout = getElementLayout(nodes, edges, direction)

      setNodes([...layout.nodes])

      setEdges([...layout.edges])
    },
    [nodes, setEdges, edges, setNodes],
  )

  const className =
    'w-full h-full lg:min-h-160 md:min-h-120 sm:min-h-80 flex items-center justify-center border rounded-lg bg-muted/20'

  if (!store.cst) {
    return (
      <div className={className}>
        <P>parse an input to view a parse tree...</P>
      </div>
    )
  }

  return (
    <div className={className}>
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        connectionLineType={ConnectionLineType.SmoothStep}
        fitView
      >
        <Panel position="top-right">
          <Button onClick={() => onLayout('TB')} className="mr-2">
            vertical layout
          </Button>
          <Button onClick={() => onLayout('LR')} className="mr-2">
            horizontal layout
          </Button>
        </Panel>
        <Background />
      </ReactFlow>
    </div>
  )
}

export { ParserOutputGraph }
