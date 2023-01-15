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
  return {
    nodes: data.nodes.filter(n => n.count >= values.minNodeOccurs && n.count <= values.maxNodeOccurs),
    edges: data.edges.filter(e => e.count >= values.minEdgeOccurs && e.count <= values.maxEdgeOccurs),
  }
};

export { initData, truncateData, filterData };
