import { scaleLog } from 'd3-scale';
import { ConfigValues } from '../events/sliders';
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

const filterData = (initData: SimData, values: ConfigValues) => {
  let lowestNodeOccurs = initData.nodes.slice(-1)[0].count;
  let highestNodeOccurs = initData.nodes[0].count;
  let lowestEdgeOccurs = initData.edges.slice(-1)[0].count;
  let highestEdgeOccurs = initData.edges[0].count;

  const scaleNode = scaleLog()
    .domain([lowestNodeOccurs, highestNodeOccurs])
    .range([0, 100]);
  const scaleEdge = scaleLog()
    .domain([lowestEdgeOccurs, highestEdgeOccurs])
    .range([0, 100]);

  let minNodeOccurs = scaleNode.invert(values.minNodeOccurs);
  let maxNodeOccurs = scaleNode.invert(values.maxNodeOccurs);
  let minEdgeOccurs = scaleEdge.invert(values.minEdgeOccurs);
  let maxEdgeOccurs = scaleEdge.invert(values.maxEdgeOccurs);

  let newNodes = initData.nodes.filter((n) => n.count >= minNodeOccurs && n.count <= maxNodeOccurs);

  let namesToNodes: { [name: string]: boolean } = {};
  for (const node of newNodes) {
    namesToNodes[node.name] = true;
  }

  const newEdges = initData.edges.filter(e =>
    e.count >= minEdgeOccurs &&
    e.count <= maxEdgeOccurs &&
    (e as any).source.name in namesToNodes &&
    (e as any).target.name in namesToNodes
  );

  console.log('XXXXXXXX 5', { data: initData, newNodes, newEdges })

  return {
    ...initData,
    nodes: newNodes,
    edges: newEdges
  }
};

export { initData, truncateData, filterData };
