import { SimulationLinkDatum, SimulationNodeDatum } from 'd3-force';
import { scaleLinear } from 'd3-scale';
import { genColorHash } from '../colors';

export interface RawNode extends SimulationNodeDatum {
  id: number;
  name: String;
  count: number;
};

export type SimNode = RawNode & { normCount: number };


export interface RawEdge extends SimulationLinkDatum<RawNode> {
  source: number;
  target: number;
  label: String;
  count: number;
  link_num: number;
};

export type SimEdge = RawEdge & { normCount: number, colorHash: string };

interface RawData {
  edges: RawEdge[];
  nodes: RawNode[];
}

export class SimData {
  edges: SimEdge[];
  nodes: SimNode[];

  minNodeCount: number;
  maxNodeCount: number;
  minEdgeCount: number;
  maxEdgeCount: number;

  constructor({ nodes, edges }: RawData) {

    const colorHash = genColorHash(edges);

    this.minNodeCount = nodes.slice(-1)[0].count;
    this.maxNodeCount = nodes[0].count;
    this.minEdgeCount = edges.slice(-1)[0].count;
    this.maxEdgeCount = edges[0].count;

    const scaleNode = scaleLinear().domain([this.minNodeCount, this.maxNodeCount]).range([10, 100]);
    const scaleEdge = scaleLinear().domain([this.minEdgeCount, this.maxEdgeCount]).range([10, 100]);

    this.nodes = nodes.map((n) => ({
      ...n,
      normCount: scaleNode(n.count)
    }));

    this.edges = edges.map((e) => ({
      ...e,
      colorHash: colorHash[e.label.toString()],
      normCount: scaleEdge(e.count)
    }));

  }
}

export const initData = new SimData({{ data | json_encode(pretty = true) | safe }});
