import { SimulationLinkDatum, SimulationNodeDatum } from 'd3-force';
import { genColorHash } from '../colors';

export interface RawNode extends SimulationNodeDatum {
  id: number;
  name: String;
  count: number;
};

export type SimEdge = RawEdge & { colorHash: string };

export interface RawEdge extends SimulationLinkDatum<RawNode> {
  source: number;
  target: number;
  label: String;
  count: number;
  link_num: number;
};

interface RawData {
  edges: RawEdge[];
  nodes: RawNode[];
}

export class SimData {
  edges: SimEdge[];
  nodes: RawNode[];

  minNodeCount: number;
  maxNodeCount: number;
  minEdgeCount: number;
  maxEdgeCount: number;

  constructor({ nodes, edges }: RawData) {

    const colorHash = genColorHash(edges);


    this.nodes = nodes;
    this.edges = edges.map((e) => ({ ...e, colorHash: colorHash[e.label.toString()] }));

    this.minNodeCount = this.nodes.slice(-1)[0].count;
    this.maxNodeCount = this.nodes[0].count;
    this.minEdgeCount = this.edges.slice(-1)[0].count;
    this.maxEdgeCount = this.edges[0].count;
  }
}

export const initData = new SimData({{ data | json_encode(pretty = true) | safe }});
