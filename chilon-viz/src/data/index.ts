import { scaleLog } from 'd3-scale';
import { ConfigValues } from '../events/config';
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


  // filter nodes
  let newNodes = initData.nodes.filter((n) => n.count >= minNodeOccurs && n.count <= maxNodeOccurs);

  // filter blank or unknown
  if (!values.bau) {
    newNodes = newNodes.filter((n) => n.name !== 'BLANK' && n.name !== 'UNKNOWN');
  }

  // log vs linear scale
  newNodes = newNodes.map((n) => {
    n.normCount = values.logarithm ? n.logScaleCount : n.linScaleCount;
    return n;
  });




  // filter outer nodes
  //let edgesCount: { [nodeName: string]: number } = {};
  //for (const node of initData.nodes) {
  //  edgesCount[node.name] = 0;
  //}
  //for (const edge of initData.edges) {
  //  edgesCount[edge.source.name] += 1;
  //  edgesCount[edge.target.name] += 1;
  //}

  //if (!values.outer) {
  //  newNodes = newNodes.filter((n) => edgesCount[n.name] > 1);
  //}


  // filter edges
  let namesToNodes: { [name: string]: boolean } = {};
  for (const node of newNodes) {
    namesToNodes[node.name] = true;
  }

  let newEdges = initData.edges.filter(e =>
    e.count >= minEdgeOccurs &&
    e.count <= maxEdgeOccurs &&
    (e as any).source.name in namesToNodes &&
    (e as any).target.name in namesToNodes
  );


  // filter loops
  if (!values.loops) {
    newEdges = newEdges.filter((e) => e.source.name !== e.target.name);
  }

  // filter datatype links
  if (!values.datatypes) {
    newEdges = newEdges.filter((e) => !e.is_datatype);
  }

  return {
    ...initData,
    nodes: newNodes,
    edges: newEdges
  }
};

export { initData, truncateData, filterData };
