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
      "count": 4308772,
      "is_datatype": false,
      "label": "owl",
      "link_num": 1,
      "source": "db",
      "target": "yago"
    },
    {
      "count": 776980,
      "is_datatype": false,
      "label": "schema",
      "link_num": 1,
      "source": "db",
      "target": "viaf"
    },
    {
      "count": 596134,
      "is_datatype": false,
      "label": "rdf",
      "link_num": 1,
      "source": "db",
      "target": "umbelrc"
    },
    {
      "count": 430839,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "wn20"
    },
    {
      "count": 233106,
      "is_datatype": false,
      "label": "owl",
      "link_num": 1,
      "source": "db",
      "target": "gnd"
    },
    {
      "count": 173942,
      "is_datatype": false,
      "label": "owl",
      "link_num": 2,
      "source": "db",
      "target": "viaf"
    },
    {
      "count": 106498,
      "is_datatype": false,
      "label": "owl",
      "link_num": 1,
      "source": "db",
      "target": "lgd"
    },
    {
      "count": 95721,
      "is_datatype": false,
      "label": "owl",
      "link_num": -1,
      "source": "db",
      "target": "data4"
    },
    {
      "count": 86498,
      "is_datatype": false,
      "label": "owl",
      "link_num": 1,
      "source": "db",
      "target": "geodata"
    },
    {
      "count": 38786,
      "is_datatype": false,
      "label": "owl",
      "link_num": 1,
      "source": "db",
      "target": "gadm2"
    },
    {
      "count": 30426,
      "is_datatype": false,
      "label": "skos",
      "link_num": 1,
      "source": "db",
      "target": "www2"
    },
    {
      "count": 27504,
      "is_datatype": false,
      "label": "owl",
      "link_num": 1,
      "source": "db",
      "target": "linkedmdb"
    },
    {
      "count": 26940,
      "is_datatype": false,
      "label": "owl",
      "link_num": 1,
      "source": "db",
      "target": "sworg"
    },
    {
      "count": 24990,
      "is_datatype": false,
      "label": "owl",
      "link_num": 1,
      "source": "db",
      "target": "www4"
    },
    {
      "count": 17742,
      "is_datatype": false,
      "label": "owl",
      "link_num": -1,
      "source": "db",
      "target": "apiorg"
    },
    {
      "count": 12864,
      "is_datatype": false,
      "label": "owl",
      "link_num": 1,
      "source": "db",
      "target": "lod"
    },
    {
      "count": 12592,
      "is_datatype": false,
      "label": "owl",
      "link_num": 1,
      "source": "db",
      "target": "govtrackus"
    },
    {
      "count": 11198,
      "is_datatype": false,
      "label": "owl",
      "link_num": 1,
      "source": "db",
      "target": "eunis2"
    },
    {
      "count": 10089,
      "is_datatype": false,
      "label": "dcterms",
      "link_num": -1,
      "source": "db",
      "target": "data3"
    },
    {
      "count": 7687,
      "is_datatype": false,
      "label": "skos",
      "link_num": 1,
      "source": "db",
      "target": "globalwordnet"
    },
    {
      "count": 6861,
      "is_datatype": false,
      "label": "owl",
      "link_num": -1,
      "source": "db",
      "target": "dati"
    },
    {
      "count": 5822,
      "is_datatype": false,
      "label": "owl",
      "link_num": -1,
      "source": "db",
      "target": "data2"
    },
    {
      "count": 5793,
      "is_datatype": false,
      "label": "rdfs",
      "link_num": 1,
      "source": "db",
      "target": "lobid"
    },
    {
      "count": 5793,
      "is_datatype": false,
      "label": "rdrel",
      "link_num": 2,
      "source": "db",
      "target": "lobid"
    },
    {
      "count": 4998,
      "is_datatype": false,
      "label": "owl",
      "link_num": 1,
      "source": "db",
      "target": "wwwcom"
    },
    {
      "count": 4297,
      "is_datatype": false,
      "label": "rdf",
      "link_num": 1,
      "source": "db",
      "target": "db"
    },
    {
      "count": 3506,
      "is_datatype": false,
      "label": "umbel",
      "link_num": 3,
      "source": "db",
      "target": "lobid"
    },
    {
      "count": 3486,
      "is_datatype": false,
      "label": "rdf",
      "link_num": 1,
      "source": "db",
      "target": "foaf"
    },
    {
      "count": 3099,
      "is_datatype": false,
      "label": "owl",
      "link_num": 1,
      "source": "db",
      "target": "ecowlim"
    },
    {
      "count": 3048,
      "is_datatype": false,
      "label": "owl",
      "link_num": -1,
      "source": "db",
      "target": "UNKNOWN"
    },
    {
      "count": 2867,
      "is_datatype": false,
      "label": "skos",
      "link_num": 1,
      "source": "db",
      "target": "www"
    },
    {
      "count": 2612,
      "is_datatype": false,
      "label": "skos",
      "link_num": 1,
      "source": "db",
      "target": "zbw"
    },
    {
      "count": 2446,
      "is_datatype": false,
      "label": "dcterms",
      "link_num": 2,
      "source": "db",
      "target": "zbw"
    },
    {
      "count": 2339,
      "is_datatype": false,
      "label": "rdf",
      "link_num": 1,
      "source": "db",
      "target": "vocabcom"
    },
    {
      "count": 1521,
      "is_datatype": false,
      "label": "owl",
      "link_num": 1,
      "source": "db",
      "target": "purl2"
    },
    {
      "count": 1482,
      "is_datatype": false,
      "label": "owl",
      "link_num": -1,
      "source": "db",
      "target": "datatw"
    },
    {
      "count": 519,
      "is_datatype": false,
      "label": "skos",
      "link_num": -2,
      "source": "db",
      "target": "apiorg"
    },
    {
      "count": 312,
      "is_datatype": false,
      "label": "db",
      "link_num": 2,
      "source": "db",
      "target": "www4"
    },
    {
      "count": 307,
      "is_datatype": false,
      "label": "skos",
      "link_num": -2,
      "source": "db",
      "target": "UNKNOWN"
    },
    {
      "count": 214,
      "is_datatype": false,
      "label": "owl",
      "link_num": 1,
      "source": "db",
      "target": "wbc"
    }
  ],
  "nodes": [
    {
      "count": 7094927,
      "name": "db"
    },
    {
      "count": 4308772,
      "name": "yago"
    },
    {
      "count": 950922,
      "name": "viaf"
    },
    {
      "count": 596134,
      "name": "umbelrc"
    },
    {
      "count": 430839,
      "name": "wn20"
    },
    {
      "count": 233106,
      "name": "gnd"
    },
    {
      "count": 106498,
      "name": "lgd"
    },
    {
      "count": 95721,
      "name": "data4"
    },
    {
      "count": 86498,
      "name": "geodata"
    },
    {
      "count": 38786,
      "name": "gadm2"
    },
    {
      "count": 30426,
      "name": "www2"
    },
    {
      "count": 27504,
      "name": "linkedmdb"
    },
    {
      "count": 26940,
      "name": "sworg"
    },
    {
      "count": 25302,
      "name": "www4"
    },
    {
      "count": 18261,
      "name": "apiorg"
    },
    {
      "count": 15092,
      "name": "lobid"
    },
    {
      "count": 12864,
      "name": "lod"
    },
    {
      "count": 12592,
      "name": "govtrackus"
    },
    {
      "count": 11198,
      "name": "eunis2"
    },
    {
      "count": 10089,
      "name": "data3"
    },
    {
      "count": 7687,
      "name": "globalwordnet"
    },
    {
      "count": 6861,
      "name": "dati"
    },
    {
      "count": 5822,
      "name": "data2"
    },
    {
      "count": 5058,
      "name": "zbw"
    },
    {
      "count": 4998,
      "name": "wwwcom"
    },
    {
      "count": 3486,
      "name": "foaf"
    },
    {
      "count": 3355,
      "name": "UNKNOWN"
    },
    {
      "count": 3099,
      "name": "ecowlim"
    },
    {
      "count": 2867,
      "name": "www"
    },
    {
      "count": 2339,
      "name": "vocabcom"
    },
    {
      "count": 1521,
      "name": "purl2"
    },
    {
      "count": 1482,
      "name": "datatw"
    },
    {
      "count": 214,
      "name": "wbc"
    }
  ]
});
