// @ts-ignore
const originalData = {{ data | json_encode(pretty = true) | safe }};


class FNode {
  id: number;
  name: String;
  count: number;

  constructor(id: number, name: String, count: number) {
    this.id = id;
    this.name = name;
    this.count = count;
  }
}

class FEdge {
  source: number;
  target: number;
  label: String;
  count: number;

  constructor(source: number, target: number, label: String, count: number) {
    this.source = source;
    this.target = target;
    this.label = label;
    this.count = count;
  }
}

class MainObj {
  allNodes: FNode[];
  allEdges: FEdge[];

  constructor(nodes: FNode[], edges: FEdge[]) {
    this.allNodes = nodes;
    this.allEdges = edges;
  }
}
