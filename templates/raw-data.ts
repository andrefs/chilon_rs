import { SimulationLinkDatum, SimulationNodeDatum } from 'd3-force';
import { scaleLinear } from 'd3-scale';
import { genColorHash } from '../colors';

export interface RawNode extends SimulationNodeDatum {
  name: string;
  count: number;
};

export type SimNode = RawNode & {
  normCount: number,
  occursPerc: number
};

export interface RawEdge extends SimulationLinkDatum<RawNode> {
  source: string;
  target: string;
  label: string;
  count: number;
  link_num: number;
};

interface SimEdge extends Omit<RawEdge, 'source' | 'target'> {
  source: SimNode,
  target: SimNode,
  normCount: number,
  colorHash: string
};

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

  totalNodeCount: number;

  constructor({ nodes, edges }: RawData) {
    const colorHash = genColorHash(edges);

    this.minNodeCount = nodes.slice(-1)[0].count;
    this.maxNodeCount = nodes[0].count;
    this.minEdgeCount = edges.slice(-1)[0].count;
    this.maxEdgeCount = edges[0].count;

    const scaleNode = scaleLinear().domain([this.minNodeCount, this.maxNodeCount]).range([10, 100]);
    const scaleEdge = scaleLinear().domain([this.minEdgeCount, this.maxEdgeCount]).range([10, 100]);


    this.totalNodeCount = nodes.reduce((acc, cur) => acc + cur.count, 0);
    this.nodes = nodes.map((n) => ({
      ...n,
      normCount: scaleNode(n.count),
      occursPerc: n.count / this.totalNodeCount
    }));


    let namesToNodes: { [name: string]: SimNode } = {};
    for (const node of this.nodes) {
      namesToNodes[node.name] = node;
    }

    this.edges = edges.map((e) => ({
      ...e,
      source: namesToNodes[e.source],
      target: namesToNodes[e.target],
      colorHash: colorHash[e.label.toString()],
      normCount: scaleEdge(e.count)
    }));
  }
}

export const initData = new SimData({{ data | json_encode(pretty = true) | safe }});
