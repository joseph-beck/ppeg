// the code for this is adapted from the reactflow dagre example:
// https://reactflow.dev/examples/layout/dagre

import dagre from '@dagrejs/dagre'
import { Edge, Node, Position } from '@xyflow/react'

import { getDagreLayout } from './get-dagre-layout'

const nodeWidth = 172
const nodeHeight = 36

type Direction = 'TB' | 'LR'

interface GetElementLayoutResult {
  nodes: Node[]
  edges: Edge[]
}

const getElementLayout = (nodes: Node[], edges: Edge[], direction: Direction = 'LR'): GetElementLayoutResult => {
  const layout = getDagreLayout()

  const isHorizontal = direction === 'LR'

  layout.setGraph({
    rankdir: direction,
  })

  nodes.forEach((node) => {
    layout.setNode(node.id, {
      width: nodeWidth,
      height: nodeHeight,
    })
  })

  edges.forEach((edge) => {
    layout.setEdge(edge.source, edge.target)
  })

  dagre.layout(layout)

  const newNodes = nodes.map((node) => {
    const nodeWithPosition = layout.node(node.id)

    const newNode = {
      ...node,
      targetPosition: isHorizontal ? Position.Left : Position.Top,
      sourcePosition: isHorizontal ? Position.Right : Position.Bottom,
      position: {
        x: nodeWithPosition.x - nodeWidth / 2,
        y: nodeWithPosition.y - nodeHeight / 2,
      },
    }

    return newNode
  })

  return {
    nodes: newNodes,
    edges,
  }
}

export type { Direction, GetElementLayoutResult }

export { getElementLayout }
