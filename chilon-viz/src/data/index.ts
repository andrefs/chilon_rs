import { SliderValues } from '../events/sliders';
import { initData, SimData } from './raw-data';


const truncateData = (data: SimData, maxNodes = 50, maxEdges = 50) => {
  let newNodes = data.nodes.slice(0, maxNodes);

  let namesToNodes: { [name: string]: boolean } = {};
  for (const node of newNodes) {
    namesToNodes[node.name] = true;
  }


  let res = {
    ...data,
    nodes: newNodes,
    edges: data.edges
      .slice(0, maxEdges)
      .filter(e => e.source.name in namesToNodes && e.target.name in namesToNodes)
  };

  return res;
}

const filterData = (data: SimData, values: SliderValues) => {
  let newNodes = data.nodes.filter((n) => n.count >= values.minNodeOccurs && n.count <= values.maxNodeOccurs);

  let namesToNodes: { [name: string]: boolean } = {};
  for (const node of newNodes) {
    namesToNodes[node.name] = true;
  }

  const newEdges = data.edges.filter(e =>
    e.count >= values.minEdgeOccurs &&
    e.count <= values.maxEdgeOccurs &&
    (e as any).source.name in namesToNodes &&
    (e as any).target.name in namesToNodes
  );

  console.log('XXXXXXXX 5', { data, newNodes, newEdges })

  return {
    ...data,
    nodes: newNodes,
    edges: newEdges
  }
};

export { initData, truncateData, filterData };
