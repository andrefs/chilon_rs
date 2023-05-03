import { SimulationLinkDatum, SimulationNodeDatum } from 'd3-force';
import { scaleLinear, scaleLog } from 'd3-scale';
import { genColorHash } from '../colors';

export interface RawNode extends SimulationNodeDatum {
  name: string;
  count: number;
  node_type: 'Namespace' | 'Unknown' | 'Blank'
};

export type SimNode = RawNode & {
  normCount: number,
  namespace: string,
  linScaleCount: number,
  logScaleCount: number,
  occursPerc: number
};

export interface RawEdge extends SimulationLinkDatum<RawNode> {
  source: string;
  target: string;
  label: string;
  is_datatype?: boolean;
  count: number;
  link_num: number;
};

interface SimEdge extends Omit<RawEdge, 'source' | 'target'> {
  source: SimNode,
  target: SimNode,
  namespace: string,
  normCount: number,
  colorHash: string
};

interface RawData {
  edges: RawEdge[];
  nodes: RawNode[];
  aliases: { [name: string]: string };
}

export class SimData {
  edges: SimEdge[];
  nodes: SimNode[];
  aliases: { [name: string]: string };

  minNodeCount: number;
  maxNodeCount: number;
  minEdgeCount: number;
  maxEdgeCount: number;

  totalNodeCount: number;

  constructor({ nodes, edges, aliases }: RawData) {
    this.aliases = aliases;

    const colorHash = genColorHash(edges);

    this.minNodeCount = nodes.slice(-1)[0].count;
    this.maxNodeCount = nodes[0].count;
    this.minEdgeCount = edges.slice(-1)[0].count;
    this.maxEdgeCount = edges[0].count;

    const scaleNodeLinear = scaleLinear().domain([this.minNodeCount, this.maxNodeCount]).range([10, 100]);
    const scaleNodeLog = scaleLog().domain([this.minNodeCount, this.maxNodeCount]).range([10, 100]);
    const scaleEdge = scaleLinear().domain([this.minEdgeCount, this.maxEdgeCount]).range([10, 100]);


    this.totalNodeCount = nodes.reduce((acc, cur) => acc + cur.count, 0);
    this.nodes = nodes.map((n) => ({
      ...n,
      namespace: aliases[n.name] || '',
      normCount: scaleNodeLinear(n.count), // default
      linScaleCount: scaleNodeLinear(n.count),
      logScaleCount: scaleNodeLog(n.count),
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
      namespace: aliases[e.label] || '',
      colorHash: colorHash[e.label.toString()],
      normCount: scaleEdge(e.count)
    }));
  }
}

export const initData = new SimData({
  "aliases": {
    "dbp": "http://dbpedia.org/property/",
    "dbr": "http://dbpedia.org/resource/",
    "ex": "http://example.org/",
    "xsd": "http://www.w3.org/2001/XMLSchema#",
    "yago": "http://dbpedia.org/class/yago/"
  },
  "edges": [
    {
      "count": 1,
      "is_datatype": true,
      "label": "dbp",
      "link_num": 1,
      "source": "dbr",
      "target": "xsd"
    }
  ],
  "nodes": [
    {
      "count": 1,
      "name": "dbr",
      "node_type": "Namespace"
    },
    {
      "count": 1,
      "name": "xsd",
      "node_type": "Namespace"
    }
  ]
});
