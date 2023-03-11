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

  let minNodeOccurs = Math.floor(scaleNode.invert(values.minNodeOccurs));
  let maxNodeOccurs = Math.ceil(scaleNode.invert(values.maxNodeOccurs));
  let minEdgeOccurs = Math.floor(scaleEdge.invert(values.minEdgeOccurs));
  let maxEdgeOccurs = Math.ceil(scaleEdge.invert(values.maxEdgeOccurs));

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

  //let origEdgesCount: { [nodeName: string]: number } = {};
  //for (const edge of initData.edges) {
  //  origEdgesCount[edge.source.name] = origEdgesCount[edge.source.name] || 0;
  //  origEdgesCount[edge.source.name] += 1;
  //  origEdgesCount[edge.target.name] = origEdgesCount[edge.target.name] || 0;
  //  origEdgesCount[edge.target.name] += 1;
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


  let largestNode = newNodes[0];
  let newEdgesCount: { [nodeName: string]: number } = {};

  for (const node of newNodes) {
    newEdgesCount[node.name] = 0;
  }
  for (const edge of newEdges) {
    newEdgesCount[edge.source.name] = newEdgesCount[edge.source.name] || 0;
    newEdgesCount[edge.source.name] += 1;
    newEdgesCount[edge.target.name] = newEdgesCount[edge.target.name] || 0;
    newEdgesCount[edge.target.name] += 1;
  }

  // filter disconnected nodes
  if (!values.disconnected) {
    newNodes = newNodes.filter((n) => newEdgesCount[n.name] !== 0 || n.name === largestNode.name);
  }

  return {
    ...initData,
    nodes: newNodes,
    edges: newEdges
  }
};

export { initData, truncateData, filterData };
