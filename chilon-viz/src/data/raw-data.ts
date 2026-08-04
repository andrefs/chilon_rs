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
    "aksw": "http://aksw.org/",
    "beth": "http://www.google.com/",
    "dbkwik": "http://dbkwik.webdatacommons.org/",
    "dcterms": "http://purl.org/dc/terms/",
    "eg": "http://www.example.org/",
    "foaf": "http://xmlns.com/foaf/0.1/",
    "meat": "http://example.com/",
    "owl": "http://www.w3.org/2002/07/owl#",
    "purl": "http://www.purl.org/",
    "rdf": "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
    "rdfs": "http://www.w3.org/2000/01/rdf-schema#",
    "vam": "http://www.metmuseum.org/",
    "weki": "https://en.wikipedia.org/wiki/",
    "wiki": "http://en.wikipedia.org/wiki/",
    "xsd": "http://www.w3.org/2001/XMLSchema#"
  },
  "edges": [
    {
      "count": 40957805,
      "is_datatype": false,
      "label": "dbkwik",
      "link_num": 1,
      "source": "dbkwik",
      "target": "dbkwik"
    },
    {
      "count": 31798137,
      "is_datatype": true,
      "label": "dbkwik",
      "link_num": 1,
      "source": "dbkwik",
      "target": "rdf"
    },
    {
      "count": 22680984,
      "is_datatype": false,
      "label": "dcterms",
      "link_num": 2,
      "source": "dbkwik",
      "target": "dbkwik"
    },
    {
      "count": 18734926,
      "is_datatype": true,
      "label": "rdfs",
      "link_num": 2,
      "source": "dbkwik",
      "target": "rdf"
    },
    {
      "count": 9787954,
      "is_datatype": true,
      "label": "dbkwik",
      "link_num": 1,
      "source": "dbkwik",
      "target": "xsd"
    },
    {
      "count": 1676522,
      "is_datatype": false,
      "label": "rdf",
      "link_num": 3,
      "source": "dbkwik",
      "target": "dbkwik"
    },
    {
      "count": 647177,
      "is_datatype": true,
      "label": "dbkwik",
      "link_num": 4,
      "source": "dbkwik",
      "target": "dbkwik"
    },
    {
      "count": 128566,
      "is_datatype": false,
      "label": "rdf",
      "link_num": 3,
      "source": "dbkwik",
      "target": "rdf"
    },
    {
      "count": 111068,
      "is_datatype": false,
      "label": "rdfs",
      "link_num": 1,
      "source": "dbkwik",
      "target": "rdfs"
    },
    {
      "count": 80912,
      "is_datatype": false,
      "label": "foaf",
      "link_num": -1,
      "source": "dbkwik",
      "target": "UNKNOWN"
    },
    {
      "count": 67275,
      "is_datatype": false,
      "label": "rdfs",
      "link_num": 5,
      "source": "dbkwik",
      "target": "dbkwik"
    },
    {
      "count": 12029,
      "is_datatype": false,
      "label": "rdf",
      "link_num": 1,
      "source": "dbkwik",
      "target": "owl"
    },
    {
      "count": 34,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 1,
      "source": "dbkwik",
      "target": "meat"
    },
    {
      "count": 23,
      "is_datatype": false,
      "label": "foaf",
      "link_num": -1,
      "source": "dbkwik",
      "target": "beth"
    },
    {
      "count": 20,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 1,
      "source": "dbkwik",
      "target": "wiki"
    }
  ],
  "nodes": [
    {
      "count": 192713195,
      "name": "dbkwik",
      "node_type": "Namespace"
    },
    {
      "count": 50661629,
      "name": "rdf",
      "node_type": "Namespace"
    },
    {
      "count": 9787954,
      "name": "xsd",
      "node_type": "Namespace"
    },
    {
      "count": 111068,
      "name": "rdfs",
      "node_type": "Namespace"
    },
    {
      "count": 80912,
      "name": "UNKNOWN",
      "node_type": "Unknown"
    },
    {
      "count": 12029,
      "name": "owl",
      "node_type": "Namespace"
    },
    {
      "count": 34,
      "name": "meat",
      "node_type": "Namespace"
    },
    {
      "count": 23,
      "name": "beth",
      "node_type": "Namespace"
    },
    {
      "count": 20,
      "name": "wiki",
      "node_type": "Namespace"
    }
  ]
});
