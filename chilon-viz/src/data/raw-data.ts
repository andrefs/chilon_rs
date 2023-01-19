import { SimulationLinkDatum, SimulationNodeDatum } from 'd3-force';
import { scaleLinear } from 'd3-scale';
import { genColorHash } from '../colors';

export interface RawNode extends SimulationNodeDatum {
  name: string;
  count: number;
};

export type SimNode = RawNode & { normCount: number };

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
      "count": 366287,
      "label": "rdf",
      "link_num": -1,
      "source": "wordnet-rdf",
      "target": "ontolex"
    },
    {
      "count": 285668,
      "label": "wordnet",
      "link_num": 1,
      "source": "wordn2",
      "target": "wordn2"
    },
    {
      "count": 207272,
      "label": "ontolex",
      "link_num": 1,
      "source": "BLANK",
      "target": "LANG-STRING"
    },
    {
      "count": 207272,
      "label": "ontolex",
      "link_num": 1,
      "source": "wordnet-rdf",
      "target": "wordnet-rdf"
    },
    {
      "count": 207272,
      "label": "ontolex",
      "link_num": -1,
      "source": "wordnet-rdf",
      "target": "wordn2"
    },
    {
      "count": 207272,
      "label": "ontolex",
      "link_num": -1,
      "source": "wordnet-rdf",
      "target": "BLANK"
    },
    {
      "count": 159015,
      "label": "wordnet",
      "link_num": -1,
      "source": "wordnet-rdf",
      "target": "wordnet"
    },
    {
      "count": 117791,
      "label": "rdf",
      "link_num": 2,
      "source": "BLANK",
      "target": "LANG-STRING"
    },
    {
      "count": 117791,
      "label": "rdf",
      "link_num": -1,
      "source": "wordn2",
      "target": "ontolex"
    },
    {
      "count": 117791,
      "label": "wordnet",
      "link_num": -1,
      "source": "wordn2",
      "target": "BLANK"
    },
    {
      "count": 117791,
      "label": "owl",
      "link_num": -1,
      "source": "wordn2",
      "target": "ili"
    },
    {
      "count": 117791,
      "label": "terms",
      "link_num": -1,
      "source": "wordn2",
      "target": "STRING"
    },
    {
      "count": 117791,
      "label": "wordnet",
      "link_num": 1,
      "source": "wordn2",
      "target": "wordnet"
    },
    {
      "count": 98923,
      "label": "rdfs",
      "link_num": 3,
      "source": "BLANK",
      "target": "LANG-STRING"
    },
    {
      "count": 98923,
      "label": "www",
      "link_num": -2,
      "source": "wordnet-rdf",
      "target": "BLANK"
    },
    {
      "count": 92518,
      "label": "wordnet",
      "link_num": 2,
      "source": "wordnet-rdf",
      "target": "wordnet-rdf"
    }
  ],
  "nodes": [
    {
      "count": 1638349,
      "name": "wordnet-rdf"
    },
    {
      "count": 1367563,
      "name": "wordn2"
    },
    {
      "count": 847972,
      "name": "BLANK"
    },
    {
      "count": 484078,
      "name": "ontolex"
    },
    {
      "count": 423986,
      "name": "LANG-STRING"
    },
    {
      "count": 276806,
      "name": "wordnet"
    },
    {
      "count": 117791,
      "name": "STRING"
    },
    {
      "count": 117791,
      "name": "ili"
    }
  ]
});
