import { Edge, Node } from '@xyflow/react'

import { isLeaf } from '@/lib/cst/is-leaf'
import { isRoot } from '@/lib/cst/is-root'
import { CST } from '@/types/ppeg/cst'

import { defaultNodePosition } from './default-node-position'

interface GetOutputElementsResult {
  nodes: Node[]
  edges: Edge[]
}

const getOutputElements = (cst?: CST): GetOutputElementsResult => {
  const nodes: Node[] = []
  const edges: Edge[] = []

  if (!cst) {
    return {
      nodes,
      edges,
    }
  }

  let nodeId = 0

  const traverse = (node: CST, parentId?: string): string => {
    const id = String(nodeId++)

    nodes.push({
      id,
      type: isRoot(parentId) ? 'input' : isLeaf(node) ? 'output' : undefined,
      data: {
        label: node.value,
      },
      position: defaultNodePosition,
      draggable: false,
    })

    if (parentId !== undefined) {
      edges.push({
        id: `e${parentId}-${id}`,
        source: parentId,
        target: id,
        animated: true,
      })
    }

    for (const child of node.children) {
      traverse(child, id)
    }

    return id
  }

  traverse(cst)

  return {
    nodes,
    edges,
  }
}

export { getOutputElements }
