import { SliderValues } from '../events/sliders';
import { initData, SimData } from './raw-data';


const truncateData = (data: SimData, maxNodes = 50, maxEdges = 50) => {

  let res = {
    ...data,
    nodes: data.nodes.slice(0, maxNodes),
    edges: data.edges.slice(0, maxEdges)
  };

  return res;
}

const filterData = (data: SimData, values: SliderValues) => {
  const newNodes = data.nodes.filter(n => n.count >= values.minNodeOccurs && n.count <= values.maxNodeOccurs);
  const nodeIds = new Set(newNodes.map(n => n.id))
  console.log('XXXXXXX', { nodeIds })

  const newEdges = data.edges.filter(e =>
    e.count >= values.minEdgeOccurs &&
    e.count <= values.maxEdgeOccurs &&
    nodeIds.has((e as any).source.id) &&
    nodeIds.has((e as any).target.id)
  );

  return {
    ...data,
    nodes: newNodes,
    edges: newEdges
  }
};

export { initData, truncateData, filterData };
