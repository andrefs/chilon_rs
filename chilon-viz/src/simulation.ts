import { Simulation, forceLink, forceManyBody, forceCollide, forceCenter } from 'd3-force';
import { RawEdge, RawNode, SimData } from './data/raw-data';
import { selectAll, select } from 'd3-selection';


export const initSimulation = (sim: Simulation<RawNode, RawEdge>, data: SimData, width: number, height: number) => {
  if (!sim) { return; }

  sim.nodes(data.nodes)
    .force("linkForce", forceLink(data.edges).distance(300).strength(2))
    .force("charge", forceManyBody().strength(-800).distanceMin(200).distanceMax(400))
    .force('collision', forceCollide().radius((d) => (d as RawNode).count + 4))
    .force('center', forceCenter(width / 2, height / 2))
    .on("tick", ticked(sim));
}


export const restartSimulation = (sim: Simulation<RawNode, RawEdge>, data: SimData) => {
  if (!sim) { return; }
  sim.nodes(data.nodes)
    .force("linkForce", forceLink(data.edges).distance(300).strength(2))
    .alpha(1).restart()
    .on('tick', ticked(sim));
}


const ticked = (sim: Simulation<RawNode, RawEdge>) => () => {
  let edgepaths = selectAll("svg g.edge-g path");
  edgepaths.attr('d', function(d: any) {
    const sId = d.source?.id ?? d.source;
    const tId = d.target?.id ?? d.target;
    return sId === tId ? calcLoop(d) : calcEdge(d);
  });

  let nodeGroups = selectAll("svg g.node-g");

  let nodes = nodeGroups.selectAll("circle");
  nodes.attr("cx", (d: any) => d.x)
    .attr("cy", (d: any) => d.y)

  let nodelabels = nodeGroups.selectAll("text");
  nodelabels.attr("x", (d: any) => d.x)
    .attr("y", (d: any) => d.y);

  select('#alpha_value').style('flex-basis', (sim.alpha() * 100) + '%');
}

const calcEdge = (d: any) => {
  let signal = 0;
  if (d.linknum % 2 === 1 && d.linknum > 0) { signal = 1; }
  if (d.linknum % 2 === 0 && d.linknum < 0) { signal = 1; }
  const dl = Math.abs(d.linknum)
  const divisor = Math.floor(dl / 2) * 2;
  const dr = dl === 1 ? 0 : 1500 / divisor;  //linknum is defined above

  const pathd = `M${d.source.x},${d.source.y}
                 A${dr},${dr} 0 0 ${signal} ${d.target.x},${d.target.y}`;
  return pathd;
};

const calcLoop = (d: any) => {
  const dl = Math.abs(d.linknum)
  const dr = 40 + d.normCount * dl;  //linknum is defined above

  //loop
  //d="M334.5179247605647,472.7245628100564
  //     A73,73 -45 1 1 335.5179247605647,473.7245628100564"
  const pathd = `M${d.source.x},${d.source.y}
                   A${dr},${dr} -45 1 0 ${d.target.x + 1},${d.target.y + 1}`;
  return pathd;

}
