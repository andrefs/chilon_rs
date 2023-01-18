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

export const initData = new SimData({
  "edges": [
    {
      "count": 366287,
      "label": "rdf",
      "link_num": 1,
      "source": 0,
      "target": 1
    },
    {
      "count": 285668,
      "label": "wordnet",
      "link_num": 1,
      "source": 2,
      "target": 2
    },
    {
      "count": 207272,
      "label": "ontolex",
      "link_num": 1,
      "source": 0,
      "target": 2
    },
    {
      "count": 207272,
      "label": "ontolex",
      "link_num": 2,
      "source": 0,
      "target": 0
    },
    {
      "count": 207272,
      "label": "ontolex",
      "link_num": 3,
      "source": 3,
      "target": 4
    },
    {
      "count": 207272,
      "label": "ontolex",
      "link_num": 2,
      "source": 0,
      "target": 3
    },
    {
      "count": 159015,
      "label": "wordnet",
      "link_num": 1,
      "source": 0,
      "target": 5
    },
    {
      "count": 117791,
      "label": "owl",
      "link_num": 1,
      "source": 2,
      "target": 6
    },
    {
      "count": 117791,
      "label": "rdf",
      "link_num": 1,
      "source": 2,
      "target": 1
    },
    {
      "count": 117791,
      "label": "wordnet",
      "link_num": 1,
      "source": 2,
      "target": 3
    },
    {
      "count": 117791,
      "label": "rdf",
      "link_num": 3,
      "source": 3,
      "target": 4
    },
    {
      "count": 117791,
      "label": "wordnet",
      "link_num": 1,
      "source": 2,
      "target": 5
    },
    {
      "count": 117791,
      "label": "terms",
      "link_num": 1,
      "source": 2,
      "target": 7
    },
    {
      "count": 98923,
      "label": "rdfs",
      "link_num": 3,
      "source": 3,
      "target": 4
    },
    {
      "count": 98923,
      "label": "www",
      "link_num": 2,
      "source": 0,
      "target": 3
    },
    {
      "count": 92518,
      "label": "wordnet",
      "link_num": 2,
      "source": 0,
      "target": 0
    }
  ],
  "nodes": [
    {
      "count": 1638349,
      "id": 0,
      "name": "wordnet-rdf"
    },
    {
      "count": 1367563,
      "id": 2,
      "name": "wordn2"
    },
    {
      "count": 847972,
      "id": 3,
      "name": "BLANK"
    },
    {
      "count": 484078,
      "id": 1,
      "name": "ontolex"
    },
    {
      "count": 423986,
      "id": 4,
      "name": "LANG-STRING"
    },
    {
      "count": 276806,
      "id": 5,
      "name": "wordnet"
    },
    {
      "count": 117791,
      "id": 6,
      "name": "ili"
    },
    {
      "count": 117791,
      "id": 7,
      "name": "STRING"
    }
  ]
});
