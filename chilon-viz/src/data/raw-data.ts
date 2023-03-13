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
  "aliases": {},
  "edges": [
    {
      "count": 386302435,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "db"
    },
    {
      "count": 107017645,
      "is_datatype": true,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "xsd"
    },
    {
      "count": 87609876,
      "is_datatype": false,
      "label": "rdf",
      "link_num": 2,
      "source": "db",
      "target": "db"
    },
    {
      "count": 54885030,
      "is_datatype": true,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "rdf"
    },
    {
      "count": 35204254,
      "is_datatype": false,
      "label": "dcterms",
      "link_num": 3,
      "source": "db",
      "target": "db"
    },
    {
      "count": 25448768,
      "is_datatype": true,
      "label": "rdfs",
      "link_num": 2,
      "source": "db",
      "target": "rdf"
    },
    {
      "count": 20617024,
      "is_datatype": false,
      "label": "prov",
      "link_num": 1,
      "source": "db",
      "target": "wiki"
    },
    {
      "count": 16750551,
      "is_datatype": false,
      "label": "rdf",
      "link_num": -1,
      "source": "wiki",
      "target": "foaf"
    },
    {
      "count": 16750551,
      "is_datatype": true,
      "label": "dc",
      "link_num": 1,
      "source": "wiki",
      "target": "xsd"
    },
    {
      "count": 16750551,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "wiki"
    },
    {
      "count": 16750551,
      "is_datatype": false,
      "label": "foaf",
      "link_num": -3,
      "source": "wiki",
      "target": "db"
    },
    {
      "count": 12865387,
      "is_datatype": false,
      "label": "rdf",
      "link_num": 1,
      "source": "db",
      "target": "wd"
    },
    {
      "count": 11894660,
      "is_datatype": false,
      "label": "rdf",
      "link_num": 1,
      "source": "commo4",
      "target": "db"
    },
    {
      "count": 11894660,
      "is_datatype": false,
      "label": "dc",
      "link_num": 1,
      "source": "commo4",
      "target": "wiki"
    },
    {
      "count": 10169314,
      "is_datatype": false,
      "label": "owl",
      "link_num": 1,
      "source": "db",
      "target": "freebase"
    },
    {
      "count": 8717177,
      "is_datatype": false,
      "label": "foaf",
      "link_num": -2,
      "source": "db",
      "target": "commo4"
    },
    {
      "count": 6062767,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "UNKNOWN"
    },
    {
      "count": 5947330,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 1,
      "source": "commo4",
      "target": "commo4"
    },
    {
      "count": 5425496,
      "is_datatype": false,
      "label": "rdf",
      "link_num": 1,
      "source": "db",
      "target": "owl"
    },
    {
      "count": 4877667,
      "is_datatype": false,
      "label": "owl",
      "link_num": 1,
      "source": "db",
      "target": "yago"
    },
    {
      "count": 4817830,
      "is_datatype": false,
      "label": "rdf",
      "link_num": 1,
      "source": "db",
      "target": "schema"
    },
    {
      "count": 4770287,
      "is_datatype": false,
      "label": "rdf",
      "link_num": 1,
      "source": "db",
      "target": "dul"
    },
    {
      "count": 4516936,
      "is_datatype": false,
      "label": "skos",
      "link_num": 4,
      "source": "db",
      "target": "db"
    },
    {
      "count": 4421697,
      "is_datatype": true,
      "label": "db",
      "link_num": 5,
      "source": "db",
      "target": "db"
    },
    {
      "count": 4265005,
      "is_datatype": true,
      "label": "foaf",
      "link_num": 3,
      "source": "db",
      "target": "rdf"
    },
    {
      "count": 4013670,
      "is_datatype": false,
      "label": "purl2",
      "link_num": 6,
      "source": "db",
      "target": "db"
    },
    {
      "count": 3785974,
      "is_datatype": true,
      "label": "wgs84",
      "link_num": 2,
      "source": "db",
      "target": "xsd"
    },
    {
      "count": 3121813,
      "is_datatype": false,
      "label": "db",
      "link_num": -3,
      "source": "db",
      "target": "commo4"
    },
    {
      "count": 2193008,
      "is_datatype": false,
      "label": "rdf",
      "link_num": 1,
      "source": "db",
      "target": "skos"
    },
    {
      "count": 2193008,
      "is_datatype": true,
      "label": "skos",
      "link_num": 4,
      "source": "db",
      "target": "rdf"
    },
    {
      "count": 1892992,
      "is_datatype": true,
      "label": "georss",
      "link_num": 3,
      "source": "db",
      "target": "xsd"
    },
    {
      "count": 1892617,
      "is_datatype": false,
      "label": "rdf",
      "link_num": 1,
      "source": "db",
      "target": "wgs84"
    },
    {
      "count": 1849993,
      "is_datatype": false,
      "label": "rdf",
      "link_num": 1,
      "source": "db",
      "target": "foaf"
    },
    {
      "count": 1288290,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "web2"
    },
    {
      "count": 1006375,
      "is_datatype": false,
      "label": "foaf",
      "link_num": -2,
      "source": "db",
      "target": "UNKNOWN"
    },
    {
      "count": 861372,
      "is_datatype": false,
      "label": "rdfs",
      "link_num": 7,
      "source": "db",
      "target": "db"
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
      "count": 357685,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "books"
    },
    {
      "count": 309915,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "archi2"
    },
    {
      "count": 233851,
      "is_datatype": false,
      "label": "owl",
      "link_num": 1,
      "source": "db",
      "target": "gnd"
    },
    {
      "count": 175012,
      "is_datatype": false,
      "label": "owl",
      "link_num": 2,
      "source": "db",
      "target": "viaf"
    },
    {
      "count": 130303,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "int"
    },
    {
      "count": 123826,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www50"
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
      "count": 96476,
      "is_datatype": false,
      "label": "owl",
      "link_num": 8,
      "source": "db",
      "target": "db"
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
      "count": 90374,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www71"
    },
    {
      "count": 89678,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www67"
    },
    {
      "count": 87071,
      "is_datatype": false,
      "label": "owl",
      "link_num": 1,
      "source": "db",
      "target": "geodata"
    },
    {
      "count": 71567,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www73"
    },
    {
      "count": 61386,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "archi3"
    },
    {
      "count": 60958,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "web2"
    },
    {
      "count": 60271,
      "is_datatype": false,
      "label": "owl",
      "link_num": -1,
      "source": "db",
      "target": "commo2"
    },
    {
      "count": 56395,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www62"
    },
    {
      "count": 53247,
      "is_datatype": false,
      "label": "rdf",
      "link_num": 5,
      "source": "db",
      "target": "rdf"
    },
    {
      "count": 51134,
      "is_datatype": true,
      "label": "dc",
      "link_num": 4,
      "source": "db",
      "target": "xsd"
    },
    {
      "count": 50628,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "newscom"
    },
    {
      "count": 48752,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www146"
    },
    {
      "count": 48160,
      "is_datatype": false,
      "label": "rdf",
      "link_num": -1,
      "source": "db",
      "target": "bibo"
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
      "target": "www74"
    },
    {
      "count": 27608,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www138"
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
      "count": 27339,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www133"
    },
    {
      "count": 27024,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "cricketarchive"
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
      "count": 26927,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www72"
    },
    {
      "count": 26593,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www65"
    },
    {
      "count": 25996,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "rugby"
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
      "count": 24892,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www144"
    },
    {
      "count": 22724,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www63"
    },
    {
      "count": 22694,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www134"
    },
    {
      "count": 22229,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www221"
    },
    {
      "count": 21502,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www128"
    },
    {
      "count": 19834,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www69"
    },
    {
      "count": 19801,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www43"
    },
    {
      "count": 19079,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "tms"
    },
    {
      "count": 17829,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www66"
    },
    {
      "count": 17803,
      "is_datatype": false,
      "label": "rdf",
      "link_num": 1,
      "source": "db",
      "target": "www466"
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
      "count": 17459,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "uk"
    },
    {
      "count": 17189,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www70"
    },
    {
      "count": 16289,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "twitt2"
    },
    {
      "count": 16013,
      "is_datatype": false,
      "label": "owl",
      "link_num": -3,
      "source": "db",
      "target": "UNKNOWN"
    },
    {
      "count": 15813,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www110"
    },
    {
      "count": 14938,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www56"
    },
    {
      "count": 14150,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www118"
    },
    {
      "count": 13866,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www68"
    },
    {
      "count": 13782,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www125"
    },
    {
      "count": 13655,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www99"
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
      "count": 12846,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www64"
    },
    {
      "count": 12644,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www178"
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
      "count": 11932,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www100"
    },
    {
      "count": 11589,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www57"
    },
    {
      "count": 11564,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "archiinfo"
    },
    {
      "count": 11422,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www172"
    },
    {
      "count": 11379,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www91"
    },
    {
      "count": 11271,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www59"
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
      "count": 11050,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www39"
    },
    {
      "count": 10885,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www142"
    },
    {
      "count": 10712,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www106"
    },
    {
      "count": 10319,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www140"
    },
    {
      "count": 10291,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "eu-football"
    },
    {
      "count": 10286,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www176"
    },
    {
      "count": 10184,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "doi"
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
      "count": 9857,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www60"
    },
    {
      "count": 9845,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www173"
    },
    {
      "count": 9729,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www139"
    },
    {
      "count": 9580,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www177"
    },
    {
      "count": 9391,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www248"
    },
    {
      "count": 9259,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www58"
    },
    {
      "count": 9005,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www36"
    },
    {
      "count": 8958,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www171"
    },
    {
      "count": 8939,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www8"
    },
    {
      "count": 8894,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www48"
    },
    {
      "count": 8828,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "eclipse"
    },
    {
      "count": 8755,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "github"
    },
    {
      "count": 8696,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www52"
    },
    {
      "count": 8676,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www136"
    },
    {
      "count": 8623,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "news"
    },
    {
      "count": 8609,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www55"
    },
    {
      "count": 8600,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www137"
    },
    {
      "count": 8276,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "reports"
    },
    {
      "count": 8262,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "stats3"
    },
    {
      "count": 8183,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www175"
    },
    {
      "count": 8078,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www29"
    },
    {
      "count": 7997,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www169"
    },
    {
      "count": 7950,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www166"
    },
    {
      "count": 7880,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www174"
    },
    {
      "count": 7716,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www260"
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
      "count": 7684,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www286"
    },
    {
      "count": 7672,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www111"
    },
    {
      "count": 7667,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "mlb"
    },
    {
      "count": 7641,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www61"
    },
    {
      "count": 7503,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "de"
    },
    {
      "count": 7492,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "geonaorg"
    },
    {
      "count": 7468,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "babel"
    },
    {
      "count": 7360,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www135"
    },
    {
      "count": 7314,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www97"
    },
    {
      "count": 7226,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www161"
    },
    {
      "count": 7177,
      "is_datatype": false,
      "label": "owl",
      "link_num": 1,
      "source": "db",
      "target": "global"
    },
    {
      "count": 7041,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www116"
    },
    {
      "count": 7028,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www32"
    },
    {
      "count": 6921,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "supreme"
    },
    {
      "count": 6906,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "maps"
    },
    {
      "count": 6894,
      "is_datatype": false,
      "label": "owl",
      "link_num": 2,
      "source": "db",
      "target": "wd"
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
      "count": 6814,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www51"
    },
    {
      "count": 6665,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www49"
    },
    {
      "count": 6578,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www204"
    },
    {
      "count": 6564,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www162"
    },
    {
      "count": 6546,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www155"
    },
    {
      "count": 6445,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "scholar"
    },
    {
      "count": 6350,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www37"
    },
    {
      "count": 6266,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www143"
    },
    {
      "count": 6141,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www132"
    },
    {
      "count": 6126,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www31"
    },
    {
      "count": 6123,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "adatbank"
    },
    {
      "count": 6053,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "us"
    },
    {
      "count": 6025,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www160"
    },
    {
      "count": 5983,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www151"
    },
    {
      "count": 5937,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www141"
    },
    {
      "count": 5936,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www54"
    },
    {
      "count": 5927,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www47"
    },
    {
      "count": 5885,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www3"
    },
    {
      "count": 5849,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "eu-fo2"
    },
    {
      "count": 5845,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "bookscom"
    },
    {
      "count": 5844,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www259"
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
      "count": 5797,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www46"
    },
    {
      "count": 5793,
      "is_datatype": false,
      "label": "rdrel",
      "link_num": 1,
      "source": "db",
      "target": "lobid"
    },
    {
      "count": 5793,
      "is_datatype": false,
      "label": "rdfs",
      "link_num": 2,
      "source": "db",
      "target": "lobid"
    },
    {
      "count": 5779,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "data5"
    },
    {
      "count": 5614,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www167"
    },
    {
      "count": 5550,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www182"
    },
    {
      "count": 5545,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www42"
    },
    {
      "count": 5494,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "galli2"
    },
    {
      "count": 5486,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www163"
    },
    {
      "count": 5469,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www34"
    },
    {
      "count": 5433,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www369"
    },
    {
      "count": 5428,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www168"
    },
    {
      "count": 5409,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "estadisticas"
    },
    {
      "count": 5353,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "pdfhost"
    },
    {
      "count": 5351,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "undocs"
    },
    {
      "count": 5330,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www165"
    },
    {
      "count": 5224,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "match2"
    },
    {
      "count": 5091,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "globoesporte"
    },
    {
      "count": 5051,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "archive"
    },
    {
      "count": 5034,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www156"
    },
    {
      "count": 5017,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "premierliga"
    },
    {
      "count": 5002,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www126"
    },
    {
      "count": 4998,
      "is_datatype": false,
      "label": "owl",
      "link_num": 2,
      "source": "db",
      "target": "www58"
    },
    {
      "count": 4949,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www157"
    },
    {
      "count": 4861,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "frcom"
    },
    {
      "count": 4851,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www53"
    },
    {
      "count": 4846,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www120"
    },
    {
      "count": 4806,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "gd2"
    },
    {
      "count": 4799,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "calcio-seriea"
    },
    {
      "count": 4794,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www38"
    },
    {
      "count": 4791,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "svenskfotboll"
    },
    {
      "count": 4786,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www40"
    },
    {
      "count": 4759,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www159"
    },
    {
      "count": 4746,
      "is_datatype": false,
      "label": "db",
      "link_num": -2,
      "source": "db",
      "target": "commo2"
    },
    {
      "count": 4699,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "hdl"
    },
    {
      "count": 4682,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "cdn"
    },
    {
      "count": 4677,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "nflcdns"
    },
    {
      "count": 4648,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www164"
    },
    {
      "count": 4588,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www149"
    },
    {
      "count": 4572,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www138"
    },
    {
      "count": 4540,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www129"
    },
    {
      "count": 4521,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www35"
    },
    {
      "count": 4468,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "statsorg"
    },
    {
      "count": 4444,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "data6"
    },
    {
      "count": 4436,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "hathitrust"
    },
    {
      "count": 4416,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www153"
    },
    {
      "count": 4410,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www41"
    },
    {
      "count": 4342,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "actas"
    },
    {
      "count": 4310,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www7"
    },
    {
      "count": 4288,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www206"
    },
    {
      "count": 4284,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www258"
    },
    {
      "count": 4282,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "ucjeps"
    },
    {
      "count": 4274,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www33"
    },
    {
      "count": 4258,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www188"
    },
    {
      "count": 4251,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "nla"
    },
    {
      "count": 4224,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "coludata"
    },
    {
      "count": 4214,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www256"
    },
    {
      "count": 4186,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www285"
    },
    {
      "count": 4129,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www23"
    },
    {
      "count": 4124,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www154"
    },
    {
      "count": 4116,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www257"
    },
    {
      "count": 4115,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "mediacom"
    },
    {
      "count": 4114,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www123"
    },
    {
      "count": 4102,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "politicalgraveyard"
    },
    {
      "count": 4086,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www205"
    },
    {
      "count": 4070,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www207"
    },
    {
      "count": 4023,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www203"
    },
    {
      "count": 3999,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www199"
    },
    {
      "count": 3990,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www30"
    },
    {
      "count": 3982,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "calphotos"
    },
    {
      "count": 3921,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "sites"
    },
    {
      "count": 3904,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www148"
    },
    {
      "count": 3891,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www231"
    },
    {
      "count": 3850,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www82"
    },
    {
      "count": 3838,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www201"
    },
    {
      "count": 3803,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "vimeo"
    },
    {
      "count": 3800,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "onlin2"
    },
    {
      "count": 3766,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www121"
    },
    {
      "count": 3750,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "drive"
    },
    {
      "count": 3737,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "apcbg"
    },
    {
      "count": 3695,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www18"
    },
    {
      "count": 3686,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www11"
    },
    {
      "count": 3683,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www12"
    },
    {
      "count": 3660,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www196"
    },
    {
      "count": 3658,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www28"
    },
    {
      "count": 3658,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "query"
    },
    {
      "count": 3588,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "whc"
    },
    {
      "count": 3572,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www101"
    },
    {
      "count": 3528,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "atlasuk"
    },
    {
      "count": 3525,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "espn"
    },
    {
      "count": 3519,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "bacdive"
    },
    {
      "count": 3511,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www228"
    },
    {
      "count": 3507,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "ratings"
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
      "count": 3505,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "kunishitei"
    },
    {
      "count": 3479,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www44"
    },
    {
      "count": 3443,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "trove"
    },
    {
      "count": 3442,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www208"
    },
    {
      "count": 3437,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www113"
    },
    {
      "count": 3433,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www185"
    },
    {
      "count": 3422,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www202"
    },
    {
      "count": 3390,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "linkorg"
    },
    {
      "count": 3370,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "plants"
    },
    {
      "count": 3366,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "zenodo.record"
    },
    {
      "count": 3343,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "timesmachine"
    },
    {
      "count": 3322,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www27"
    },
    {
      "count": 3322,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "gallica"
    },
    {
      "count": 3283,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www115"
    },
    {
      "count": 3278,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www158"
    },
    {
      "count": 3261,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "dx"
    },
    {
      "count": 3253,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www105"
    },
    {
      "count": 3215,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www6"
    },
    {
      "count": 3215,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "artuk"
    },
    {
      "count": 3211,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "matchcenter"
    },
    {
      "count": 3194,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www189"
    },
    {
      "count": 3189,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www194"
    },
    {
      "count": 3168,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www195"
    },
    {
      "count": 3161,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www103"
    },
    {
      "count": 3134,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www22"
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
      "count": 3095,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www112"
    },
    {
      "count": 3077,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www87"
    },
    {
      "count": 3076,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www114"
    },
    {
      "count": 3072,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "caselaw"
    },
    {
      "count": 3071,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www119"
    },
    {
      "count": 3068,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www187"
    },
    {
      "count": 3044,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www127"
    },
    {
      "count": 3044,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www14"
    },
    {
      "count": 3039,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www21"
    },
    {
      "count": 3037,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www89"
    },
    {
      "count": 3026,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www368"
    },
    {
      "count": 3024,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www95"
    },
    {
      "count": 3021,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www107"
    },
    {
      "count": 3008,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www190"
    },
    {
      "count": 3006,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www306"
    },
    {
      "count": 2996,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "ark"
    },
    {
      "count": 2994,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www306"
    },
    {
      "count": 2993,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "catalog"
    },
    {
      "count": 2987,
      "is_datatype": false,
      "label": "dc",
      "link_num": 9,
      "source": "db",
      "target": "db"
    },
    {
      "count": 2980,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "adminnet"
    },
    {
      "count": 2967,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www235"
    },
    {
      "count": 2951,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www108"
    },
    {
      "count": 2938,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www2"
    },
    {
      "count": 2936,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www191"
    },
    {
      "count": 2911,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www192"
    },
    {
      "count": 2889,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "itcom"
    },
    {
      "count": 2887,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www26"
    },
    {
      "count": 2875,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "adatb2"
    },
    {
      "count": 2871,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www9"
    },
    {
      "count": 2867,
      "is_datatype": false,
      "label": "skos",
      "link_num": 1,
      "source": "db",
      "target": "www45"
    },
    {
      "count": 2865,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www197"
    },
    {
      "count": 2842,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www131"
    },
    {
      "count": 2834,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www88"
    },
    {
      "count": 2832,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "babel2"
    },
    {
      "count": 2819,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www220"
    },
    {
      "count": 2814,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "esdbpr"
    },
    {
      "count": 2797,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www264"
    },
    {
      "count": 2777,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "sports"
    },
    {
      "count": 2770,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www93"
    },
    {
      "count": 2768,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www224"
    },
    {
      "count": 2755,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www225"
    },
    {
      "count": 2746,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www130"
    },
    {
      "count": 2723,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "commo3"
    },
    {
      "count": 2707,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "jacom"
    },
    {
      "count": 2701,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www200"
    },
    {
      "count": 2694,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www152"
    },
    {
      "count": 2691,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www255"
    },
    {
      "count": 2660,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "penelope"
    },
    {
      "count": 2658,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www17"
    },
    {
      "count": 2629,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www96"
    },
    {
      "count": 2618,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www230"
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
      "count": 2610,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www278"
    },
    {
      "count": 2608,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www94"
    },
    {
      "count": 2604,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www150"
    },
    {
      "count": 2602,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www109"
    },
    {
      "count": 2596,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www184"
    },
    {
      "count": 2578,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "globalsportsarchive"
    },
    {
      "count": 2574,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www13"
    },
    {
      "count": 2572,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www253"
    },
    {
      "count": 2571,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www79"
    },
    {
      "count": 2563,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "historiawisly"
    },
    {
      "count": 2550,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www117"
    },
    {
      "count": 2543,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "basketball"
    },
    {
      "count": 2542,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www20"
    },
    {
      "count": 2513,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www10"
    },
    {
      "count": 2506,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www193"
    },
    {
      "count": 2505,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www229"
    },
    {
      "count": 2499,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www284"
    },
    {
      "count": 2498,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "wikimapia"
    },
    {
      "count": 2478,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "tools"
    },
    {
      "count": 2472,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "nl"
    },
    {
      "count": 2468,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www90"
    },
    {
      "count": 2466,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www86"
    },
    {
      "count": 2446,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "itunes"
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
      "count": 2445,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www15"
    },
    {
      "count": 2440,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www461"
    },
    {
      "count": 2422,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www173"
    },
    {
      "count": 2417,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www244"
    },
    {
      "count": 2399,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "olympics"
    },
    {
      "count": 2387,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www98"
    },
    {
      "count": 2382,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "svcom"
    },
    {
      "count": 2361,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "fibalivestats"
    },
    {
      "count": 2344,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www242"
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
      "count": 2339,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www245"
    },
    {
      "count": 2325,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www226"
    },
    {
      "count": 2318,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "law"
    },
    {
      "count": 2312,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www215"
    },
    {
      "count": 2310,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www104"
    },
    {
      "count": 2296,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "soundcloud"
    },
    {
      "count": 2292,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "data7"
    },
    {
      "count": 2291,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www217"
    },
    {
      "count": 2282,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www179"
    },
    {
      "count": 2279,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "trove2"
    },
    {
      "count": 2263,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "onlinelibrary"
    },
    {
      "count": 2259,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www147"
    },
    {
      "count": 2247,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "libmma"
    },
    {
      "count": 2242,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www246"
    },
    {
      "count": 2235,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "census"
    },
    {
      "count": 2223,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "pt"
    },
    {
      "count": 2218,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www80"
    },
    {
      "count": 2217,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www243"
    },
    {
      "count": 2211,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "gutenberg"
    },
    {
      "count": 2207,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www50"
    },
    {
      "count": 2206,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www-old"
    },
    {
      "count": 2190,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www296"
    },
    {
      "count": 2187,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "topostext"
    },
    {
      "count": 2184,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www76"
    },
    {
      "count": 2182,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www145"
    },
    {
      "count": 2176,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www5"
    },
    {
      "count": 2174,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www223"
    },
    {
      "count": 2169,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www81"
    },
    {
      "count": 2153,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www211"
    },
    {
      "count": 2149,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www77"
    },
    {
      "count": 2147,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www241"
    },
    {
      "count": 2142,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www16"
    },
    {
      "count": 2142,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "ligamx"
    },
    {
      "count": 2141,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www124"
    },
    {
      "count": 2141,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "psephos"
    },
    {
      "count": 2137,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "pl"
    },
    {
      "count": 2137,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "historicengland"
    },
    {
      "count": 2136,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www254"
    },
    {
      "count": 2134,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www227"
    },
    {
      "count": 2121,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "soccernet"
    },
    {
      "count": 2119,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "eredivisie"
    },
    {
      "count": 2111,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www279"
    },
    {
      "count": 2105,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "runeberg"
    },
    {
      "count": 2102,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "ru2"
    },
    {
      "count": 2089,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www287"
    },
    {
      "count": 2080,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "beth"
    },
    {
      "count": 2076,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "webarchive"
    },
    {
      "count": 2058,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www214"
    },
    {
      "count": 2057,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www218"
    },
    {
      "count": 2051,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "stats2"
    },
    {
      "count": 2048,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "scores"
    },
    {
      "count": 2047,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www75"
    },
    {
      "count": 2046,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www303"
    },
    {
      "count": 2028,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www24"
    },
    {
      "count": 2022,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "samuraiblue"
    },
    {
      "count": 2021,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www186"
    },
    {
      "count": 2019,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www250"
    },
    {
      "count": 2012,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "ghostarchive"
    },
    {
      "count": 2010,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "eurohockey"
    },
    {
      "count": 2004,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www216"
    },
    {
      "count": 1986,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www302"
    },
    {
      "count": 1980,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "vimeo2"
    },
    {
      "count": 1974,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www233"
    },
    {
      "count": 1970,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "content-aus"
    },
    {
      "count": 1969,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "arxiv"
    },
    {
      "count": 1969,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www219"
    },
    {
      "count": 1968,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www239"
    },
    {
      "count": 1965,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www232"
    },
    {
      "count": 1962,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www222"
    },
    {
      "count": 1957,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "openlibrary"
    },
    {
      "count": 1956,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www324"
    },
    {
      "count": 1956,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "druginfo"
    },
    {
      "count": 1949,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www251"
    },
    {
      "count": 1941,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www122"
    },
    {
      "count": 1934,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "hockeyaustralia"
    },
    {
      "count": 1918,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www240"
    },
    {
      "count": 1917,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "hemeroteca-paginas"
    },
    {
      "count": 1913,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www210"
    },
    {
      "count": 1909,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "spfl2"
    },
    {
      "count": 1902,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www331"
    },
    {
      "count": 1897,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www280"
    },
    {
      "count": 1892,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "uboat"
    },
    {
      "count": 1891,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www181"
    },
    {
      "count": 1886,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www270"
    },
    {
      "count": 1876,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www238"
    },
    {
      "count": 1850,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www439"
    },
    {
      "count": 1836,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www213"
    },
    {
      "count": 1834,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www371"
    },
    {
      "count": 1833,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www438"
    },
    {
      "count": 1826,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "rugby2"
    },
    {
      "count": 1816,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www435"
    },
    {
      "count": 1815,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "articles"
    },
    {
      "count": 1803,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "fbref"
    },
    {
      "count": 1798,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www19"
    },
    {
      "count": 1793,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www83"
    },
    {
      "count": 1791,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "data9"
    },
    {
      "count": 1787,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www276"
    },
    {
      "count": 1786,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "obswww"
    },
    {
      "count": 1781,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www25"
    },
    {
      "count": 1774,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "bwf"
    },
    {
      "count": 1774,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "esporte"
    },
    {
      "count": 1770,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www322"
    },
    {
      "count": 1764,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "biodiversitylibrary"
    },
    {
      "count": 1760,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www332"
    },
    {
      "count": 1759,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www283"
    },
    {
      "count": 1757,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www272"
    },
    {
      "count": 1756,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "osmrel"
    },
    {
      "count": 1756,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www303"
    },
    {
      "count": 1755,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "findarticles"
    },
    {
      "count": 1754,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www277"
    },
    {
      "count": 1753,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www456"
    },
    {
      "count": 1750,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www266"
    },
    {
      "count": 1747,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www273"
    },
    {
      "count": 1747,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "issuu"
    },
    {
      "count": 1745,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www170"
    },
    {
      "count": 1741,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "muse"
    },
    {
      "count": 1737,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www212"
    },
    {
      "count": 1731,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "dare"
    },
    {
      "count": 1730,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www237"
    },
    {
      "count": 1723,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "hdl2"
    },
    {
      "count": 1720,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www252"
    },
    {
      "count": 1720,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www428"
    },
    {
      "count": 1717,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www282"
    },
    {
      "count": 1716,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www427"
    },
    {
      "count": 1715,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www329"
    },
    {
      "count": 1712,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "imslp"
    },
    {
      "count": 1710,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www274"
    },
    {
      "count": 1707,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www198"
    },
    {
      "count": 1702,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www299"
    },
    {
      "count": 1702,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "open"
    },
    {
      "count": 1695,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "msrmaps"
    },
    {
      "count": 1693,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www271"
    },
    {
      "count": 1692,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "vam"
    },
    {
      "count": 1690,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www262"
    },
    {
      "count": 1689,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "wwwcom"
    },
    {
      "count": 1688,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "quod"
    },
    {
      "count": 1686,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www290"
    },
    {
      "count": 1684,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www416"
    },
    {
      "count": 1674,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "myspaorg"
    },
    {
      "count": 1672,
      "is_datatype": false,
      "label": "owl",
      "link_num": 2,
      "source": "db",
      "target": "frcom"
    },
    {
      "count": 1669,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www305"
    },
    {
      "count": 1668,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www437"
    },
    {
      "count": 1667,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "fr"
    },
    {
      "count": 1666,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www"
    },
    {
      "count": 1666,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www209"
    },
    {
      "count": 1664,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www432"
    },
    {
      "count": 1663,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www304"
    },
    {
      "count": 1662,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www408"
    },
    {
      "count": 1659,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "ourairports"
    },
    {
      "count": 1649,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www298"
    },
    {
      "count": 1645,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "designatedsites"
    },
    {
      "count": 1644,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www261"
    },
    {
      "count": 1644,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www247"
    },
    {
      "count": 1644,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www356"
    },
    {
      "count": 1640,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "humantfs"
    },
    {
      "count": 1632,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "ligam2"
    },
    {
      "count": 1632,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www436"
    },
    {
      "count": 1632,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www268"
    },
    {
      "count": 1631,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "upl"
    },
    {
      "count": 1629,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www434"
    },
    {
      "count": 1629,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www301"
    },
    {
      "count": 1622,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "magic"
    },
    {
      "count": 1621,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www418"
    },
    {
      "count": 1610,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "indiarailinfo"
    },
    {
      "count": 1609,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "m"
    },
    {
      "count": 1608,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www364"
    },
    {
      "count": 1607,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "hrnogomet"
    },
    {
      "count": 1606,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "kulturarvsdata"
    },
    {
      "count": 1603,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "copernix"
    },
    {
      "count": 1600,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www300"
    },
    {
      "count": 1599,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "allworldcup"
    },
    {
      "count": 1592,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "player"
    },
    {
      "count": 1580,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "ecnet"
    },
    {
      "count": 1579,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www433"
    },
    {
      "count": 1576,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "adsabs"
    },
    {
      "count": 1575,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "webargov"
    },
    {
      "count": 1573,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www249"
    },
    {
      "count": 1570,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www293"
    },
    {
      "count": 1569,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www292"
    },
    {
      "count": 1568,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www265"
    },
    {
      "count": 1559,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www327"
    },
    {
      "count": 1556,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www431"
    },
    {
      "count": 1552,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "referenceworks"
    },
    {
      "count": 1550,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www85"
    },
    {
      "count": 1547,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "waterpolo"
    },
    {
      "count": 1546,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www429"
    },
    {
      "count": 1543,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www269"
    },
    {
      "count": 1538,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www430"
    },
    {
      "count": 1538,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www180"
    },
    {
      "count": 1536,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "academic"
    },
    {
      "count": 1530,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www452"
    },
    {
      "count": 1529,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www319"
    },
    {
      "count": 1529,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "paperspast"
    },
    {
      "count": 1527,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www295"
    },
    {
      "count": 1526,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "muse2"
    },
    {
      "count": 1521,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "jalgpall"
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
      "count": 1515,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "hdlnet"
    },
    {
      "count": 1503,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "translate"
    },
    {
      "count": 1501,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "canpl"
    },
    {
      "count": 1500,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "gd22"
    },
    {
      "count": 1490,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www326"
    },
    {
      "count": 1489,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www425"
    },
    {
      "count": 1489,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www297"
    },
    {
      "count": 1486,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "wtafinet"
    },
    {
      "count": 1486,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www426"
    },
    {
      "count": 1486,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www417"
    },
    {
      "count": 1482,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www422"
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
      "count": 1478,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "journals"
    },
    {
      "count": 1476,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www455"
    },
    {
      "count": 1474,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www314"
    },
    {
      "count": 1468,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "eci"
    },
    {
      "count": 1466,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "docs"
    },
    {
      "count": 1464,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www413"
    },
    {
      "count": 1463,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www102"
    },
    {
      "count": 1462,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www343"
    },
    {
      "count": 1462,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "zh"
    },
    {
      "count": 1459,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www288"
    },
    {
      "count": 1457,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "memory"
    },
    {
      "count": 1454,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www334"
    },
    {
      "count": 1453,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "webar2"
    },
    {
      "count": 1452,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "ballotpedia"
    },
    {
      "count": 1449,
      "is_datatype": false,
      "label": "owl",
      "link_num": 2,
      "source": "db",
      "target": "de"
    },
    {
      "count": 1449,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www313"
    },
    {
      "count": 1448,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www294"
    },
    {
      "count": 1442,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www328"
    },
    {
      "count": 1441,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "host"
    },
    {
      "count": 1440,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www423"
    },
    {
      "count": 1439,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "weborg"
    },
    {
      "count": 1438,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www339"
    },
    {
      "count": 1437,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www321"
    },
    {
      "count": 1436,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "wtafiles"
    },
    {
      "count": 1430,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www424"
    },
    {
      "count": 1429,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www307"
    },
    {
      "count": 1426,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www84"
    },
    {
      "count": 1425,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www384"
    },
    {
      "count": 1425,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "standardebooks"
    },
    {
      "count": 1417,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "beachsoccerrussia"
    },
    {
      "count": 1409,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www183"
    },
    {
      "count": 1406,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "fmg"
    },
    {
      "count": 1406,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www378"
    },
    {
      "count": 1403,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www263"
    },
    {
      "count": 1400,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "ada1bank"
    },
    {
      "count": 1399,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www337"
    },
    {
      "count": 1398,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www421"
    },
    {
      "count": 1398,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www78"
    },
    {
      "count": 1397,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www338"
    },
    {
      "count": 1396,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www454"
    },
    {
      "count": 1395,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www361"
    },
    {
      "count": 1383,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www344"
    },
    {
      "count": 1382,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www318"
    },
    {
      "count": 1379,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www419"
    },
    {
      "count": 1378,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www340"
    },
    {
      "count": 1371,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www92"
    },
    {
      "count": 1365,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www336"
    },
    {
      "count": 1364,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "content-uk"
    },
    {
      "count": 1363,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www320"
    },
    {
      "count": 1362,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www420"
    },
    {
      "count": 1356,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www308"
    },
    {
      "count": 1355,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "thomas"
    },
    {
      "count": 1352,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www403"
    },
    {
      "count": 1350,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www325"
    },
    {
      "count": 1347,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "dx2"
    },
    {
      "count": 1345,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "beachsoccer"
    },
    {
      "count": 1345,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www367"
    },
    {
      "count": 1344,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "uk2"
    },
    {
      "count": 1342,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "football"
    },
    {
      "count": 1341,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www346"
    },
    {
      "count": 1339,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "soccerstats"
    },
    {
      "count": 1339,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "pba2"
    },
    {
      "count": 1337,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www365"
    },
    {
      "count": 1333,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www323"
    },
    {
      "count": 1333,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www345"
    },
    {
      "count": 1331,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "nces"
    },
    {
      "count": 1321,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "ehfcl"
    },
    {
      "count": 1321,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "wikidata"
    },
    {
      "count": 1320,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www414"
    },
    {
      "count": 1319,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "europeancup"
    },
    {
      "count": 1316,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www462"
    },
    {
      "count": 1313,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www415"
    },
    {
      "count": 1311,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "en"
    },
    {
      "count": 1308,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www309"
    },
    {
      "count": 1308,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "baske2"
    },
    {
      "count": 1307,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "openjurist"
    },
    {
      "count": 1305,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "prvahnl"
    },
    {
      "count": 1305,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "en2"
    },
    {
      "count": 1301,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www330"
    },
    {
      "count": 1298,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www399"
    },
    {
      "count": 1295,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www396"
    },
    {
      "count": 1293,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www412"
    },
    {
      "count": 1288,
      "is_datatype": false,
      "label": "owl",
      "link_num": 2,
      "source": "db",
      "target": "itcom"
    },
    {
      "count": 1283,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www453"
    },
    {
      "count": 1273,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www316"
    },
    {
      "count": 1265,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www447"
    },
    {
      "count": 1263,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "universiade2013"
    },
    {
      "count": 1263,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www315"
    },
    {
      "count": 1262,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www382"
    },
    {
      "count": 1260,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www333"
    },
    {
      "count": 1259,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "livestats"
    },
    {
      "count": 1256,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "competiciones"
    },
    {
      "count": 1255,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www366"
    },
    {
      "count": 1254,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "wayback"
    },
    {
      "count": 1245,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "twitt2"
    },
    {
      "count": 1242,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "users"
    },
    {
      "count": 1241,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www236"
    },
    {
      "count": 1241,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "playorg"
    },
    {
      "count": 1238,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "2009-2017"
    },
    {
      "count": 1237,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "pqasb"
    },
    {
      "count": 1236,
      "is_datatype": false,
      "label": "owl",
      "link_num": 2,
      "source": "db",
      "target": "esdbpr"
    },
    {
      "count": 1233,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www404"
    },
    {
      "count": 1232,
      "is_datatype": false,
      "label": "foaf",
      "link_num": -2,
      "source": "db",
      "target": "archi3"
    },
    {
      "count": 1229,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www409"
    },
    {
      "count": 1227,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www398"
    },
    {
      "count": 1227,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "cdngov"
    },
    {
      "count": 1224,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "womenscompetitions"
    },
    {
      "count": 1224,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "statiorg"
    },
    {
      "count": 1223,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www411"
    },
    {
      "count": 1223,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www310"
    },
    {
      "count": 1221,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "ceb"
    },
    {
      "count": 1221,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www341"
    },
    {
      "count": 1219,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "spfl"
    },
    {
      "count": 1218,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www281"
    },
    {
      "count": 1217,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www342"
    },
    {
      "count": 1215,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www410"
    },
    {
      "count": 1214,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "nbn-resolving"
    },
    {
      "count": 1214,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "scenaripolitici"
    },
    {
      "count": 1209,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "ssrn"
    },
    {
      "count": 1209,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www291"
    },
    {
      "count": 1208,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "prvah2"
    },
    {
      "count": 1207,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www407"
    },
    {
      "count": 1200,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "brie"
    },
    {
      "count": 1198,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "bioguide"
    },
    {
      "count": 1198,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www406"
    },
    {
      "count": 1198,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www312"
    },
    {
      "count": 1196,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www354"
    },
    {
      "count": 1196,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www317"
    },
    {
      "count": 1194,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "knyga"
    },
    {
      "count": 1193,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www358"
    },
    {
      "count": 1190,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www335"
    },
    {
      "count": 1189,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "plato"
    },
    {
      "count": 1188,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "pba"
    },
    {
      "count": 1188,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www381"
    },
    {
      "count": 1187,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "pflk"
    },
    {
      "count": 1179,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www383"
    },
    {
      "count": 1178,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www2com"
    },
    {
      "count": 1173,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www405"
    },
    {
      "count": 1172,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www359"
    },
    {
      "count": 1169,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www363"
    },
    {
      "count": 1166,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www357"
    },
    {
      "count": 1166,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www386"
    },
    {
      "count": 1161,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "content"
    },
    {
      "count": 1159,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www351"
    },
    {
      "count": 1158,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www352"
    },
    {
      "count": 1157,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www353"
    },
    {
      "count": 1155,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www402"
    },
    {
      "count": 1152,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www459"
    },
    {
      "count": 1149,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "bugguide"
    },
    {
      "count": 1148,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www350"
    },
    {
      "count": 1147,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "bwf2"
    },
    {
      "count": 1145,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "164"
    },
    {
      "count": 1144,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www401"
    },
    {
      "count": 1142,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "licensing"
    },
    {
      "count": 1141,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "sites2"
    },
    {
      "count": 1139,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "enorg"
    },
    {
      "count": 1137,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "wrsd"
    },
    {
      "count": 1136,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www379"
    },
    {
      "count": 1134,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www400"
    },
    {
      "count": 1131,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www380"
    },
    {
      "count": 1128,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www465"
    },
    {
      "count": 1128,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www275"
    },
    {
      "count": 1128,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "paleodb"
    },
    {
      "count": 1127,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www360"
    },
    {
      "count": 1125,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "keepup"
    },
    {
      "count": 1124,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "bsrussia"
    },
    {
      "count": 1123,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "loksabhaph"
    },
    {
      "count": 1120,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "semanticscholar"
    },
    {
      "count": 1118,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www375"
    },
    {
      "count": 1117,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "data10"
    },
    {
      "count": 1116,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "eci2"
    },
    {
      "count": 1113,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www362"
    },
    {
      "count": 1111,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www444"
    },
    {
      "count": 1108,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www397"
    },
    {
      "count": 1104,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "cms"
    },
    {
      "count": 1104,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www347"
    },
    {
      "count": 1102,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www458"
    },
    {
      "count": 1100,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www395"
    },
    {
      "count": 1099,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www377"
    },
    {
      "count": 1098,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "egrove"
    },
    {
      "count": 1093,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www348"
    },
    {
      "count": 1093,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www373"
    },
    {
      "count": 1093,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www289"
    },
    {
      "count": 1088,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www394"
    },
    {
      "count": 1087,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www370"
    },
    {
      "count": 1087,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "journcom"
    },
    {
      "count": 1084,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www464"
    },
    {
      "count": 1082,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www234"
    },
    {
      "count": 1078,
      "is_datatype": false,
      "label": "owl",
      "link_num": -1,
      "source": "db",
      "target": "arz"
    },
    {
      "count": 1078,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "generals"
    },
    {
      "count": 1076,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "bacdi2"
    },
    {
      "count": 1075,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www467"
    },
    {
      "count": 1074,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www391"
    },
    {
      "count": 1072,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www174"
    },
    {
      "count": 1072,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "v7player"
    },
    {
      "count": 1069,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "ethos"
    },
    {
      "count": 1066,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www451"
    },
    {
      "count": 1064,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www224"
    },
    {
      "count": 1061,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www311"
    },
    {
      "count": 1056,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www374"
    },
    {
      "count": 1056,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "eur-lex"
    },
    {
      "count": 1052,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www355"
    },
    {
      "count": 1051,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "archi4"
    },
    {
      "count": 1050,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "byucougars"
    },
    {
      "count": 1050,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www392"
    },
    {
      "count": 1049,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www393"
    },
    {
      "count": 1049,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www390"
    },
    {
      "count": 1046,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www450"
    },
    {
      "count": 1045,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "data8"
    },
    {
      "count": 1045,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "jewishencyclopedia"
    },
    {
      "count": 1044,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www446"
    },
    {
      "count": 1041,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "aic"
    },
    {
      "count": 1040,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www376"
    },
    {
      "count": 1038,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www449"
    },
    {
      "count": 1038,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www389"
    },
    {
      "count": 1038,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www267"
    },
    {
      "count": 1037,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www387"
    },
    {
      "count": 1036,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "worldcat"
    },
    {
      "count": 1033,
      "is_datatype": false,
      "label": "owl",
      "link_num": 2,
      "source": "db",
      "target": "ru2"
    },
    {
      "count": 1030,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "ballo2"
    },
    {
      "count": 1026,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www442"
    },
    {
      "count": 1025,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "ncbibook"
    },
    {
      "count": 1021,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www388"
    },
    {
      "count": 1020,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www349"
    },
    {
      "count": 1019,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www441"
    },
    {
      "count": 1017,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "progenetix"
    },
    {
      "count": 1015,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "globo2"
    },
    {
      "count": 1011,
      "is_datatype": false,
      "label": "owl",
      "link_num": 2,
      "source": "db",
      "target": "nl"
    },
    {
      "count": 1008,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www372"
    },
    {
      "count": 1004,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "texassports"
    },
    {
      "count": 1002,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www463"
    },
    {
      "count": 1000,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "bisinfo"
    },
    {
      "count": 1000,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www440"
    },
    {
      "count": 999,
      "is_datatype": false,
      "label": "owl",
      "link_num": 2,
      "source": "db",
      "target": "svcom"
    },
    {
      "count": 988,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "le"
    },
    {
      "count": 988,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "fi"
    },
    {
      "count": 986,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www443"
    },
    {
      "count": 985,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "websites"
    },
    {
      "count": 969,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www460"
    },
    {
      "count": 961,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www448"
    },
    {
      "count": 957,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "ameblo"
    },
    {
      "count": 954,
      "is_datatype": false,
      "label": "owl",
      "link_num": 2,
      "source": "db",
      "target": "pl"
    },
    {
      "count": 903,
      "is_datatype": false,
      "label": "owl",
      "link_num": 2,
      "source": "db",
      "target": "pt"
    },
    {
      "count": 891,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www445"
    },
    {
      "count": 880,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www448"
    },
    {
      "count": 873,
      "is_datatype": false,
      "label": "owl",
      "link_num": -1,
      "source": "db",
      "target": "ar"
    },
    {
      "count": 864,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www457"
    },
    {
      "count": 863,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "onlin2"
    },
    {
      "count": 858,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "he"
    },
    {
      "count": 853,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www445"
    },
    {
      "count": 832,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "instagram"
    },
    {
      "count": 827,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "weki"
    },
    {
      "count": 797,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "no"
    },
    {
      "count": 795,
      "is_datatype": false,
      "label": "owl",
      "link_num": -2,
      "source": "db",
      "target": "ceb"
    },
    {
      "count": 759,
      "is_datatype": false,
      "label": "owl",
      "link_num": 2,
      "source": "db",
      "target": "zh"
    },
    {
      "count": 738,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www468"
    },
    {
      "count": 726,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www385"
    },
    {
      "count": 694,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "www469"
    },
    {
      "count": 686,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "sites"
    },
    {
      "count": 682,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "cacom"
    },
    {
      "count": 680,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www469"
    },
    {
      "count": 673,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www264"
    },
    {
      "count": 633,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www220"
    },
    {
      "count": 627,
      "is_datatype": false,
      "label": "owl",
      "link_num": 2,
      "source": "db",
      "target": "jacom"
    },
    {
      "count": 620,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "eolife"
    },
    {
      "count": 614,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "csdbp"
    },
    {
      "count": 539,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www468"
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
      "count": 515,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www71"
    },
    {
      "count": 513,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www159"
    },
    {
      "count": 505,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 10,
      "source": "db",
      "target": "db"
    },
    {
      "count": 475,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "loksabhaph"
    },
    {
      "count": 474,
      "is_datatype": false,
      "label": "owl",
      "link_num": -2,
      "source": "db",
      "target": "cacom"
    },
    {
      "count": 465,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "gbif"
    },
    {
      "count": 461,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www378"
    },
    {
      "count": 438,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "github"
    },
    {
      "count": 424,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www225"
    },
    {
      "count": 409,
      "is_datatype": false,
      "label": "owl",
      "link_num": 2,
      "source": "db",
      "target": "no"
    },
    {
      "count": 408,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "onlinelibrary"
    },
    {
      "count": 407,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www245"
    },
    {
      "count": 393,
      "is_datatype": false,
      "label": "foaf",
      "link_num": -2,
      "source": "db",
      "target": "164"
    },
    {
      "count": 390,
      "is_datatype": false,
      "label": "owl",
      "link_num": 2,
      "source": "db",
      "target": "fi"
    },
    {
      "count": 387,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www211"
    },
    {
      "count": 384,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "motogp"
    },
    {
      "count": 382,
      "is_datatype": false,
      "label": "db",
      "link_num": -2,
      "source": "db",
      "target": "ar"
    },
    {
      "count": 366,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www261"
    },
    {
      "count": 365,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www19"
    },
    {
      "count": 355,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www385"
    },
    {
      "count": 342,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "myspaorg"
    },
    {
      "count": 339,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "journals"
    },
    {
      "count": 332,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www243"
    },
    {
      "count": 332,
      "is_datatype": false,
      "label": "foaf",
      "link_num": -2,
      "source": "db",
      "target": "ameblo"
    },
    {
      "count": 324,
      "is_datatype": false,
      "label": "owl",
      "link_num": -2,
      "source": "db",
      "target": "csdbp"
    },
    {
      "count": 323,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 3,
      "source": "db",
      "target": "www58"
    },
    {
      "count": 314,
      "is_datatype": false,
      "label": "db",
      "link_num": 2,
      "source": "db",
      "target": "www4"
    },
    {
      "count": 314,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www165"
    },
    {
      "count": 307,
      "is_datatype": false,
      "label": "skos",
      "link_num": -4,
      "source": "db",
      "target": "UNKNOWN"
    },
    {
      "count": 304,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "bgdbr"
    },
    {
      "count": 274,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "inaturalist.taxon"
    },
    {
      "count": 267,
      "is_datatype": false,
      "label": "owl",
      "link_num": 2,
      "source": "db",
      "target": "he"
    },
    {
      "count": 256,
      "is_datatype": false,
      "label": "foaf",
      "link_num": -2,
      "source": "db",
      "target": "academic"
    },
    {
      "count": 253,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www51"
    },
    {
      "count": 250,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www386"
    },
    {
      "count": 226,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "osmway"
    },
    {
      "count": 214,
      "is_datatype": false,
      "label": "owl",
      "link_num": 1,
      "source": "db",
      "target": "wbc"
    },
    {
      "count": 213,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www136"
    },
    {
      "count": 212,
      "is_datatype": false,
      "label": "owl",
      "link_num": -2,
      "source": "db",
      "target": "bgdbr"
    },
    {
      "count": 207,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www457"
    },
    {
      "count": 205,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www36"
    },
    {
      "count": 204,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www319"
    },
    {
      "count": 195,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www192"
    },
    {
      "count": 180,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www207"
    },
    {
      "count": 178,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "journcom"
    },
    {
      "count": 176,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www384"
    },
    {
      "count": 170,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www215"
    },
    {
      "count": 170,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "instagram"
    },
    {
      "count": 164,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www67"
    },
    {
      "count": 158,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www73"
    },
    {
      "count": 141,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www340"
    },
    {
      "count": 135,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www53"
    },
    {
      "count": 134,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www66"
    },
    {
      "count": 133,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www12"
    },
    {
      "count": 132,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www141"
    },
    {
      "count": 132,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www133"
    },
    {
      "count": 128,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www244"
    },
    {
      "count": 126,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "linkorg"
    },
    {
      "count": 121,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www279"
    },
    {
      "count": 114,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www140"
    },
    {
      "count": 112,
      "is_datatype": false,
      "label": "db",
      "link_num": 2,
      "source": "db",
      "target": "gnd"
    },
    {
      "count": 112,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "websites"
    },
    {
      "count": 108,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www148"
    },
    {
      "count": 108,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www64"
    },
    {
      "count": 103,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "soundcloud"
    },
    {
      "count": 101,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "mlb"
    },
    {
      "count": 100,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www152"
    },
    {
      "count": 99,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "meat"
    },
    {
      "count": 98,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www383"
    },
    {
      "count": 95,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www54"
    },
    {
      "count": 94,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www43"
    },
    {
      "count": 93,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www48"
    },
    {
      "count": 92,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www460"
    },
    {
      "count": 92,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www130"
    },
    {
      "count": 90,
      "is_datatype": false,
      "label": "foaf",
      "link_num": -2,
      "source": "db",
      "target": "archi2"
    },
    {
      "count": 89,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www156"
    },
    {
      "count": 88,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www329"
    },
    {
      "count": 87,
      "is_datatype": false,
      "label": "foaf",
      "link_num": -2,
      "source": "db",
      "target": "archiinfo"
    },
    {
      "count": 87,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www118"
    },
    {
      "count": 86,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "designatedsites"
    },
    {
      "count": 84,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "ple"
    },
    {
      "count": 83,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www200"
    },
    {
      "count": 81,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "sites2"
    },
    {
      "count": 80,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www451"
    },
    {
      "count": 79,
      "is_datatype": false,
      "label": "db",
      "link_num": 3,
      "source": "db",
      "target": "viaf"
    },
    {
      "count": 78,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www323"
    },
    {
      "count": 77,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "whc"
    },
    {
      "count": 75,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "ecnet"
    },
    {
      "count": 74,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "osmnode"
    },
    {
      "count": 70,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www212"
    },
    {
      "count": 69,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www126"
    },
    {
      "count": 67,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www358"
    },
    {
      "count": 66,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "nbn-resolving"
    },
    {
      "count": 64,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "vimeo"
    },
    {
      "count": 63,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www115"
    },
    {
      "count": 62,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www110"
    },
    {
      "count": 62,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "ex"
    },
    {
      "count": 61,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www444"
    },
    {
      "count": 61,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "inaturalist.place"
    },
    {
      "count": 59,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "le"
    },
    {
      "count": 58,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www62"
    },
    {
      "count": 57,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "jalgpall"
    },
    {
      "count": 57,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www8"
    },
    {
      "count": 56,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "wwf.ecoregion"
    },
    {
      "count": 53,
      "is_datatype": false,
      "label": "db",
      "link_num": -2,
      "source": "db",
      "target": "data4"
    },
    {
      "count": 53,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www31"
    },
    {
      "count": 52,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www29"
    },
    {
      "count": 52,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "m"
    },
    {
      "count": 52,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www410"
    },
    {
      "count": 50,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www241"
    },
    {
      "count": 50,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www443"
    },
    {
      "count": 49,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www180"
    },
    {
      "count": 49,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "ligamx"
    },
    {
      "count": 49,
      "is_datatype": false,
      "label": "db",
      "link_num": 2,
      "source": "db",
      "target": "purl2"
    },
    {
      "count": 49,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www21"
    },
    {
      "count": 48,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "webarchive"
    },
    {
      "count": 48,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www412"
    },
    {
      "count": 47,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "svenskfotboll"
    },
    {
      "count": 46,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www30"
    },
    {
      "count": 46,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www233"
    },
    {
      "count": 45,
      "is_datatype": false,
      "label": "foaf",
      "link_num": -2,
      "source": "db",
      "target": "cricketarchive"
    },
    {
      "count": 45,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www301"
    },
    {
      "count": 42,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www299"
    },
    {
      "count": 41,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "cordis.project"
    },
    {
      "count": 40,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www56"
    },
    {
      "count": 40,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www382"
    },
    {
      "count": 40,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "snac"
    },
    {
      "count": 40,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www63"
    },
    {
      "count": 39,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "ncbi.genome"
    },
    {
      "count": 39,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "publons.researcher"
    },
    {
      "count": 38,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "wayback"
    },
    {
      "count": 38,
      "is_datatype": false,
      "label": "foaf",
      "link_num": -2,
      "source": "db",
      "target": "books"
    },
    {
      "count": 36,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www160"
    },
    {
      "count": 36,
      "is_datatype": false,
      "label": "db",
      "link_num": 4,
      "source": "db",
      "target": "wiki"
    },
    {
      "count": 34,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www151"
    },
    {
      "count": 33,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "en2"
    },
    {
      "count": 33,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www86"
    },
    {
      "count": 32,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www302"
    },
    {
      "count": 32,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www91"
    },
    {
      "count": 32,
      "is_datatype": false,
      "label": "rdfs",
      "link_num": 3,
      "source": "db",
      "target": "frcom"
    },
    {
      "count": 31,
      "is_datatype": false,
      "label": "rdfs",
      "link_num": 3,
      "source": "db",
      "target": "de"
    },
    {
      "count": 30,
      "is_datatype": false,
      "label": "db",
      "link_num": 2,
      "source": "db",
      "target": "eunis2"
    },
    {
      "count": 28,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "newscom"
    },
    {
      "count": 28,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www134"
    },
    {
      "count": 27,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "bsb"
    },
    {
      "count": 27,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www175"
    },
    {
      "count": 27,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www197"
    },
    {
      "count": 27,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www84"
    },
    {
      "count": 27,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www176"
    },
    {
      "count": 27,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www93"
    },
    {
      "count": 27,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www78"
    },
    {
      "count": 26,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www16"
    },
    {
      "count": 26,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www208"
    },
    {
      "count": 26,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "geonames"
    },
    {
      "count": 26,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www408"
    },
    {
      "count": 26,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www179"
    },
    {
      "count": 25,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "translate"
    },
    {
      "count": 25,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www351"
    },
    {
      "count": 25,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "espn"
    },
    {
      "count": 25,
      "is_datatype": false,
      "label": "rdfs",
      "link_num": 3,
      "source": "db",
      "target": "jacom"
    },
    {
      "count": 24,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "inaturalist.observation"
    },
    {
      "count": 24,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www268"
    },
    {
      "count": 24,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "itunes"
    },
    {
      "count": 24,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www229"
    },
    {
      "count": 24,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www314"
    },
    {
      "count": 23,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www143"
    },
    {
      "count": 23,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "playorg"
    },
    {
      "count": 23,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "sgv"
    },
    {
      "count": 23,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www125"
    },
    {
      "count": 23,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www117"
    },
    {
      "count": 22,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www87"
    },
    {
      "count": 22,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "vimeo2"
    },
    {
      "count": 22,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www61"
    },
    {
      "count": 22,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www155"
    },
    {
      "count": 22,
      "is_datatype": false,
      "label": "db",
      "link_num": 2,
      "source": "db",
      "target": "foaf"
    },
    {
      "count": 22,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www380"
    },
    {
      "count": 22,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "open"
    },
    {
      "count": 22,
      "is_datatype": false,
      "label": "db",
      "link_num": -1,
      "source": "db",
      "target": "biorxiv"
    },
    {
      "count": 21,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www101"
    },
    {
      "count": 21,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www38"
    },
    {
      "count": 20,
      "is_datatype": false,
      "label": "foaf",
      "link_num": -2,
      "source": "db",
      "target": "2009-2017"
    },
    {
      "count": 20,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "dc"
    },
    {
      "count": 20,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www461"
    },
    {
      "count": 20,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www309"
    },
    {
      "count": 20,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "vz"
    },
    {
      "count": 19,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "olympics"
    },
    {
      "count": 19,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www255"
    },
    {
      "count": 19,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "motogp"
    },
    {
      "count": 19,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www60"
    },
    {
      "count": 18,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www397"
    },
    {
      "count": 18,
      "is_datatype": false,
      "label": "rdfs",
      "link_num": -5,
      "source": "db",
      "target": "UNKNOWN"
    },
    {
      "count": 18,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www7"
    },
    {
      "count": 18,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www248"
    },
    {
      "count": 18,
      "is_datatype": false,
      "label": "db",
      "link_num": 2,
      "source": "db",
      "target": "schema"
    },
    {
      "count": 18,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "spfl2"
    },
    {
      "count": 18,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www167"
    },
    {
      "count": 17,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www427"
    },
    {
      "count": 17,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www88"
    },
    {
      "count": 17,
      "is_datatype": false,
      "label": "rdfs",
      "link_num": 3,
      "source": "db",
      "target": "itcom"
    },
    {
      "count": 17,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "purl"
    },
    {
      "count": 17,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "ehfcl"
    },
    {
      "count": 17,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "webar2"
    },
    {
      "count": 16,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "noaa"
    },
    {
      "count": 16,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www169"
    },
    {
      "count": 16,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www411"
    },
    {
      "count": 16,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www370"
    },
    {
      "count": 16,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www120"
    },
    {
      "count": 16,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www332"
    },
    {
      "count": 16,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www291"
    },
    {
      "count": 16,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www65"
    },
    {
      "count": 16,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www387"
    },
    {
      "count": 15,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "ligam2"
    },
    {
      "count": 15,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "europeancup"
    },
    {
      "count": 15,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www39"
    },
    {
      "count": 15,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www453"
    },
    {
      "count": 15,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "weborg"
    },
    {
      "count": 15,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "issuu"
    },
    {
      "count": 15,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www359"
    },
    {
      "count": 15,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www242"
    },
    {
      "count": 15,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www300"
    },
    {
      "count": 15,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www34"
    },
    {
      "count": 15,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www185"
    },
    {
      "count": 15,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www271"
    },
    {
      "count": 15,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "eur-lex"
    },
    {
      "count": 14,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "eppo"
    },
    {
      "count": 14,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "vam"
    },
    {
      "count": 14,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www161"
    },
    {
      "count": 14,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www49"
    },
    {
      "count": 13,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "spfl"
    },
    {
      "count": 13,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www344"
    },
    {
      "count": 13,
      "is_datatype": false,
      "label": "foaf",
      "link_num": -2,
      "source": "db",
      "target": "beth"
    },
    {
      "count": 13,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www331"
    },
    {
      "count": 13,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "nlm"
    },
    {
      "count": 13,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "mime"
    },
    {
      "count": 13,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www85"
    },
    {
      "count": 13,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "stats2"
    },
    {
      "count": 13,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "eg"
    },
    {
      "count": 13,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www428"
    },
    {
      "count": 13,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "ghr"
    },
    {
      "count": 13,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www171"
    },
    {
      "count": 12,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www353"
    },
    {
      "count": 12,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www2com"
    },
    {
      "count": 12,
      "is_datatype": false,
      "label": "foaf",
      "link_num": -2,
      "source": "db",
      "target": "bioguide"
    },
    {
      "count": 12,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www465"
    },
    {
      "count": 12,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www149"
    },
    {
      "count": 12,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www447"
    },
    {
      "count": 12,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www127"
    },
    {
      "count": 12,
      "is_datatype": false,
      "label": "foaf",
      "link_num": -2,
      "source": "db",
      "target": "archi4"
    },
    {
      "count": 12,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www105"
    },
    {
      "count": 12,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www163"
    },
    {
      "count": 12,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www338"
    },
    {
      "count": 11,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www418"
    },
    {
      "count": 11,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www147"
    },
    {
      "count": 11,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "wikimapia"
    },
    {
      "count": 11,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www10"
    },
    {
      "count": 11,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www146"
    },
    {
      "count": 11,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www13"
    },
    {
      "count": 11,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www238"
    },
    {
      "count": 10,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "wtafiles"
    },
    {
      "count": 10,
      "is_datatype": false,
      "label": "db",
      "link_num": 1,
      "source": "db",
      "target": "genbank"
    },
    {
      "count": 10,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www75"
    },
    {
      "count": 10,
      "is_datatype": false,
      "label": "foaf",
      "link_num": 2,
      "source": "db",
      "target": "www435"
    }
  ],
  "nodes": [
    {
      "count": 1387635340,
      "name": "db",
      "node_type": "Namespace"
    },
    {
      "count": 129498296,
      "name": "xsd",
      "node_type": "Namespace"
    },
    {
      "count": 99513924,
      "name": "wiki",
      "node_type": "Namespace"
    },
    {
      "count": 86845058,
      "name": "rdf",
      "node_type": "Namespace"
    },
    {
      "count": 47522970,
      "name": "commo4",
      "node_type": "Namespace"
    },
    {
      "count": 18600566,
      "name": "foaf",
      "node_type": "Namespace"
    },
    {
      "count": 12872281,
      "name": "wd",
      "node_type": "Namespace"
    },
    {
      "count": 10169314,
      "name": "freebase",
      "node_type": "Namespace"
    },
    {
      "count": 7085480,
      "name": "UNKNOWN",
      "node_type": "Unknown"
    },
    {
      "count": 5425496,
      "name": "owl",
      "node_type": "Namespace"
    },
    {
      "count": 4877667,
      "name": "yago",
      "node_type": "Namespace"
    },
    {
      "count": 4817848,
      "name": "schema",
      "node_type": "Namespace"
    },
    {
      "count": 4770287,
      "name": "dul",
      "node_type": "Namespace"
    },
    {
      "count": 2193008,
      "name": "skos",
      "node_type": "Namespace"
    },
    {
      "count": 1892617,
      "name": "wgs84",
      "node_type": "Namespace"
    },
    {
      "count": 1349248,
      "name": "web2",
      "node_type": "Namespace"
    },
    {
      "count": 952071,
      "name": "viaf",
      "node_type": "Namespace"
    },
    {
      "count": 596134,
      "name": "umbelrc",
      "node_type": "Namespace"
    },
    {
      "count": 430839,
      "name": "wn20",
      "node_type": "Namespace"
    },
    {
      "count": 357723,
      "name": "books",
      "node_type": "Namespace"
    },
    {
      "count": 310005,
      "name": "archi2",
      "node_type": "Namespace"
    },
    {
      "count": 233963,
      "name": "gnd",
      "node_type": "Namespace"
    },
    {
      "count": 130303,
      "name": "int",
      "node_type": "Namespace"
    },
    {
      "count": 126033,
      "name": "www50",
      "node_type": "Namespace"
    },
    {
      "count": 106498,
      "name": "lgd",
      "node_type": "Namespace"
    },
    {
      "count": 95774,
      "name": "data4",
      "node_type": "Namespace"
    },
    {
      "count": 90889,
      "name": "www71",
      "node_type": "Namespace"
    },
    {
      "count": 89842,
      "name": "www67",
      "node_type": "Namespace"
    },
    {
      "count": 87071,
      "name": "geodata",
      "node_type": "Namespace"
    },
    {
      "count": 71725,
      "name": "www73",
      "node_type": "Namespace"
    },
    {
      "count": 65017,
      "name": "commo2",
      "node_type": "Namespace"
    },
    {
      "count": 62618,
      "name": "archi3",
      "node_type": "Namespace"
    },
    {
      "count": 56453,
      "name": "www62",
      "node_type": "Namespace"
    },
    {
      "count": 50656,
      "name": "newscom",
      "node_type": "Namespace"
    },
    {
      "count": 48763,
      "name": "www146",
      "node_type": "Namespace"
    },
    {
      "count": 48160,
      "name": "bibo",
      "node_type": "Namespace"
    },
    {
      "count": 38786,
      "name": "gadm2",
      "node_type": "Namespace"
    },
    {
      "count": 32180,
      "name": "www138",
      "node_type": "Namespace"
    },
    {
      "count": 30426,
      "name": "www74",
      "node_type": "Namespace"
    },
    {
      "count": 27504,
      "name": "linkedmdb",
      "node_type": "Namespace"
    },
    {
      "count": 27471,
      "name": "www133",
      "node_type": "Namespace"
    },
    {
      "count": 27069,
      "name": "cricketarchive",
      "node_type": "Namespace"
    },
    {
      "count": 26940,
      "name": "sworg",
      "node_type": "Namespace"
    },
    {
      "count": 26927,
      "name": "www72",
      "node_type": "Namespace"
    },
    {
      "count": 26609,
      "name": "www65",
      "node_type": "Namespace"
    },
    {
      "count": 25996,
      "name": "rugby",
      "node_type": "Namespace"
    },
    {
      "count": 25304,
      "name": "www4",
      "node_type": "Namespace"
    },
    {
      "count": 24892,
      "name": "www144",
      "node_type": "Namespace"
    },
    {
      "count": 22764,
      "name": "www63",
      "node_type": "Namespace"
    },
    {
      "count": 22722,
      "name": "www134",
      "node_type": "Namespace"
    },
    {
      "count": 22229,
      "name": "www221",
      "node_type": "Namespace"
    },
    {
      "count": 21502,
      "name": "www128",
      "node_type": "Namespace"
    },
    {
      "count": 19895,
      "name": "www43",
      "node_type": "Namespace"
    },
    {
      "count": 19834,
      "name": "www69",
      "node_type": "Namespace"
    },
    {
      "count": 19079,
      "name": "tms",
      "node_type": "Namespace"
    },
    {
      "count": 18261,
      "name": "apiorg",
      "node_type": "Namespace"
    },
    {
      "count": 17963,
      "name": "www66",
      "node_type": "Namespace"
    },
    {
      "count": 17803,
      "name": "www466",
      "node_type": "Namespace"
    },
    {
      "count": 17534,
      "name": "twitt2",
      "node_type": "Namespace"
    },
    {
      "count": 17459,
      "name": "uk",
      "node_type": "Namespace"
    },
    {
      "count": 17189,
      "name": "www70",
      "node_type": "Namespace"
    },
    {
      "count": 15875,
      "name": "www110",
      "node_type": "Namespace"
    },
    {
      "count": 15092,
      "name": "lobid",
      "node_type": "Namespace"
    },
    {
      "count": 14978,
      "name": "www56",
      "node_type": "Namespace"
    },
    {
      "count": 14580,
      "name": "www58",
      "node_type": "Namespace"
    },
    {
      "count": 14237,
      "name": "www118",
      "node_type": "Namespace"
    },
    {
      "count": 13866,
      "name": "www68",
      "node_type": "Namespace"
    },
    {
      "count": 13805,
      "name": "www125",
      "node_type": "Namespace"
    },
    {
      "count": 13655,
      "name": "www99",
      "node_type": "Namespace"
    },
    {
      "count": 12954,
      "name": "www64",
      "node_type": "Namespace"
    },
    {
      "count": 12864,
      "name": "lod",
      "node_type": "Namespace"
    },
    {
      "count": 12644,
      "name": "www178",
      "node_type": "Namespace"
    },
    {
      "count": 12592,
      "name": "govtrackus",
      "node_type": "Namespace"
    },
    {
      "count": 12267,
      "name": "www173",
      "node_type": "Namespace"
    },
    {
      "count": 11932,
      "name": "www100",
      "node_type": "Namespace"
    },
    {
      "count": 11651,
      "name": "archiinfo",
      "node_type": "Namespace"
    },
    {
      "count": 11589,
      "name": "www57",
      "node_type": "Namespace"
    },
    {
      "count": 11422,
      "name": "www172",
      "node_type": "Namespace"
    },
    {
      "count": 11411,
      "name": "www91",
      "node_type": "Namespace"
    },
    {
      "count": 11271,
      "name": "www59",
      "node_type": "Namespace"
    },
    {
      "count": 11228,
      "name": "eunis2",
      "node_type": "Namespace"
    },
    {
      "count": 11065,
      "name": "www39",
      "node_type": "Namespace"
    },
    {
      "count": 10885,
      "name": "www142",
      "node_type": "Namespace"
    },
    {
      "count": 10712,
      "name": "www106",
      "node_type": "Namespace"
    },
    {
      "count": 10433,
      "name": "www140",
      "node_type": "Namespace"
    },
    {
      "count": 10313,
      "name": "www176",
      "node_type": "Namespace"
    },
    {
      "count": 10291,
      "name": "eu-football",
      "node_type": "Namespace"
    },
    {
      "count": 10184,
      "name": "doi",
      "node_type": "Namespace"
    },
    {
      "count": 10089,
      "name": "data3",
      "node_type": "Namespace"
    },
    {
      "count": 9876,
      "name": "www60",
      "node_type": "Namespace"
    },
    {
      "count": 9729,
      "name": "www139",
      "node_type": "Namespace"
    },
    {
      "count": 9580,
      "name": "www177",
      "node_type": "Namespace"
    },
    {
      "count": 9409,
      "name": "www248",
      "node_type": "Namespace"
    },
    {
      "count": 9210,
      "name": "www36",
      "node_type": "Namespace"
    },
    {
      "count": 9193,
      "name": "github",
      "node_type": "Namespace"
    },
    {
      "count": 8996,
      "name": "www8",
      "node_type": "Namespace"
    },
    {
      "count": 8987,
      "name": "www48",
      "node_type": "Namespace"
    },
    {
      "count": 8983,
      "name": "de",
      "node_type": "Namespace"
    },
    {
      "count": 8971,
      "name": "www171",
      "node_type": "Namespace"
    },
    {
      "count": 8952,
      "name": "www174",
      "node_type": "Namespace"
    },
    {
      "count": 8889,
      "name": "www136",
      "node_type": "Namespace"
    },
    {
      "count": 8828,
      "name": "eclipse",
      "node_type": "Namespace"
    },
    {
      "count": 8696,
      "name": "www52",
      "node_type": "Namespace"
    },
    {
      "count": 8623,
      "name": "news",
      "node_type": "Namespace"
    },
    {
      "count": 8609,
      "name": "www55",
      "node_type": "Namespace"
    },
    {
      "count": 8600,
      "name": "www137",
      "node_type": "Namespace"
    },
    {
      "count": 8276,
      "name": "reports",
      "node_type": "Namespace"
    },
    {
      "count": 8262,
      "name": "stats3",
      "node_type": "Namespace"
    },
    {
      "count": 8210,
      "name": "www175",
      "node_type": "Namespace"
    },
    {
      "count": 8130,
      "name": "www29",
      "node_type": "Namespace"
    },
    {
      "count": 8013,
      "name": "www169",
      "node_type": "Namespace"
    },
    {
      "count": 7950,
      "name": "www166",
      "node_type": "Namespace"
    },
    {
      "count": 7768,
      "name": "mlb",
      "node_type": "Namespace"
    },
    {
      "count": 7716,
      "name": "www260",
      "node_type": "Namespace"
    },
    {
      "count": 7687,
      "name": "globalwordnet",
      "node_type": "Namespace"
    },
    {
      "count": 7684,
      "name": "www286",
      "node_type": "Namespace"
    },
    {
      "count": 7672,
      "name": "www111",
      "node_type": "Namespace"
    },
    {
      "count": 7663,
      "name": "www61",
      "node_type": "Namespace"
    },
    {
      "count": 7492,
      "name": "geonaorg",
      "node_type": "Namespace"
    },
    {
      "count": 7468,
      "name": "babel",
      "node_type": "Namespace"
    },
    {
      "count": 7360,
      "name": "www135",
      "node_type": "Namespace"
    },
    {
      "count": 7314,
      "name": "www97",
      "node_type": "Namespace"
    },
    {
      "count": 7240,
      "name": "www161",
      "node_type": "Namespace"
    },
    {
      "count": 7177,
      "name": "global",
      "node_type": "Namespace"
    },
    {
      "count": 7067,
      "name": "www51",
      "node_type": "Namespace"
    },
    {
      "count": 7041,
      "name": "www116",
      "node_type": "Namespace"
    },
    {
      "count": 7028,
      "name": "www32",
      "node_type": "Namespace"
    },
    {
      "count": 6921,
      "name": "supreme",
      "node_type": "Namespace"
    },
    {
      "count": 6906,
      "name": "maps",
      "node_type": "Namespace"
    },
    {
      "count": 6861,
      "name": "dati",
      "node_type": "Namespace"
    },
    {
      "count": 6679,
      "name": "www49",
      "node_type": "Namespace"
    },
    {
      "count": 6578,
      "name": "www204",
      "node_type": "Namespace"
    },
    {
      "count": 6568,
      "name": "www155",
      "node_type": "Namespace"
    },
    {
      "count": 6565,
      "name": "frcom",
      "node_type": "Namespace"
    },
    {
      "count": 6564,
      "name": "www162",
      "node_type": "Namespace"
    },
    {
      "count": 6445,
      "name": "scholar",
      "node_type": "Namespace"
    },
    {
      "count": 6350,
      "name": "www37",
      "node_type": "Namespace"
    },
    {
      "count": 6289,
      "name": "www143",
      "node_type": "Namespace"
    },
    {
      "count": 6179,
      "name": "www31",
      "node_type": "Namespace"
    },
    {
      "count": 6141,
      "name": "www132",
      "node_type": "Namespace"
    },
    {
      "count": 6123,
      "name": "adatbank",
      "node_type": "Namespace"
    },
    {
      "count": 6069,
      "name": "www141",
      "node_type": "Namespace"
    },
    {
      "count": 6061,
      "name": "www160",
      "node_type": "Namespace"
    },
    {
      "count": 6053,
      "name": "us",
      "node_type": "Namespace"
    },
    {
      "count": 6031,
      "name": "www54",
      "node_type": "Namespace"
    },
    {
      "count": 6017,
      "name": "www151",
      "node_type": "Namespace"
    },
    {
      "count": 6000,
      "name": "www306",
      "node_type": "Namespace"
    },
    {
      "count": 5927,
      "name": "www47",
      "node_type": "Namespace"
    },
    {
      "count": 5885,
      "name": "www3",
      "node_type": "Namespace"
    },
    {
      "count": 5849,
      "name": "eu-fo2",
      "node_type": "Namespace"
    },
    {
      "count": 5845,
      "name": "bookscom",
      "node_type": "Namespace"
    },
    {
      "count": 5844,
      "name": "www259",
      "node_type": "Namespace"
    },
    {
      "count": 5822,
      "name": "data2",
      "node_type": "Namespace"
    },
    {
      "count": 5797,
      "name": "www46",
      "node_type": "Namespace"
    },
    {
      "count": 5779,
      "name": "data5",
      "node_type": "Namespace"
    },
    {
      "count": 5644,
      "name": "www165",
      "node_type": "Namespace"
    },
    {
      "count": 5632,
      "name": "www167",
      "node_type": "Namespace"
    },
    {
      "count": 5550,
      "name": "www182",
      "node_type": "Namespace"
    },
    {
      "count": 5545,
      "name": "www42",
      "node_type": "Namespace"
    },
    {
      "count": 5498,
      "name": "www163",
      "node_type": "Namespace"
    },
    {
      "count": 5494,
      "name": "galli2",
      "node_type": "Namespace"
    },
    {
      "count": 5484,
      "name": "www34",
      "node_type": "Namespace"
    },
    {
      "count": 5433,
      "name": "www369",
      "node_type": "Namespace"
    },
    {
      "count": 5428,
      "name": "www168",
      "node_type": "Namespace"
    },
    {
      "count": 5409,
      "name": "estadisticas",
      "node_type": "Namespace"
    },
    {
      "count": 5353,
      "name": "pdfhost",
      "node_type": "Namespace"
    },
    {
      "count": 5351,
      "name": "undocs",
      "node_type": "Namespace"
    },
    {
      "count": 5272,
      "name": "www159",
      "node_type": "Namespace"
    },
    {
      "count": 5224,
      "name": "match2",
      "node_type": "Namespace"
    },
    {
      "count": 5123,
      "name": "www156",
      "node_type": "Namespace"
    },
    {
      "count": 5091,
      "name": "globoesporte",
      "node_type": "Namespace"
    },
    {
      "count": 5071,
      "name": "www126",
      "node_type": "Namespace"
    },
    {
      "count": 5058,
      "name": "zbw",
      "node_type": "Namespace"
    },
    {
      "count": 5051,
      "name": "archive",
      "node_type": "Namespace"
    },
    {
      "count": 5017,
      "name": "premierliga",
      "node_type": "Namespace"
    },
    {
      "count": 4986,
      "name": "www53",
      "node_type": "Namespace"
    },
    {
      "count": 4949,
      "name": "www157",
      "node_type": "Namespace"
    },
    {
      "count": 4862,
      "name": "www120",
      "node_type": "Namespace"
    },
    {
      "count": 4838,
      "name": "svenskfotboll",
      "node_type": "Namespace"
    },
    {
      "count": 4815,
      "name": "www38",
      "node_type": "Namespace"
    },
    {
      "count": 4806,
      "name": "gd2",
      "node_type": "Namespace"
    },
    {
      "count": 4799,
      "name": "calcio-seriea",
      "node_type": "Namespace"
    },
    {
      "count": 4786,
      "name": "www40",
      "node_type": "Namespace"
    },
    {
      "count": 4699,
      "name": "hdl",
      "node_type": "Namespace"
    },
    {
      "count": 4682,
      "name": "cdn",
      "node_type": "Namespace"
    },
    {
      "count": 4677,
      "name": "nflcdns",
      "node_type": "Namespace"
    },
    {
      "count": 4663,
      "name": "onlin2",
      "node_type": "Namespace"
    },
    {
      "count": 4648,
      "name": "www164",
      "node_type": "Namespace"
    },
    {
      "count": 4607,
      "name": "sites",
      "node_type": "Namespace"
    },
    {
      "count": 4600,
      "name": "www149",
      "node_type": "Namespace"
    },
    {
      "count": 4540,
      "name": "www129",
      "node_type": "Namespace"
    },
    {
      "count": 4521,
      "name": "www35",
      "node_type": "Namespace"
    },
    {
      "count": 4468,
      "name": "statsorg",
      "node_type": "Namespace"
    },
    {
      "count": 4444,
      "name": "data6",
      "node_type": "Namespace"
    },
    {
      "count": 4436,
      "name": "hathitrust",
      "node_type": "Namespace"
    },
    {
      "count": 4416,
      "name": "www153",
      "node_type": "Namespace"
    },
    {
      "count": 4410,
      "name": "www41",
      "node_type": "Namespace"
    },
    {
      "count": 4342,
      "name": "actas",
      "node_type": "Namespace"
    },
    {
      "count": 4328,
      "name": "www7",
      "node_type": "Namespace"
    },
    {
      "count": 4288,
      "name": "www206",
      "node_type": "Namespace"
    },
    {
      "count": 4284,
      "name": "www258",
      "node_type": "Namespace"
    },
    {
      "count": 4282,
      "name": "ucjeps",
      "node_type": "Namespace"
    },
    {
      "count": 4274,
      "name": "www33",
      "node_type": "Namespace"
    },
    {
      "count": 4258,
      "name": "www188",
      "node_type": "Namespace"
    },
    {
      "count": 4251,
      "name": "nla",
      "node_type": "Namespace"
    },
    {
      "count": 4250,
      "name": "www207",
      "node_type": "Namespace"
    },
    {
      "count": 4224,
      "name": "coludata",
      "node_type": "Namespace"
    },
    {
      "count": 4214,
      "name": "www256",
      "node_type": "Namespace"
    },
    {
      "count": 4194,
      "name": "itcom",
      "node_type": "Namespace"
    },
    {
      "count": 4186,
      "name": "www285",
      "node_type": "Namespace"
    },
    {
      "count": 4129,
      "name": "www23",
      "node_type": "Namespace"
    },
    {
      "count": 4124,
      "name": "www154",
      "node_type": "Namespace"
    },
    {
      "count": 4116,
      "name": "www257",
      "node_type": "Namespace"
    },
    {
      "count": 4115,
      "name": "mediacom",
      "node_type": "Namespace"
    },
    {
      "count": 4114,
      "name": "www123",
      "node_type": "Namespace"
    },
    {
      "count": 4102,
      "name": "politicalgraveyard",
      "node_type": "Namespace"
    },
    {
      "count": 4086,
      "name": "www205",
      "node_type": "Namespace"
    },
    {
      "count": 4050,
      "name": "esdbpr",
      "node_type": "Namespace"
    },
    {
      "count": 4036,
      "name": "www30",
      "node_type": "Namespace"
    },
    {
      "count": 4023,
      "name": "www203",
      "node_type": "Namespace"
    },
    {
      "count": 4012,
      "name": "www148",
      "node_type": "Namespace"
    },
    {
      "count": 3999,
      "name": "www199",
      "node_type": "Namespace"
    },
    {
      "count": 3982,
      "name": "calphotos",
      "node_type": "Namespace"
    },
    {
      "count": 3891,
      "name": "www231",
      "node_type": "Namespace"
    },
    {
      "count": 3867,
      "name": "vimeo",
      "node_type": "Namespace"
    },
    {
      "count": 3850,
      "name": "www82",
      "node_type": "Namespace"
    },
    {
      "count": 3838,
      "name": "www201",
      "node_type": "Namespace"
    },
    {
      "count": 3832,
      "name": "www224",
      "node_type": "Namespace"
    },
    {
      "count": 3816,
      "name": "www12",
      "node_type": "Namespace"
    },
    {
      "count": 3802,
      "name": "www303",
      "node_type": "Namespace"
    },
    {
      "count": 3766,
      "name": "www121",
      "node_type": "Namespace"
    },
    {
      "count": 3750,
      "name": "drive",
      "node_type": "Namespace"
    },
    {
      "count": 3737,
      "name": "apcbg",
      "node_type": "Namespace"
    },
    {
      "count": 3695,
      "name": "www18",
      "node_type": "Namespace"
    },
    {
      "count": 3686,
      "name": "www11",
      "node_type": "Namespace"
    },
    {
      "count": 3665,
      "name": "whc",
      "node_type": "Namespace"
    },
    {
      "count": 3660,
      "name": "www196",
      "node_type": "Namespace"
    },
    {
      "count": 3658,
      "name": "query",
      "node_type": "Namespace"
    },
    {
      "count": 3658,
      "name": "www28",
      "node_type": "Namespace"
    },
    {
      "count": 3593,
      "name": "www101",
      "node_type": "Namespace"
    },
    {
      "count": 3550,
      "name": "espn",
      "node_type": "Namespace"
    },
    {
      "count": 3528,
      "name": "atlasuk",
      "node_type": "Namespace"
    },
    {
      "count": 3519,
      "name": "bacdive",
      "node_type": "Namespace"
    },
    {
      "count": 3516,
      "name": "linkorg",
      "node_type": "Namespace"
    },
    {
      "count": 3511,
      "name": "www228",
      "node_type": "Namespace"
    },
    {
      "count": 3507,
      "name": "ratings",
      "node_type": "Namespace"
    },
    {
      "count": 3505,
      "name": "kunishitei",
      "node_type": "Namespace"
    },
    {
      "count": 3483,
      "name": "nl",
      "node_type": "Namespace"
    },
    {
      "count": 3479,
      "name": "www44",
      "node_type": "Namespace"
    },
    {
      "count": 3470,
      "name": "www264",
      "node_type": "Namespace"
    },
    {
      "count": 3468,
      "name": "www208",
      "node_type": "Namespace"
    },
    {
      "count": 3452,
      "name": "www220",
      "node_type": "Namespace"
    },
    {
      "count": 3448,
      "name": "www185",
      "node_type": "Namespace"
    },
    {
      "count": 3443,
      "name": "trove",
      "node_type": "Namespace"
    },
    {
      "count": 3437,
      "name": "www113",
      "node_type": "Namespace"
    },
    {
      "count": 3422,
      "name": "www202",
      "node_type": "Namespace"
    },
    {
      "count": 3381,
      "name": "svcom",
      "node_type": "Namespace"
    },
    {
      "count": 3370,
      "name": "plants",
      "node_type": "Namespace"
    },
    {
      "count": 3366,
      "name": "zenodo.record",
      "node_type": "Namespace"
    },
    {
      "count": 3359,
      "name": "jacom",
      "node_type": "Namespace"
    },
    {
      "count": 3346,
      "name": "www115",
      "node_type": "Namespace"
    },
    {
      "count": 3343,
      "name": "timesmachine",
      "node_type": "Namespace"
    },
    {
      "count": 3322,
      "name": "gallica",
      "node_type": "Namespace"
    },
    {
      "count": 3322,
      "name": "www27",
      "node_type": "Namespace"
    },
    {
      "count": 3278,
      "name": "www158",
      "node_type": "Namespace"
    },
    {
      "count": 3265,
      "name": "www105",
      "node_type": "Namespace"
    },
    {
      "count": 3261,
      "name": "dx",
      "node_type": "Namespace"
    },
    {
      "count": 3215,
      "name": "artuk",
      "node_type": "Namespace"
    },
    {
      "count": 3215,
      "name": "www6",
      "node_type": "Namespace"
    },
    {
      "count": 3211,
      "name": "matchcenter",
      "node_type": "Namespace"
    },
    {
      "count": 3194,
      "name": "www189",
      "node_type": "Namespace"
    },
    {
      "count": 3189,
      "name": "www194",
      "node_type": "Namespace"
    },
    {
      "count": 3179,
      "name": "www225",
      "node_type": "Namespace"
    },
    {
      "count": 3168,
      "name": "www195",
      "node_type": "Namespace"
    },
    {
      "count": 3161,
      "name": "www103",
      "node_type": "Namespace"
    },
    {
      "count": 3135,
      "name": "ru2",
      "node_type": "Namespace"
    },
    {
      "count": 3134,
      "name": "www22",
      "node_type": "Namespace"
    },
    {
      "count": 3126,
      "name": "pt",
      "node_type": "Namespace"
    },
    {
      "count": 3106,
      "name": "www192",
      "node_type": "Namespace"
    },
    {
      "count": 3099,
      "name": "ecowlim",
      "node_type": "Namespace"
    },
    {
      "count": 3099,
      "name": "www87",
      "node_type": "Namespace"
    },
    {
      "count": 3095,
      "name": "www112",
      "node_type": "Namespace"
    },
    {
      "count": 3091,
      "name": "pl",
      "node_type": "Namespace"
    },
    {
      "count": 3088,
      "name": "www21",
      "node_type": "Namespace"
    },
    {
      "count": 3076,
      "name": "www114",
      "node_type": "Namespace"
    },
    {
      "count": 3072,
      "name": "caselaw",
      "node_type": "Namespace"
    },
    {
      "count": 3071,
      "name": "www119",
      "node_type": "Namespace"
    },
    {
      "count": 3068,
      "name": "www187",
      "node_type": "Namespace"
    },
    {
      "count": 3056,
      "name": "www127",
      "node_type": "Namespace"
    },
    {
      "count": 3044,
      "name": "www14",
      "node_type": "Namespace"
    },
    {
      "count": 3037,
      "name": "www89",
      "node_type": "Namespace"
    },
    {
      "count": 3026,
      "name": "www368",
      "node_type": "Namespace"
    },
    {
      "count": 3024,
      "name": "www95",
      "node_type": "Namespace"
    },
    {
      "count": 3021,
      "name": "www107",
      "node_type": "Namespace"
    },
    {
      "count": 3008,
      "name": "www190",
      "node_type": "Namespace"
    },
    {
      "count": 2996,
      "name": "ark",
      "node_type": "Namespace"
    },
    {
      "count": 2993,
      "name": "catalog",
      "node_type": "Namespace"
    },
    {
      "count": 2980,
      "name": "adminnet",
      "node_type": "Namespace"
    },
    {
      "count": 2967,
      "name": "www235",
      "node_type": "Namespace"
    },
    {
      "count": 2951,
      "name": "www108",
      "node_type": "Namespace"
    },
    {
      "count": 2938,
      "name": "www2",
      "node_type": "Namespace"
    },
    {
      "count": 2936,
      "name": "www191",
      "node_type": "Namespace"
    },
    {
      "count": 2892,
      "name": "www197",
      "node_type": "Namespace"
    },
    {
      "count": 2887,
      "name": "www26",
      "node_type": "Namespace"
    },
    {
      "count": 2875,
      "name": "adatb2",
      "node_type": "Namespace"
    },
    {
      "count": 2871,
      "name": "www9",
      "node_type": "Namespace"
    },
    {
      "count": 2867,
      "name": "www45",
      "node_type": "Namespace"
    },
    {
      "count": 2851,
      "name": "www88",
      "node_type": "Namespace"
    },
    {
      "count": 2842,
      "name": "www131",
      "node_type": "Namespace"
    },
    {
      "count": 2838,
      "name": "www130",
      "node_type": "Namespace"
    },
    {
      "count": 2832,
      "name": "babel2",
      "node_type": "Namespace"
    },
    {
      "count": 2797,
      "name": "www93",
      "node_type": "Namespace"
    },
    {
      "count": 2794,
      "name": "www152",
      "node_type": "Namespace"
    },
    {
      "count": 2784,
      "name": "www200",
      "node_type": "Namespace"
    },
    {
      "count": 2777,
      "name": "sports",
      "node_type": "Namespace"
    },
    {
      "count": 2746,
      "name": "www245",
      "node_type": "Namespace"
    },
    {
      "count": 2723,
      "name": "commo3",
      "node_type": "Namespace"
    },
    {
      "count": 2710,
      "name": "www255",
      "node_type": "Namespace"
    },
    {
      "count": 2671,
      "name": "onlinelibrary",
      "node_type": "Namespace"
    },
    {
      "count": 2660,
      "name": "penelope",
      "node_type": "Namespace"
    },
    {
      "count": 2658,
      "name": "www17",
      "node_type": "Namespace"
    },
    {
      "count": 2629,
      "name": "www96",
      "node_type": "Namespace"
    },
    {
      "count": 2618,
      "name": "www230",
      "node_type": "Namespace"
    },
    {
      "count": 2610,
      "name": "www278",
      "node_type": "Namespace"
    },
    {
      "count": 2608,
      "name": "www94",
      "node_type": "Namespace"
    },
    {
      "count": 2604,
      "name": "www150",
      "node_type": "Namespace"
    },
    {
      "count": 2602,
      "name": "www109",
      "node_type": "Namespace"
    },
    {
      "count": 2596,
      "name": "www184",
      "node_type": "Namespace"
    },
    {
      "count": 2585,
      "name": "www13",
      "node_type": "Namespace"
    },
    {
      "count": 2578,
      "name": "globalsportsarchive",
      "node_type": "Namespace"
    },
    {
      "count": 2573,
      "name": "www117",
      "node_type": "Namespace"
    },
    {
      "count": 2572,
      "name": "www253",
      "node_type": "Namespace"
    },
    {
      "count": 2571,
      "name": "www79",
      "node_type": "Namespace"
    },
    {
      "count": 2563,
      "name": "historiawisly",
      "node_type": "Namespace"
    },
    {
      "count": 2549,
      "name": "www243",
      "node_type": "Namespace"
    },
    {
      "count": 2545,
      "name": "www244",
      "node_type": "Namespace"
    },
    {
      "count": 2543,
      "name": "basketball",
      "node_type": "Namespace"
    },
    {
      "count": 2542,
      "name": "www20",
      "node_type": "Namespace"
    },
    {
      "count": 2540,
      "name": "www211",
      "node_type": "Namespace"
    },
    {
      "count": 2529,
      "name": "www229",
      "node_type": "Namespace"
    },
    {
      "count": 2524,
      "name": "www10",
      "node_type": "Namespace"
    },
    {
      "count": 2509,
      "name": "wikimapia",
      "node_type": "Namespace"
    },
    {
      "count": 2506,
      "name": "www193",
      "node_type": "Namespace"
    },
    {
      "count": 2499,
      "name": "www284",
      "node_type": "Namespace"
    },
    {
      "count": 2499,
      "name": "www86",
      "node_type": "Namespace"
    },
    {
      "count": 2482,
      "name": "www215",
      "node_type": "Namespace"
    },
    {
      "count": 2478,
      "name": "tools",
      "node_type": "Namespace"
    },
    {
      "count": 2470,
      "name": "itunes",
      "node_type": "Namespace"
    },
    {
      "count": 2468,
      "name": "www90",
      "node_type": "Namespace"
    },
    {
      "count": 2460,
      "name": "www461",
      "node_type": "Namespace"
    },
    {
      "count": 2445,
      "name": "www15",
      "node_type": "Namespace"
    },
    {
      "count": 2418,
      "name": "olympics",
      "node_type": "Namespace"
    },
    {
      "count": 2399,
      "name": "soundcloud",
      "node_type": "Namespace"
    },
    {
      "count": 2387,
      "name": "www98",
      "node_type": "Namespace"
    },
    {
      "count": 2361,
      "name": "fibalivestats",
      "node_type": "Namespace"
    },
    {
      "count": 2359,
      "name": "www242",
      "node_type": "Namespace"
    },
    {
      "count": 2339,
      "name": "vocabcom",
      "node_type": "Namespace"
    },
    {
      "count": 2325,
      "name": "www226",
      "node_type": "Namespace"
    },
    {
      "count": 2318,
      "name": "law",
      "node_type": "Namespace"
    },
    {
      "count": 2310,
      "name": "www104",
      "node_type": "Namespace"
    },
    {
      "count": 2308,
      "name": "www179",
      "node_type": "Namespace"
    },
    {
      "count": 2292,
      "name": "data7",
      "node_type": "Namespace"
    },
    {
      "count": 2291,
      "name": "www217",
      "node_type": "Namespace"
    },
    {
      "count": 2279,
      "name": "trove2",
      "node_type": "Namespace"
    },
    {
      "count": 2270,
      "name": "www147",
      "node_type": "Namespace"
    },
    {
      "count": 2247,
      "name": "libmma",
      "node_type": "Namespace"
    },
    {
      "count": 2242,
      "name": "www246",
      "node_type": "Namespace"
    },
    {
      "count": 2235,
      "name": "census",
      "node_type": "Namespace"
    },
    {
      "count": 2232,
      "name": "www279",
      "node_type": "Namespace"
    },
    {
      "count": 2221,
      "name": "zh",
      "node_type": "Namespace"
    },
    {
      "count": 2218,
      "name": "www80",
      "node_type": "Namespace"
    },
    {
      "count": 2211,
      "name": "gutenberg",
      "node_type": "Namespace"
    },
    {
      "count": 2206,
      "name": "www-old",
      "node_type": "Namespace"
    },
    {
      "count": 2197,
      "name": "www241",
      "node_type": "Namespace"
    },
    {
      "count": 2191,
      "name": "ligamx",
      "node_type": "Namespace"
    },
    {
      "count": 2190,
      "name": "www296",
      "node_type": "Namespace"
    },
    {
      "count": 2187,
      "name": "topostext",
      "node_type": "Namespace"
    },
    {
      "count": 2184,
      "name": "www76",
      "node_type": "Namespace"
    },
    {
      "count": 2182,
      "name": "www145",
      "node_type": "Namespace"
    },
    {
      "count": 2176,
      "name": "www5",
      "node_type": "Namespace"
    },
    {
      "count": 2174,
      "name": "www223",
      "node_type": "Namespace"
    },
    {
      "count": 2169,
      "name": "www81",
      "node_type": "Namespace"
    },
    {
      "count": 2168,
      "name": "www16",
      "node_type": "Namespace"
    },
    {
      "count": 2163,
      "name": "www19",
      "node_type": "Namespace"
    },
    {
      "count": 2149,
      "name": "www77",
      "node_type": "Namespace"
    },
    {
      "count": 2141,
      "name": "psephos",
      "node_type": "Namespace"
    },
    {
      "count": 2141,
      "name": "www124",
      "node_type": "Namespace"
    },
    {
      "count": 2137,
      "name": "historicengland",
      "node_type": "Namespace"
    },
    {
      "count": 2136,
      "name": "www254",
      "node_type": "Namespace"
    },
    {
      "count": 2134,
      "name": "www227",
      "node_type": "Namespace"
    },
    {
      "count": 2124,
      "name": "webarchive",
      "node_type": "Namespace"
    },
    {
      "count": 2121,
      "name": "soccernet",
      "node_type": "Namespace"
    },
    {
      "count": 2119,
      "name": "eredivisie",
      "node_type": "Namespace"
    },
    {
      "count": 2105,
      "name": "runeberg",
      "node_type": "Namespace"
    },
    {
      "count": 2093,
      "name": "beth",
      "node_type": "Namespace"
    },
    {
      "count": 2089,
      "name": "www287",
      "node_type": "Namespace"
    },
    {
      "count": 2064,
      "name": "stats2",
      "node_type": "Namespace"
    },
    {
      "count": 2058,
      "name": "www214",
      "node_type": "Namespace"
    },
    {
      "count": 2057,
      "name": "www218",
      "node_type": "Namespace"
    },
    {
      "count": 2057,
      "name": "www75",
      "node_type": "Namespace"
    },
    {
      "count": 2048,
      "name": "scores",
      "node_type": "Namespace"
    },
    {
      "count": 2028,
      "name": "www24",
      "node_type": "Namespace"
    },
    {
      "count": 2022,
      "name": "samuraiblue",
      "node_type": "Namespace"
    },
    {
      "count": 2021,
      "name": "www186",
      "node_type": "Namespace"
    },
    {
      "count": 2020,
      "name": "www233",
      "node_type": "Namespace"
    },
    {
      "count": 2019,
      "name": "www250",
      "node_type": "Namespace"
    },
    {
      "count": 2018,
      "name": "www302",
      "node_type": "Namespace"
    },
    {
      "count": 2016,
      "name": "ceb",
      "node_type": "Namespace"
    },
    {
      "count": 2016,
      "name": "myspaorg",
      "node_type": "Namespace"
    },
    {
      "count": 2012,
      "name": "ghostarchive",
      "node_type": "Namespace"
    },
    {
      "count": 2010,
      "name": "eurohockey",
      "node_type": "Namespace"
    },
    {
      "count": 2010,
      "name": "www261",
      "node_type": "Namespace"
    },
    {
      "count": 2004,
      "name": "www216",
      "node_type": "Namespace"
    },
    {
      "count": 2002,
      "name": "vimeo2",
      "node_type": "Namespace"
    },
    {
      "count": 1970,
      "name": "content-aus",
      "node_type": "Namespace"
    },
    {
      "count": 1969,
      "name": "arxiv",
      "node_type": "Namespace"
    },
    {
      "count": 1969,
      "name": "www219",
      "node_type": "Namespace"
    },
    {
      "count": 1968,
      "name": "www239",
      "node_type": "Namespace"
    },
    {
      "count": 1965,
      "name": "www232",
      "node_type": "Namespace"
    },
    {
      "count": 1962,
      "name": "www222",
      "node_type": "Namespace"
    },
    {
      "count": 1957,
      "name": "openlibrary",
      "node_type": "Namespace"
    },
    {
      "count": 1956,
      "name": "druginfo",
      "node_type": "Namespace"
    },
    {
      "count": 1956,
      "name": "www324",
      "node_type": "Namespace"
    },
    {
      "count": 1949,
      "name": "www251",
      "node_type": "Namespace"
    },
    {
      "count": 1941,
      "name": "www122",
      "node_type": "Namespace"
    },
    {
      "count": 1934,
      "name": "hockeyaustralia",
      "node_type": "Namespace"
    },
    {
      "count": 1927,
      "name": "spfl2",
      "node_type": "Namespace"
    },
    {
      "count": 1918,
      "name": "www240",
      "node_type": "Namespace"
    },
    {
      "count": 1917,
      "name": "hemeroteca-paginas",
      "node_type": "Namespace"
    },
    {
      "count": 1915,
      "name": "www331",
      "node_type": "Namespace"
    },
    {
      "count": 1913,
      "name": "www210",
      "node_type": "Namespace"
    },
    {
      "count": 1897,
      "name": "www280",
      "node_type": "Namespace"
    },
    {
      "count": 1892,
      "name": "uboat",
      "node_type": "Namespace"
    },
    {
      "count": 1891,
      "name": "www181",
      "node_type": "Namespace"
    },
    {
      "count": 1887,
      "name": "www238",
      "node_type": "Namespace"
    },
    {
      "count": 1886,
      "name": "www270",
      "node_type": "Namespace"
    },
    {
      "count": 1867,
      "name": "www378",
      "node_type": "Namespace"
    },
    {
      "count": 1850,
      "name": "www439",
      "node_type": "Namespace"
    },
    {
      "count": 1841,
      "name": "www448",
      "node_type": "Namespace"
    },
    {
      "count": 1836,
      "name": "www213",
      "node_type": "Namespace"
    },
    {
      "count": 1834,
      "name": "www371",
      "node_type": "Namespace"
    },
    {
      "count": 1833,
      "name": "www438",
      "node_type": "Namespace"
    },
    {
      "count": 1826,
      "name": "rugby2",
      "node_type": "Namespace"
    },
    {
      "count": 1826,
      "name": "www435",
      "node_type": "Namespace"
    },
    {
      "count": 1817,
      "name": "journals",
      "node_type": "Namespace"
    },
    {
      "count": 1815,
      "name": "articles",
      "node_type": "Namespace"
    },
    {
      "count": 1807,
      "name": "www212",
      "node_type": "Namespace"
    },
    {
      "count": 1803,
      "name": "fbref",
      "node_type": "Namespace"
    },
    {
      "count": 1803,
      "name": "www329",
      "node_type": "Namespace"
    },
    {
      "count": 1793,
      "name": "www83",
      "node_type": "Namespace"
    },
    {
      "count": 1792,
      "name": "academic",
      "node_type": "Namespace"
    },
    {
      "count": 1791,
      "name": "data9",
      "node_type": "Namespace"
    },
    {
      "count": 1787,
      "name": "www276",
      "node_type": "Namespace"
    },
    {
      "count": 1786,
      "name": "obswww",
      "node_type": "Namespace"
    },
    {
      "count": 1781,
      "name": "www25",
      "node_type": "Namespace"
    },
    {
      "count": 1776,
      "name": "www332",
      "node_type": "Namespace"
    },
    {
      "count": 1774,
      "name": "bwf",
      "node_type": "Namespace"
    },
    {
      "count": 1774,
      "name": "esporte",
      "node_type": "Namespace"
    },
    {
      "count": 1770,
      "name": "www322",
      "node_type": "Namespace"
    },
    {
      "count": 1764,
      "name": "biodiversitylibrary",
      "node_type": "Namespace"
    },
    {
      "count": 1762,
      "name": "issuu",
      "node_type": "Namespace"
    },
    {
      "count": 1759,
      "name": "www283",
      "node_type": "Namespace"
    },
    {
      "count": 1757,
      "name": "www272",
      "node_type": "Namespace"
    },
    {
      "count": 1756,
      "name": "osmrel",
      "node_type": "Namespace"
    },
    {
      "count": 1755,
      "name": "findarticles",
      "node_type": "Namespace"
    },
    {
      "count": 1754,
      "name": "www277",
      "node_type": "Namespace"
    },
    {
      "count": 1753,
      "name": "www456",
      "node_type": "Namespace"
    },
    {
      "count": 1750,
      "name": "www266",
      "node_type": "Namespace"
    },
    {
      "count": 1747,
      "name": "www273",
      "node_type": "Namespace"
    },
    {
      "count": 1745,
      "name": "www170",
      "node_type": "Namespace"
    },
    {
      "count": 1744,
      "name": "www299",
      "node_type": "Namespace"
    },
    {
      "count": 1744,
      "name": "www445",
      "node_type": "Namespace"
    },
    {
      "count": 1741,
      "name": "muse",
      "node_type": "Namespace"
    },
    {
      "count": 1733,
      "name": "www319",
      "node_type": "Namespace"
    },
    {
      "count": 1733,
      "name": "www427",
      "node_type": "Namespace"
    },
    {
      "count": 1733,
      "name": "www428",
      "node_type": "Namespace"
    },
    {
      "count": 1731,
      "name": "dare",
      "node_type": "Namespace"
    },
    {
      "count": 1731,
      "name": "designatedsites",
      "node_type": "Namespace"
    },
    {
      "count": 1730,
      "name": "www237",
      "node_type": "Namespace"
    },
    {
      "count": 1724,
      "name": "open",
      "node_type": "Namespace"
    },
    {
      "count": 1723,
      "name": "hdl2",
      "node_type": "Namespace"
    },
    {
      "count": 1720,
      "name": "www252",
      "node_type": "Namespace"
    },
    {
      "count": 1717,
      "name": "www282",
      "node_type": "Namespace"
    },
    {
      "count": 1712,
      "name": "imslp",
      "node_type": "Namespace"
    },
    {
      "count": 1710,
      "name": "www274",
      "node_type": "Namespace"
    },
    {
      "count": 1708,
      "name": "www271",
      "node_type": "Namespace"
    },
    {
      "count": 1707,
      "name": "www198",
      "node_type": "Namespace"
    },
    {
      "count": 1706,
      "name": "vam",
      "node_type": "Namespace"
    },
    {
      "count": 1695,
      "name": "msrmaps",
      "node_type": "Namespace"
    },
    {
      "count": 1690,
      "name": "www262",
      "node_type": "Namespace"
    },
    {
      "count": 1689,
      "name": "wwwcom",
      "node_type": "Namespace"
    },
    {
      "count": 1688,
      "name": "quod",
      "node_type": "Namespace"
    },
    {
      "count": 1688,
      "name": "www408",
      "node_type": "Namespace"
    },
    {
      "count": 1686,
      "name": "www290",
      "node_type": "Namespace"
    },
    {
      "count": 1684,
      "name": "www416",
      "node_type": "Namespace"
    },
    {
      "count": 1674,
      "name": "www301",
      "node_type": "Namespace"
    },
    {
      "count": 1669,
      "name": "www305",
      "node_type": "Namespace"
    },
    {
      "count": 1668,
      "name": "www437",
      "node_type": "Namespace"
    },
    {
      "count": 1667,
      "name": "fr",
      "node_type": "Namespace"
    },
    {
      "count": 1666,
      "name": "www",
      "node_type": "Namespace"
    },
    {
      "count": 1666,
      "name": "www209",
      "node_type": "Namespace"
    },
    {
      "count": 1664,
      "name": "www432",
      "node_type": "Namespace"
    },
    {
      "count": 1663,
      "name": "www304",
      "node_type": "Namespace"
    },
    {
      "count": 1661,
      "name": "m",
      "node_type": "Namespace"
    },
    {
      "count": 1659,
      "name": "ourairports",
      "node_type": "Namespace"
    },
    {
      "count": 1656,
      "name": "www268",
      "node_type": "Namespace"
    },
    {
      "count": 1655,
      "name": "ecnet",
      "node_type": "Namespace"
    },
    {
      "count": 1649,
      "name": "www298",
      "node_type": "Namespace"
    },
    {
      "count": 1647,
      "name": "ligam2",
      "node_type": "Namespace"
    },
    {
      "count": 1644,
      "name": "www247",
      "node_type": "Namespace"
    },
    {
      "count": 1644,
      "name": "www356",
      "node_type": "Namespace"
    },
    {
      "count": 1640,
      "name": "humantfs",
      "node_type": "Namespace"
    },
    {
      "count": 1632,
      "name": "www418",
      "node_type": "Namespace"
    },
    {
      "count": 1632,
      "name": "www436",
      "node_type": "Namespace"
    },
    {
      "count": 1631,
      "name": "upl",
      "node_type": "Namespace"
    },
    {
      "count": 1629,
      "name": "www434",
      "node_type": "Namespace"
    },
    {
      "count": 1622,
      "name": "magic",
      "node_type": "Namespace"
    },
    {
      "count": 1615,
      "name": "www300",
      "node_type": "Namespace"
    },
    {
      "count": 1610,
      "name": "indiarailinfo",
      "node_type": "Namespace"
    },
    {
      "count": 1608,
      "name": "www364",
      "node_type": "Namespace"
    },
    {
      "count": 1607,
      "name": "hrnogomet",
      "node_type": "Namespace"
    },
    {
      "count": 1606,
      "name": "kulturarvsdata",
      "node_type": "Namespace"
    },
    {
      "count": 1603,
      "name": "copernix",
      "node_type": "Namespace"
    },
    {
      "count": 1601,
      "name": "www384",
      "node_type": "Namespace"
    },
    {
      "count": 1599,
      "name": "allworldcup",
      "node_type": "Namespace"
    },
    {
      "count": 1598,
      "name": "loksabhaph",
      "node_type": "Namespace"
    },
    {
      "count": 1592,
      "name": "player",
      "node_type": "Namespace"
    },
    {
      "count": 1587,
      "name": "www180",
      "node_type": "Namespace"
    },
    {
      "count": 1579,
      "name": "www433",
      "node_type": "Namespace"
    },
    {
      "count": 1578,
      "name": "jalgpall",
      "node_type": "Namespace"
    },
    {
      "count": 1576,
      "name": "adsabs",
      "node_type": "Namespace"
    },
    {
      "count": 1575,
      "name": "webargov",
      "node_type": "Namespace"
    },
    {
      "count": 1573,
      "name": "www249",
      "node_type": "Namespace"
    },
    {
      "count": 1570,
      "name": "purl2",
      "node_type": "Namespace"
    },
    {
      "count": 1570,
      "name": "www293",
      "node_type": "Namespace"
    },
    {
      "count": 1569,
      "name": "www292",
      "node_type": "Namespace"
    },
    {
      "count": 1568,
      "name": "www265",
      "node_type": "Namespace"
    },
    {
      "count": 1563,
      "name": "www85",
      "node_type": "Namespace"
    },
    {
      "count": 1559,
      "name": "www327",
      "node_type": "Namespace"
    },
    {
      "count": 1556,
      "name": "www431",
      "node_type": "Namespace"
    },
    {
      "count": 1552,
      "name": "referenceworks",
      "node_type": "Namespace"
    },
    {
      "count": 1547,
      "name": "waterpolo",
      "node_type": "Namespace"
    },
    {
      "count": 1546,
      "name": "www429",
      "node_type": "Namespace"
    },
    {
      "count": 1543,
      "name": "www269",
      "node_type": "Namespace"
    },
    {
      "count": 1538,
      "name": "164",
      "node_type": "Namespace"
    },
    {
      "count": 1538,
      "name": "www430",
      "node_type": "Namespace"
    },
    {
      "count": 1530,
      "name": "www452",
      "node_type": "Namespace"
    },
    {
      "count": 1529,
      "name": "paperspast",
      "node_type": "Namespace"
    },
    {
      "count": 1528,
      "name": "translate",
      "node_type": "Namespace"
    },
    {
      "count": 1527,
      "name": "www295",
      "node_type": "Namespace"
    },
    {
      "count": 1526,
      "name": "muse2",
      "node_type": "Namespace"
    },
    {
      "count": 1519,
      "name": "www340",
      "node_type": "Namespace"
    },
    {
      "count": 1515,
      "name": "hdlnet",
      "node_type": "Namespace"
    },
    {
      "count": 1501,
      "name": "canpl",
      "node_type": "Namespace"
    },
    {
      "count": 1500,
      "name": "gd22",
      "node_type": "Namespace"
    },
    {
      "count": 1498,
      "name": "www314",
      "node_type": "Namespace"
    },
    {
      "count": 1490,
      "name": "www326",
      "node_type": "Namespace"
    },
    {
      "count": 1489,
      "name": "www297",
      "node_type": "Namespace"
    },
    {
      "count": 1489,
      "name": "www425",
      "node_type": "Namespace"
    },
    {
      "count": 1486,
      "name": "wtafinet",
      "node_type": "Namespace"
    },
    {
      "count": 1486,
      "name": "www417",
      "node_type": "Namespace"
    },
    {
      "count": 1486,
      "name": "www426",
      "node_type": "Namespace"
    },
    {
      "count": 1482,
      "name": "datatw",
      "node_type": "Namespace"
    },
    {
      "count": 1482,
      "name": "www422",
      "node_type": "Namespace"
    },
    {
      "count": 1476,
      "name": "www455",
      "node_type": "Namespace"
    },
    {
      "count": 1470,
      "name": "webar2",
      "node_type": "Namespace"
    },
    {
      "count": 1468,
      "name": "eci",
      "node_type": "Namespace"
    },
    {
      "count": 1466,
      "name": "docs",
      "node_type": "Namespace"
    },
    {
      "count": 1464,
      "name": "www413",
      "node_type": "Namespace"
    },
    {
      "count": 1463,
      "name": "www102",
      "node_type": "Namespace"
    },
    {
      "count": 1462,
      "name": "www343",
      "node_type": "Namespace"
    },
    {
      "count": 1459,
      "name": "www288",
      "node_type": "Namespace"
    },
    {
      "count": 1457,
      "name": "memory",
      "node_type": "Namespace"
    },
    {
      "count": 1454,
      "name": "weborg",
      "node_type": "Namespace"
    },
    {
      "count": 1454,
      "name": "www334",
      "node_type": "Namespace"
    },
    {
      "count": 1453,
      "name": "www84",
      "node_type": "Namespace"
    },
    {
      "count": 1452,
      "name": "ballotpedia",
      "node_type": "Namespace"
    },
    {
      "count": 1449,
      "name": "www313",
      "node_type": "Namespace"
    },
    {
      "count": 1448,
      "name": "www294",
      "node_type": "Namespace"
    },
    {
      "count": 1446,
      "name": "wtafiles",
      "node_type": "Namespace"
    },
    {
      "count": 1442,
      "name": "www328",
      "node_type": "Namespace"
    },
    {
      "count": 1441,
      "name": "host",
      "node_type": "Namespace"
    },
    {
      "count": 1440,
      "name": "www423",
      "node_type": "Namespace"
    },
    {
      "count": 1438,
      "name": "www339",
      "node_type": "Namespace"
    },
    {
      "count": 1437,
      "name": "www321",
      "node_type": "Namespace"
    },
    {
      "count": 1430,
      "name": "www424",
      "node_type": "Namespace"
    },
    {
      "count": 1429,
      "name": "www307",
      "node_type": "Namespace"
    },
    {
      "count": 1425,
      "name": "standardebooks",
      "node_type": "Namespace"
    },
    {
      "count": 1425,
      "name": "www78",
      "node_type": "Namespace"
    },
    {
      "count": 1417,
      "name": "beachsoccerrussia",
      "node_type": "Namespace"
    },
    {
      "count": 1416,
      "name": "www386",
      "node_type": "Namespace"
    },
    {
      "count": 1411,
      "name": "www323",
      "node_type": "Namespace"
    },
    {
      "count": 1409,
      "name": "www183",
      "node_type": "Namespace"
    },
    {
      "count": 1409,
      "name": "www338",
      "node_type": "Namespace"
    },
    {
      "count": 1406,
      "name": "fmg",
      "node_type": "Namespace"
    },
    {
      "count": 1403,
      "name": "www263",
      "node_type": "Namespace"
    },
    {
      "count": 1400,
      "name": "ada1bank",
      "node_type": "Namespace"
    },
    {
      "count": 1399,
      "name": "www337",
      "node_type": "Namespace"
    },
    {
      "count": 1398,
      "name": "www421",
      "node_type": "Namespace"
    },
    {
      "count": 1396,
      "name": "www344",
      "node_type": "Namespace"
    },
    {
      "count": 1396,
      "name": "www454",
      "node_type": "Namespace"
    },
    {
      "count": 1395,
      "name": "www361",
      "node_type": "Namespace"
    },
    {
      "count": 1382,
      "name": "www318",
      "node_type": "Namespace"
    },
    {
      "count": 1379,
      "name": "www419",
      "node_type": "Namespace"
    },
    {
      "count": 1378,
      "name": "fi",
      "node_type": "Namespace"
    },
    {
      "count": 1374,
      "name": "www469",
      "node_type": "Namespace"
    },
    {
      "count": 1371,
      "name": "www92",
      "node_type": "Namespace"
    },
    {
      "count": 1365,
      "name": "www336",
      "node_type": "Namespace"
    },
    {
      "count": 1364,
      "name": "content-uk",
      "node_type": "Namespace"
    },
    {
      "count": 1363,
      "name": "www320",
      "node_type": "Namespace"
    },
    {
      "count": 1362,
      "name": "www420",
      "node_type": "Namespace"
    },
    {
      "count": 1356,
      "name": "www308",
      "node_type": "Namespace"
    },
    {
      "count": 1355,
      "name": "thomas",
      "node_type": "Namespace"
    },
    {
      "count": 1352,
      "name": "www403",
      "node_type": "Namespace"
    },
    {
      "count": 1350,
      "name": "www325",
      "node_type": "Namespace"
    },
    {
      "count": 1347,
      "name": "dx2",
      "node_type": "Namespace"
    },
    {
      "count": 1345,
      "name": "beachsoccer",
      "node_type": "Namespace"
    },
    {
      "count": 1345,
      "name": "www367",
      "node_type": "Namespace"
    },
    {
      "count": 1344,
      "name": "uk2",
      "node_type": "Namespace"
    },
    {
      "count": 1342,
      "name": "football",
      "node_type": "Namespace"
    },
    {
      "count": 1341,
      "name": "www346",
      "node_type": "Namespace"
    },
    {
      "count": 1341,
      "name": "www412",
      "node_type": "Namespace"
    },
    {
      "count": 1339,
      "name": "pba2",
      "node_type": "Namespace"
    },
    {
      "count": 1339,
      "name": "soccerstats",
      "node_type": "Namespace"
    },
    {
      "count": 1338,
      "name": "ehfcl",
      "node_type": "Namespace"
    },
    {
      "count": 1338,
      "name": "en2",
      "node_type": "Namespace"
    },
    {
      "count": 1337,
      "name": "www365",
      "node_type": "Namespace"
    },
    {
      "count": 1334,
      "name": "europeancup",
      "node_type": "Namespace"
    },
    {
      "count": 1333,
      "name": "www345",
      "node_type": "Namespace"
    },
    {
      "count": 1331,
      "name": "nces",
      "node_type": "Namespace"
    },
    {
      "count": 1328,
      "name": "www309",
      "node_type": "Namespace"
    },
    {
      "count": 1321,
      "name": "wikidata",
      "node_type": "Namespace"
    },
    {
      "count": 1320,
      "name": "www414",
      "node_type": "Namespace"
    },
    {
      "count": 1316,
      "name": "www462",
      "node_type": "Namespace"
    },
    {
      "count": 1313,
      "name": "www415",
      "node_type": "Namespace"
    },
    {
      "count": 1311,
      "name": "en",
      "node_type": "Namespace"
    },
    {
      "count": 1308,
      "name": "baske2",
      "node_type": "Namespace"
    },
    {
      "count": 1307,
      "name": "openjurist",
      "node_type": "Namespace"
    },
    {
      "count": 1305,
      "name": "prvahnl",
      "node_type": "Namespace"
    },
    {
      "count": 1302,
      "name": "www382",
      "node_type": "Namespace"
    },
    {
      "count": 1301,
      "name": "www330",
      "node_type": "Namespace"
    },
    {
      "count": 1298,
      "name": "www399",
      "node_type": "Namespace"
    },
    {
      "count": 1298,
      "name": "www453",
      "node_type": "Namespace"
    },
    {
      "count": 1295,
      "name": "www396",
      "node_type": "Namespace"
    },
    {
      "count": 1292,
      "name": "wayback",
      "node_type": "Namespace"
    },
    {
      "count": 1289,
      "name": "ameblo",
      "node_type": "Namespace"
    },
    {
      "count": 1280,
      "name": "nbn-resolving",
      "node_type": "Namespace"
    },
    {
      "count": 1277,
      "name": "www383",
      "node_type": "Namespace"
    },
    {
      "count": 1277,
      "name": "www447",
      "node_type": "Namespace"
    },
    {
      "count": 1277,
      "name": "www468",
      "node_type": "Namespace"
    },
    {
      "count": 1273,
      "name": "www316",
      "node_type": "Namespace"
    },
    {
      "count": 1267,
      "name": "www410",
      "node_type": "Namespace"
    },
    {
      "count": 1265,
      "name": "journcom",
      "node_type": "Namespace"
    },
    {
      "count": 1264,
      "name": "playorg",
      "node_type": "Namespace"
    },
    {
      "count": 1263,
      "name": "universiade2013",
      "node_type": "Namespace"
    },
    {
      "count": 1263,
      "name": "www315",
      "node_type": "Namespace"
    },
    {
      "count": 1260,
      "name": "www333",
      "node_type": "Namespace"
    },
    {
      "count": 1260,
      "name": "www358",
      "node_type": "Namespace"
    },
    {
      "count": 1259,
      "name": "livestats",
      "node_type": "Namespace"
    },
    {
      "count": 1258,
      "name": "2009-2017",
      "node_type": "Namespace"
    },
    {
      "count": 1256,
      "name": "competiciones",
      "node_type": "Namespace"
    },
    {
      "count": 1255,
      "name": "ar",
      "node_type": "Namespace"
    },
    {
      "count": 1255,
      "name": "www366",
      "node_type": "Namespace"
    },
    {
      "count": 1242,
      "name": "users",
      "node_type": "Namespace"
    },
    {
      "count": 1241,
      "name": "www236",
      "node_type": "Namespace"
    },
    {
      "count": 1239,
      "name": "www411",
      "node_type": "Namespace"
    },
    {
      "count": 1237,
      "name": "pqasb",
      "node_type": "Namespace"
    },
    {
      "count": 1233,
      "name": "www404",
      "node_type": "Namespace"
    },
    {
      "count": 1232,
      "name": "spfl",
      "node_type": "Namespace"
    },
    {
      "count": 1229,
      "name": "www409",
      "node_type": "Namespace"
    },
    {
      "count": 1227,
      "name": "cdngov",
      "node_type": "Namespace"
    },
    {
      "count": 1227,
      "name": "www398",
      "node_type": "Namespace"
    },
    {
      "count": 1225,
      "name": "www291",
      "node_type": "Namespace"
    },
    {
      "count": 1224,
      "name": "statiorg",
      "node_type": "Namespace"
    },
    {
      "count": 1224,
      "name": "womenscompetitions",
      "node_type": "Namespace"
    },
    {
      "count": 1223,
      "name": "www310",
      "node_type": "Namespace"
    },
    {
      "count": 1222,
      "name": "sites2",
      "node_type": "Namespace"
    },
    {
      "count": 1221,
      "name": "www341",
      "node_type": "Namespace"
    },
    {
      "count": 1218,
      "name": "www281",
      "node_type": "Namespace"
    },
    {
      "count": 1217,
      "name": "www342",
      "node_type": "Namespace"
    },
    {
      "count": 1214,
      "name": "scenaripolitici",
      "node_type": "Namespace"
    },
    {
      "count": 1210,
      "name": "bioguide",
      "node_type": "Namespace"
    },
    {
      "count": 1209,
      "name": "ssrn",
      "node_type": "Namespace"
    },
    {
      "count": 1208,
      "name": "prvah2",
      "node_type": "Namespace"
    },
    {
      "count": 1207,
      "name": "www407",
      "node_type": "Namespace"
    },
    {
      "count": 1206,
      "name": "no",
      "node_type": "Namespace"
    },
    {
      "count": 1200,
      "name": "brie",
      "node_type": "Namespace"
    },
    {
      "count": 1198,
      "name": "www312",
      "node_type": "Namespace"
    },
    {
      "count": 1198,
      "name": "www406",
      "node_type": "Namespace"
    },
    {
      "count": 1196,
      "name": "www317",
      "node_type": "Namespace"
    },
    {
      "count": 1196,
      "name": "www354",
      "node_type": "Namespace"
    },
    {
      "count": 1194,
      "name": "knyga",
      "node_type": "Namespace"
    },
    {
      "count": 1190,
      "name": "www2com",
      "node_type": "Namespace"
    },
    {
      "count": 1190,
      "name": "www335",
      "node_type": "Namespace"
    },
    {
      "count": 1189,
      "name": "plato",
      "node_type": "Namespace"
    },
    {
      "count": 1188,
      "name": "pba",
      "node_type": "Namespace"
    },
    {
      "count": 1188,
      "name": "www381",
      "node_type": "Namespace"
    },
    {
      "count": 1187,
      "name": "pflk",
      "node_type": "Namespace"
    },
    {
      "count": 1187,
      "name": "www359",
      "node_type": "Namespace"
    },
    {
      "count": 1184,
      "name": "www351",
      "node_type": "Namespace"
    },
    {
      "count": 1173,
      "name": "www405",
      "node_type": "Namespace"
    },
    {
      "count": 1172,
      "name": "www444",
      "node_type": "Namespace"
    },
    {
      "count": 1169,
      "name": "www353",
      "node_type": "Namespace"
    },
    {
      "count": 1169,
      "name": "www363",
      "node_type": "Namespace"
    },
    {
      "count": 1166,
      "name": "www357",
      "node_type": "Namespace"
    },
    {
      "count": 1161,
      "name": "content",
      "node_type": "Namespace"
    },
    {
      "count": 1158,
      "name": "www352",
      "node_type": "Namespace"
    },
    {
      "count": 1156,
      "name": "cacom",
      "node_type": "Namespace"
    },
    {
      "count": 1155,
      "name": "www402",
      "node_type": "Namespace"
    },
    {
      "count": 1153,
      "name": "www380",
      "node_type": "Namespace"
    },
    {
      "count": 1152,
      "name": "www459",
      "node_type": "Namespace"
    },
    {
      "count": 1149,
      "name": "bugguide",
      "node_type": "Namespace"
    },
    {
      "count": 1148,
      "name": "www350",
      "node_type": "Namespace"
    },
    {
      "count": 1147,
      "name": "bwf2",
      "node_type": "Namespace"
    },
    {
      "count": 1146,
      "name": "www451",
      "node_type": "Namespace"
    },
    {
      "count": 1144,
      "name": "www401",
      "node_type": "Namespace"
    },
    {
      "count": 1142,
      "name": "licensing",
      "node_type": "Namespace"
    },
    {
      "count": 1140,
      "name": "www465",
      "node_type": "Namespace"
    },
    {
      "count": 1139,
      "name": "enorg",
      "node_type": "Namespace"
    },
    {
      "count": 1137,
      "name": "wrsd",
      "node_type": "Namespace"
    },
    {
      "count": 1136,
      "name": "www379",
      "node_type": "Namespace"
    },
    {
      "count": 1134,
      "name": "www400",
      "node_type": "Namespace"
    },
    {
      "count": 1128,
      "name": "paleodb",
      "node_type": "Namespace"
    },
    {
      "count": 1128,
      "name": "www275",
      "node_type": "Namespace"
    },
    {
      "count": 1127,
      "name": "www360",
      "node_type": "Namespace"
    },
    {
      "count": 1126,
      "name": "www397",
      "node_type": "Namespace"
    },
    {
      "count": 1125,
      "name": "he",
      "node_type": "Namespace"
    },
    {
      "count": 1125,
      "name": "keepup",
      "node_type": "Namespace"
    },
    {
      "count": 1124,
      "name": "bsrussia",
      "node_type": "Namespace"
    },
    {
      "count": 1120,
      "name": "semanticscholar",
      "node_type": "Namespace"
    },
    {
      "count": 1118,
      "name": "www375",
      "node_type": "Namespace"
    },
    {
      "count": 1117,
      "name": "data10",
      "node_type": "Namespace"
    },
    {
      "count": 1116,
      "name": "eci2",
      "node_type": "Namespace"
    },
    {
      "count": 1113,
      "name": "www362",
      "node_type": "Namespace"
    },
    {
      "count": 1104,
      "name": "cms",
      "node_type": "Namespace"
    },
    {
      "count": 1104,
      "name": "www347",
      "node_type": "Namespace"
    },
    {
      "count": 1103,
      "name": "www370",
      "node_type": "Namespace"
    },
    {
      "count": 1102,
      "name": "www458",
      "node_type": "Namespace"
    },
    {
      "count": 1100,
      "name": "www395",
      "node_type": "Namespace"
    },
    {
      "count": 1099,
      "name": "www377",
      "node_type": "Namespace"
    },
    {
      "count": 1098,
      "name": "egrove",
      "node_type": "Namespace"
    },
    {
      "count": 1097,
      "name": "websites",
      "node_type": "Namespace"
    },
    {
      "count": 1093,
      "name": "www289",
      "node_type": "Namespace"
    },
    {
      "count": 1093,
      "name": "www348",
      "node_type": "Namespace"
    },
    {
      "count": 1093,
      "name": "www373",
      "node_type": "Namespace"
    },
    {
      "count": 1088,
      "name": "www394",
      "node_type": "Namespace"
    },
    {
      "count": 1084,
      "name": "www464",
      "node_type": "Namespace"
    },
    {
      "count": 1082,
      "name": "www234",
      "node_type": "Namespace"
    },
    {
      "count": 1081,
      "name": "www385",
      "node_type": "Namespace"
    },
    {
      "count": 1078,
      "name": "arz",
      "node_type": "Namespace"
    },
    {
      "count": 1078,
      "name": "generals",
      "node_type": "Namespace"
    },
    {
      "count": 1076,
      "name": "bacdi2",
      "node_type": "Namespace"
    },
    {
      "count": 1075,
      "name": "www467",
      "node_type": "Namespace"
    },
    {
      "count": 1074,
      "name": "www391",
      "node_type": "Namespace"
    },
    {
      "count": 1072,
      "name": "v7player",
      "node_type": "Namespace"
    },
    {
      "count": 1071,
      "name": "eur-lex",
      "node_type": "Namespace"
    },
    {
      "count": 1071,
      "name": "www457",
      "node_type": "Namespace"
    },
    {
      "count": 1069,
      "name": "ethos",
      "node_type": "Namespace"
    },
    {
      "count": 1063,
      "name": "archi4",
      "node_type": "Namespace"
    },
    {
      "count": 1061,
      "name": "www311",
      "node_type": "Namespace"
    },
    {
      "count": 1061,
      "name": "www460",
      "node_type": "Namespace"
    },
    {
      "count": 1056,
      "name": "www374",
      "node_type": "Namespace"
    },
    {
      "count": 1053,
      "name": "www387",
      "node_type": "Namespace"
    },
    {
      "count": 1052,
      "name": "www355",
      "node_type": "Namespace"
    },
    {
      "count": 1050,
      "name": "byucougars",
      "node_type": "Namespace"
    },
    {
      "count": 1050,
      "name": "www392",
      "node_type": "Namespace"
    },
    {
      "count": 1049,
      "name": "www390",
      "node_type": "Namespace"
    },
    {
      "count": 1049,
      "name": "www393",
      "node_type": "Namespace"
    },
    {
      "count": 1047,
      "name": "le",
      "node_type": "Namespace"
    },
    {
      "count": 1046,
      "name": "www450",
      "node_type": "Namespace"
    },
    {
      "count": 1045,
      "name": "data8",
      "node_type": "Namespace"
    },
    {
      "count": 1045,
      "name": "jewishencyclopedia",
      "node_type": "Namespace"
    },
    {
      "count": 1044,
      "name": "www446",
      "node_type": "Namespace"
    },
    {
      "count": 1041,
      "name": "aic",
      "node_type": "Namespace"
    },
    {
      "count": 1040,
      "name": "www376",
      "node_type": "Namespace"
    },
    {
      "count": 1038,
      "name": "www267",
      "node_type": "Namespace"
    },
    {
      "count": 1038,
      "name": "www389",
      "node_type": "Namespace"
    },
    {
      "count": 1038,
      "name": "www449",
      "node_type": "Namespace"
    },
    {
      "count": 1036,
      "name": "worldcat",
      "node_type": "Namespace"
    },
    {
      "count": 1036,
      "name": "www443",
      "node_type": "Namespace"
    },
    {
      "count": 1030,
      "name": "ballo2",
      "node_type": "Namespace"
    },
    {
      "count": 1026,
      "name": "www442",
      "node_type": "Namespace"
    },
    {
      "count": 1025,
      "name": "ncbibook",
      "node_type": "Namespace"
    },
    {
      "count": 1021,
      "name": "www388",
      "node_type": "Namespace"
    },
    {
      "count": 1020,
      "name": "www349",
      "node_type": "Namespace"
    },
    {
      "count": 1019,
      "name": "www441",
      "node_type": "Namespace"
    },
    {
      "count": 1017,
      "name": "progenetix",
      "node_type": "Namespace"
    },
    {
      "count": 1015,
      "name": "globo2",
      "node_type": "Namespace"
    },
    {
      "count": 1008,
      "name": "www372",
      "node_type": "Namespace"
    },
    {
      "count": 1004,
      "name": "texassports",
      "node_type": "Namespace"
    },
    {
      "count": 1002,
      "name": "instagram",
      "node_type": "Namespace"
    },
    {
      "count": 1002,
      "name": "www463",
      "node_type": "Namespace"
    },
    {
      "count": 1000,
      "name": "bisinfo",
      "node_type": "Namespace"
    },
    {
      "count": 1000,
      "name": "www440",
      "node_type": "Namespace"
    },
    {
      "count": 938,
      "name": "csdbp",
      "node_type": "Namespace"
    },
    {
      "count": 827,
      "name": "weki",
      "node_type": "Namespace"
    },
    {
      "count": 620,
      "name": "eolife",
      "node_type": "Namespace"
    },
    {
      "count": 516,
      "name": "bgdbr",
      "node_type": "Namespace"
    },
    {
      "count": 465,
      "name": "gbif",
      "node_type": "Namespace"
    },
    {
      "count": 403,
      "name": "motogp",
      "node_type": "Namespace"
    },
    {
      "count": 274,
      "name": "inaturalist.taxon",
      "node_type": "Namespace"
    },
    {
      "count": 226,
      "name": "osmway",
      "node_type": "Namespace"
    },
    {
      "count": 214,
      "name": "wbc",
      "node_type": "Namespace"
    },
    {
      "count": 99,
      "name": "meat",
      "node_type": "Namespace"
    },
    {
      "count": 84,
      "name": "ple",
      "node_type": "Namespace"
    },
    {
      "count": 74,
      "name": "osmnode",
      "node_type": "Namespace"
    },
    {
      "count": 62,
      "name": "ex",
      "node_type": "Namespace"
    },
    {
      "count": 61,
      "name": "inaturalist.place",
      "node_type": "Namespace"
    },
    {
      "count": 56,
      "name": "wwf.ecoregion",
      "node_type": "Namespace"
    },
    {
      "count": 41,
      "name": "cordis.project",
      "node_type": "Namespace"
    },
    {
      "count": 40,
      "name": "snac",
      "node_type": "Namespace"
    },
    {
      "count": 39,
      "name": "ncbi.genome",
      "node_type": "Namespace"
    },
    {
      "count": 39,
      "name": "publons.researcher",
      "node_type": "Namespace"
    },
    {
      "count": 27,
      "name": "bsb",
      "node_type": "Namespace"
    },
    {
      "count": 26,
      "name": "geonames",
      "node_type": "Namespace"
    },
    {
      "count": 24,
      "name": "inaturalist.observation",
      "node_type": "Namespace"
    },
    {
      "count": 23,
      "name": "sgv",
      "node_type": "Namespace"
    },
    {
      "count": 22,
      "name": "biorxiv",
      "node_type": "Namespace"
    },
    {
      "count": 20,
      "name": "dc",
      "node_type": "Namespace"
    },
    {
      "count": 20,
      "name": "vz",
      "node_type": "Namespace"
    },
    {
      "count": 17,
      "name": "purl",
      "node_type": "Namespace"
    },
    {
      "count": 16,
      "name": "noaa",
      "node_type": "Namespace"
    },
    {
      "count": 14,
      "name": "eppo",
      "node_type": "Namespace"
    },
    {
      "count": 13,
      "name": "eg",
      "node_type": "Namespace"
    },
    {
      "count": 13,
      "name": "ghr",
      "node_type": "Namespace"
    },
    {
      "count": 13,
      "name": "mime",
      "node_type": "Namespace"
    },
    {
      "count": 13,
      "name": "nlm",
      "node_type": "Namespace"
    },
    {
      "count": 10,
      "name": "genbank",
      "node_type": "Namespace"
    }
  ]
});
