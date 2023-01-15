
export type FNode = {
  id: number;
  name: String;
  count: number;
};

export type FEdge = {
  source: number;
  target: number;
  label: String;
  count: number;
  link_num: number;
};

export type RawData = { edges: FEdge[], nodes: FNode[] };

export const rawData: RawData = {{ data | json_encode(pretty = true) | safe }};
