import { rawData, RawData } from './raw-data';


const truncateRawData = (rawData: RawData, maxNodes = 50, maxEdges = 50) => {
  let res = {
    nodes: rawData.nodes.slice(0, maxNodes),
    edges: rawData.edges.slice(0, maxEdges)
  };

  return res;
}

export { rawData, truncateRawData };
