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
    "dcterms": "http://purl.org/dc/terms/",
    "ili": "http://ili.globalwordnet.org/ili/",
    "ontolex": "http://www.w3.org/ns/lemon/ontolex#",
    "owl": "http://www.w3.org/2002/07/owl#",
    "rdf": "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
    "rdfs": "http://www.w3.org/2000/01/rdf-schema#",
    "wordn2": "http://wordnet-rdf.princeton.edu/id/",
    "wordnet": "http://wordnet-rdf.princeton.edu/ontology#",
    "wordnet-rdf": "http://wordnet-rdf.princeton.edu/rdf/lemma/",
    "www": "http://www.w3.org/ns/lemon/synsem#",
    "xsd": "http://www.w3.org/TR/xmlschema11-2/"
  },
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
      "target": "BLANK"
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
      "link_num": 1,
      "source": "wordnet-rdf",
      "target": "wordnet-rdf"
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
      "count": 159015,
      "is_datatype": false,
      "label": "wordnet",
      "link_num": -1,
      "source": "wordnet-rdf",
      "target": "wordnet"
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
      "label": "rdf",
      "link_num": -1,
      "source": "wordn2",
      "target": "ontolex"
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
      "is_datatype": false,
      "label": "wordnet",
      "link_num": 1,
      "source": "wordn2",
      "target": "wordnet"
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
      "count": 98923,
      "is_datatype": false,
      "label": "www",
      "link_num": -2,
      "source": "wordnet-rdf",
      "target": "BLANK"
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
      "name": "wordnet-rdf",
      "node_type": "Namespace"
    },
    {
      "count": 1367563,
      "name": "wordn2",
      "node_type": "Namespace"
    },
    {
      "count": 847972,
      "name": "BLANK",
      "node_type": "Blank"
    },
    {
      "count": 484078,
      "name": "ontolex",
      "node_type": "Namespace"
    },
    {
      "count": 423986,
      "name": "rdf",
      "node_type": "Namespace"
    },
    {
      "count": 276806,
      "name": "wordnet",
      "node_type": "Namespace"
    },
    {
      "count": 117791,
      "name": "ili",
      "node_type": "Namespace"
    },
    {
      "count": 117791,
      "name": "xsd",
      "node_type": "Namespace"
    }
  ]
});
