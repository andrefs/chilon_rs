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
  is_datatype?: boolean;
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
      "count": 366287,
      "is_datatype": false,
      "label": "rdf",
      "link_num": -1,
      "source": "wordnet-rdf",
      "target": "ontolex"
    },
    {
      "count": 285668,
      "is_datatype": false,
      "label": "wordnet",
      "link_num": 1,
      "source": "wordn2",
      "target": "wordn2"
    },
    {
      "count": 207272,
      "is_datatype": false,
      "label": "ontolex",
      "link_num": -1,
      "source": "wordnet-rdf",
      "target": "wordn2"
    },
    {
      "count": 207272,
      "is_datatype": true,
      "label": "ontolex",
      "link_num": 1,
      "source": "BLANK",
      "target": "rdf"
    },
    {
      "count": 207272,
      "is_datatype": false,
      "label": "ontolex",
      "link_num": -1,
      "source": "wordnet-rdf",
      "target": "BLANK"
    },
    {
      "count": 207272,
      "is_datatype": false,
      "label": "ontolex",
      "link_num": 1,
      "source": "wordnet-rdf",
      "target": "wordnet-rdf"
    },
    {
      "count": 159015,
      "is_datatype": false,
      "label": "wordnet",
      "link_num": -1,
      "source": "wordnet-rdf",
      "target": "wordnet"
    },
    {
      "count": 117791,
      "is_datatype": true,
      "label": "rdf",
      "link_num": 2,
      "source": "BLANK",
      "target": "rdf"
    },
    {
      "count": 117791,
      "is_datatype": false,
      "label": "owl",
      "link_num": -1,
      "source": "wordn2",
      "target": "ili"
    },
    {
      "count": 117791,
      "is_datatype": true,
      "label": "dcterms",
      "link_num": 1,
      "source": "wordn2",
      "target": "xsd"
    },
    {
      "count": 117791,
      "is_datatype": false,
      "label": "rdf",
      "link_num": -1,
      "source": "wordn2",
      "target": "ontolex"
    },
    {
      "count": 117791,
      "is_datatype": false,
      "label": "wordnet",
      "link_num": -1,
      "source": "wordn2",
      "target": "BLANK"
    },
    {
      "count": 117791,
      "is_datatype": false,
      "label": "wordnet",
      "link_num": 1,
      "source": "wordn2",
      "target": "wordnet"
    },
    {
      "count": 98923,
      "is_datatype": true,
      "label": "rdfs",
      "link_num": 3,
      "source": "BLANK",
      "target": "rdf"
    },
    {
      "count": 98923,
      "is_datatype": false,
      "label": "www",
      "link_num": -2,
      "source": "wordnet-rdf",
      "target": "BLANK"
    },
    {
      "count": 92518,
      "is_datatype": false,
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
      "name": "rdf"
    },
    {
      "count": 276806,
      "name": "wordnet"
    },
    {
      "count": 117791,
      "name": "ili"
    },
    {
      "count": 117791,
      "name": "xsd"
    }
  ]
});
