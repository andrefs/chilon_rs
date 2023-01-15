import { SimulationLinkDatum, SimulationNodeDatum } from 'd3-force';
import { genColorHash } from '../colors';

export interface RawNode extends SimulationNodeDatum {
  id: number;
  name: String;
  count: number;
};

export type FEdge = RawEdge & { colorHash: string };

export interface RawEdge extends SimulationLinkDatum<RawNode> {
  source: number;
  target: number;
  label: String;
  count: number;
  link_num: number;
};


export class RawData {
  edges: FEdge[];
  nodes: RawNode[];

  minNodeCount: number;
  maxNodeCount: number;
  minEdgeCount: number;
  maxEdgeCount: number;

  constructor({ nodes, edges }: { nodes: RawNode[], edges: RawEdge[] }) {

    const colorHash = genColorHash(edges);


    this.nodes = nodes;
    this.edges = edges.map((e) => ({ ...e, colorHash: colorHash[e.label.toString()] }));

    this.minNodeCount = this.nodes.slice(-1)[0].count;
    this.maxNodeCount = this.nodes[0].count;
    this.minEdgeCount = this.edges.slice(-1)[0].count;
    this.maxEdgeCount = this.edges[0].count;
  }
}

export const rawData = new RawData({{ data | json_encode(pretty = true) | safe }});
