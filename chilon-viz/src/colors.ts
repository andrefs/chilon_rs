import { RawEdge } from "./data/raw-data";


export const genColorHash = (edges: RawEdge[]) => {
  const predicates = new Set(edges.flatMap(l => [l.label]))

  //const nodeColors = genColors(data.nodes.length);
  const edgeColors = genColors(edges.length);
  //const colorEntries = [...Array.from(resources).map((r, i) => [r, nodeColors[i]]),
  //                      ...Array.from(predicates).map((p, i) => [p, edgeColors[i]])];
  return Object.fromEntries(Array.from(predicates).map((p, i) => ([p, edgeColors[i]]))) as { [predicate: string]: string };
}


const RGB2Color = (r: number, g: number, b: number) => '#' + byte2Hex(r) + byte2Hex(g) + byte2Hex(b);
const byte2Hex = (n: number) => {
  const nybHexString = "0123456789ABCDEF";
  return String(nybHexString.substr((n >> 4) & 0x0F, 1)) +
    nybHexString.substr(n & 0x0F, 1);
};
const makeColorGradient = (
  frequency1: number,
  frequency2: number,
  frequency3: number,
  phase1: number,
  phase2: number,
  phase3: number,
  center: number,
  width: number,
  len: number
) => {
  const colors = []
  if (len == undefined) { len = 50; }
  if (center == undefined) { center = 128; }
  if (width == undefined) { width = 127; }

  for (let i = 0; i < len; ++i) {
    const red = Math.sin(frequency1 * i + phase1) * width + center;
    const grn = Math.sin(frequency2 * i + phase2) * width + center;
    const blu = Math.sin(frequency3 * i + phase3) * width + center;
    colors.push(RGB2Color(red, grn, blu));
  }
  return colors;
};

const genColors = (numColors: number) => {
  let center = 128;
  let width = 127;
  let frequency = 2.4;
  return makeColorGradient(frequency, frequency, frequency, 0, 2, 4, center, width, numColors);
};
