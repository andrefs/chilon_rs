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

export const initData = new SimData({
  "edges": [
    {
      "count": 471214,
      "label": "skos",
      "link_num": -1,
      "source": "kbpedia",
      "target": "STRING@en"
    },
    {
      "count": 141685,
      "label": "rdfs",
      "link_num": 1,
      "source": "kbpedia",
      "target": "kbpedia"
    },
    {
      "count": 78899,
      "label": "kko",
      "link_num": 2,
      "source": "kbpedia",
      "target": "kbpedia"
    },
    {
      "count": 62786,
      "label": "rdf",
      "link_num": 1,
      "source": "kbpedia",
      "target": "owl"
    },
    {
      "count": 45564,
      "label": "kko",
      "link_num": -1,
      "source": "wd",
      "target": "kbpedia"
    },
    {
      "count": 44965,
      "label": "rdfs",
      "link_num": -2,
      "source": "wd",
      "target": "kbpedia"
    },
    {
      "count": 44965,
      "label": "kko",
      "link_num": 3,
      "source": "kbpedia",
      "target": "wd"
    },
    {
      "count": 44905,
      "label": "rdfs",
      "link_num": 4,
      "source": "kbpedia",
      "target": "wd"
    },
    {
      "count": 44432,
      "label": "owl",
      "link_num": -5,
      "source": "wd",
      "target": "kbpedia"
    },
    {
      "count": 33036,
      "label": "kko",
      "link_num": -1,
      "source": "wikipno",
      "target": "kbpedia"
    },
    {
      "count": 32398,
      "label": "rdfs",
      "link_num": 2,
      "source": "kbpedia",
      "target": "wikipno"
    },
    {
      "count": 31892,
      "label": "rdfs",
      "link_num": -3,
      "source": "wikipno",
      "target": "kbpedia"
    },
    {
      "count": 31892,
      "label": "kko",
      "link_num": 4,
      "source": "kbpedia",
      "target": "wikipno"
    },
    {
      "count": 31447,
      "label": "owl",
      "link_num": -5,
      "source": "wikipno",
      "target": "kbpedia"
    },
    {
      "count": 5194,
      "label": "rdfs",
      "link_num": 1,
      "source": "kbpedia",
      "target": "kko"
    },
    {
      "count": 4101,
      "label": "kko",
      "link_num": 1,
      "source": "UNKNOWN",
      "target": "kbpedia"
    },
    {
      "count": 4023,
      "label": "rdfs",
      "link_num": -2,
      "source": "kbpedia",
      "target": "UNKNOWN"
    },
    {
      "count": 3953,
      "label": "kko",
      "link_num": -3,
      "source": "kbpedia",
      "target": "UNKNOWN"
    },
    {
      "count": 3953,
      "label": "rdfs",
      "link_num": 4,
      "source": "UNKNOWN",
      "target": "kbpedia"
    },
    {
      "count": 3910,
      "label": "owl",
      "link_num": 5,
      "source": "UNKNOWN",
      "target": "kbpedia"
    },
    {
      "count": 1137,
      "label": "rdfs",
      "link_num": 2,
      "source": "kbpedia",
      "target": "owl"
    },
    {
      "count": 918,
      "label": "kko",
      "link_num": -1,
      "source": "kbpedia",
      "target": "geonames"
    },
    {
      "count": 918,
      "label": "rdfs",
      "link_num": 2,
      "source": "geonames",
      "target": "kbpedia"
    },
    {
      "count": 890,
      "label": "kko",
      "link_num": -1,
      "source": "wikiCategory",
      "target": "kbpedia"
    },
    {
      "count": 886,
      "label": "rdfs",
      "link_num": 2,
      "source": "kbpedia",
      "target": "wikiCategory"
    },
    {
      "count": 879,
      "label": "kko",
      "link_num": 3,
      "source": "kbpedia",
      "target": "wikiCategory"
    },
    {
      "count": 879,
      "label": "rdfs",
      "link_num": -4,
      "source": "wikiCategory",
      "target": "kbpedia"
    },
    {
      "count": 875,
      "label": "owl",
      "link_num": -5,
      "source": "wikiCategory",
      "target": "kbpedia"
    },
    {
      "count": 769,
      "label": "rdfs",
      "link_num": -1,
      "source": "schema",
      "target": "kbpedia"
    },
    {
      "count": 769,
      "label": "kko",
      "link_num": 2,
      "source": "kbpedia",
      "target": "schema"
    },
    {
      "count": 753,
      "label": "kko",
      "link_num": -1,
      "source": "kbpedia",
      "target": "db"
    },
    {
      "count": 753,
      "label": "rdfs",
      "link_num": 2,
      "source": "db",
      "target": "kbpedia"
    },
    {
      "count": 530,
      "label": "rdfs",
      "link_num": -3,
      "source": "kbpedia",
      "target": "db"
    },
    {
      "count": 530,
      "label": "kko",
      "link_num": 4,
      "source": "db",
      "target": "kbpedia"
    },
    {
      "count": 522,
      "label": "owl",
      "link_num": 5,
      "source": "db",
      "target": "kbpedia"
    },
    {
      "count": 506,
      "label": "kko",
      "link_num": -3,
      "source": "schema",
      "target": "kbpedia"
    },
    {
      "count": 506,
      "label": "rdfs",
      "link_num": 4,
      "source": "kbpedia",
      "target": "schema"
    },
    {
      "count": 442,
      "label": "owl",
      "link_num": -3,
      "source": "kbpedia",
      "target": "geonames"
    },
    {
      "count": 442,
      "label": "rdfs",
      "link_num": -4,
      "source": "kbpedia",
      "target": "geonames"
    },
    {
      "count": 442,
      "label": "kko",
      "link_num": 5,
      "source": "geonames",
      "target": "kbpedia"
    },
    {
      "count": 437,
      "label": "owl",
      "link_num": -5,
      "source": "schema",
      "target": "kbpedia"
    },
    {
      "count": 141,
      "label": "kko",
      "link_num": -2,
      "source": "kko",
      "target": "kbpedia"
    },
    {
      "count": 27,
      "label": "kko",
      "link_num": -1,
      "source": "kbpedia",
      "target": "foaf"
    },
    {
      "count": 27,
      "label": "rdfs",
      "link_num": 2,
      "source": "foaf",
      "target": "kbpedia"
    },
    {
      "count": 25,
      "label": "kko",
      "link_num": 1,
      "source": "UNKNOWN",
      "target": "frbr"
    },
    {
      "count": 23,
      "label": "rdfs",
      "link_num": -2,
      "source": "frbr",
      "target": "UNKNOWN"
    },
    {
      "count": 20,
      "label": "rdfs",
      "link_num": -3,
      "source": "kbpedia",
      "target": "foaf"
    },
    {
      "count": 20,
      "label": "kko",
      "link_num": 4,
      "source": "foaf",
      "target": "kbpedia"
    },
    {
      "count": 20,
      "label": "kko",
      "link_num": -1,
      "source": "kbpedia",
      "target": "bibo"
    },
    {
      "count": 20,
      "label": "rdfs",
      "link_num": 2,
      "source": "bibo",
      "target": "kbpedia"
    },
    {
      "count": 19,
      "label": "kko",
      "link_num": -1,
      "source": "kbpedia",
      "target": "dct"
    },
    {
      "count": 19,
      "label": "rdfs",
      "link_num": 2,
      "source": "dct",
      "target": "kbpedia"
    },
    {
      "count": 19,
      "label": "kko",
      "link_num": 3,
      "source": "bibo",
      "target": "kbpedia"
    },
    {
      "count": 19,
      "label": "owl",
      "link_num": -4,
      "source": "kbpedia",
      "target": "bibo"
    },
    {
      "count": 19,
      "label": "rdfs",
      "link_num": -5,
      "source": "kbpedia",
      "target": "bibo"
    },
    {
      "count": 18,
      "label": "owl",
      "link_num": 5,
      "source": "foaf",
      "target": "kbpedia"
    },
    {
      "count": 14,
      "label": "rdfs",
      "link_num": 1,
      "source": "dcm",
      "target": "kbpedia"
    },
    {
      "count": 14,
      "label": "kko",
      "link_num": -2,
      "source": "kbpedia",
      "target": "dcm"
    },
    {
      "count": 14,
      "label": "kko",
      "link_num": -1,
      "source": "kbpedia",
      "target": "dc"
    },
    {
      "count": 14,
      "label": "rdfs",
      "link_num": 2,
      "source": "dc",
      "target": "kbpedia"
    },
    {
      "count": 12,
      "label": "kko",
      "link_num": 1,
      "source": "kko",
      "target": "wd"
    },
    {
      "count": 12,
      "label": "rdfs",
      "link_num": -2,
      "source": "wd",
      "target": "kko"
    },
    {
      "count": 12,
      "label": "owl",
      "link_num": -1,
      "source": "UNKNOWN",
      "target": "STRING@en"
    },
    {
      "count": 10,
      "label": "kko",
      "link_num": -1,
      "source": "mo",
      "target": "kbpedia"
    },
    {
      "count": 10,
      "label": "owl",
      "link_num": 2,
      "source": "kbpedia",
      "target": "mo"
    },
    {
      "count": 10,
      "label": "rdfs",
      "link_num": -3,
      "source": "mo",
      "target": "kbpedia"
    },
    {
      "count": 10,
      "label": "rdfs",
      "link_num": 4,
      "source": "kbpedia",
      "target": "mo"
    },
    {
      "count": 10,
      "label": "kko",
      "link_num": 5,
      "source": "kbpedia",
      "target": "mo"
    }
  ],
  "nodes": [
    {
      "count": 1401075,
      "name": "kbpedia"
    },
    {
      "count": 471226,
      "name": "STRING@en"
    },
    {
      "count": 224855,
      "name": "wd"
    },
    {
      "count": 160665,
      "name": "wikipno"
    },
    {
      "count": 63923,
      "name": "owl"
    },
    {
      "count": 20000,
      "name": "UNKNOWN"
    },
    {
      "count": 5359,
      "name": "kko"
    },
    {
      "count": 4409,
      "name": "wikiCategory"
    },
    {
      "count": 3162,
      "name": "geonames"
    },
    {
      "count": 3088,
      "name": "db"
    },
    {
      "count": 2987,
      "name": "schema"
    },
    {
      "count": 112,
      "name": "foaf"
    },
    {
      "count": 97,
      "name": "bibo"
    },
    {
      "count": 50,
      "name": "mo"
    },
    {
      "count": 48,
      "name": "frbr"
    },
    {
      "count": 38,
      "name": "dct"
    },
    {
      "count": 28,
      "name": "dc"
    },
    {
      "count": 28,
      "name": "dcm"
    }
  ]
});
