import dagre from '@dagrejs/dagre'

const getDagreLayout = (): dagre.graphlib.Graph => {
  const graph = new dagre.graphlib.Graph()
  graph.setDefaultEdgeLabel(() => ({}))

  return graph
}

export { getDagreLayout }
