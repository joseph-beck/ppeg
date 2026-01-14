import '@xyflow/react/dist/style.css'

import { Button } from '@shadcn/button'
import { P } from '@shadcn/typography'
import { useStore } from '@tanstack/react-store'
import { Background, ConnectionLineType, Panel, ReactFlow, useEdgesState, useNodesState } from '@xyflow/react'
import { ReactElement, useCallback, useEffect } from 'react'

import { Direction, getElementLayout } from './get-element-layout'
import { getOutputElements } from './get-output-elements'
import { parserOutputStore } from './parser-output-store'

const ParserGraphOutput = (): ReactElement => {
  const store = useStore(parserOutputStore)

  const elements = getOutputElements(store.cst)

  const initialLayout = getElementLayout(elements.nodes, elements.edges, 'TB')

  const [nodes, setNodes, onNodesChange] = useNodesState(initialLayout.nodes)

  const [edges, setEdges, onEdgesChange] = useEdgesState(initialLayout.edges)

  useEffect(() => {
    const layout = getElementLayout(elements.nodes, elements.edges, 'TB')

    setNodes(layout.nodes)

    setEdges(layout.edges)
  }, [elements, setNodes, setEdges])

  const onLayout = useCallback(
    (direction: Direction) => {
      const layout = getElementLayout(nodes, edges, direction)

      setNodes([...layout.nodes])

      setEdges([...layout.edges])
    },
    [nodes, setEdges, edges, setNodes],
  )

  if (!store.cst) {
    return (
      <div className="w-full max-w-7xl h-150 mt-6 flex items-center justify-center border rounded-lg bg-muted/20">
        <P>parse an input to view a parse tree...</P>
      </div>
    )
  }

  return (
    <div className="w-full max-w-7xl h-150 mt-6 flex items-center justify-center border rounded-lg bg-muted/20">
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

export { ParserGraphOutput }
