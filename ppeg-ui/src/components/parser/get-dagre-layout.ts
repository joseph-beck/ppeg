import dagre from '@dagrejs/dagre'

const layout = new dagre.graphlib.Graph().setDefaultEdgeLabel(() => ({}))

const getDagreLayout = (): dagre.graphlib.Graph => {
  return layout
}

export { getDagreLayout }
