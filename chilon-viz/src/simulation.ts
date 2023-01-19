import { Simulation, forceLink, forceManyBody, forceCollide, forceCenter } from 'd3-force';
import { RawEdge, RawNode, SimData } from './data/raw-data';
import { selectAll, select } from 'd3-selection';
import { drag } from './events/drag';


export const initSimulation = (sim: Simulation<RawNode, RawEdge>, data: SimData, width: number, height: number) => {
  if (!sim) { return; }

  sim.nodes(data.nodes)
    .force("linkForce", forceLink(data.edges).id((n: any) => n.name).distance(100).strength(2))
    .force("charge", forceManyBody().strength(-200000).distanceMax(1000))
    .force('collision', forceCollide().radius((d: any) => d.normCount + 10))
    .force('center', forceCenter(width / 2, height / 2))
    .velocityDecay(0.9)
    .on("tick", ticked(sim));
}


export const restartSimulation = (sim: Simulation<RawNode, RawEdge>, data: SimData) => {
  if (!sim) { return; }
  sim.nodes(data.nodes)
    .force("linkForce", forceLink(data.edges).id((n: any) => n.name).distance(100).strength(2))
    .force("charge", forceManyBody().strength(-200000).distanceMax(1000))
    .force('collision', forceCollide().radius((d: any) => d.normCount + 4))
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
    .call(drag(sim) as any);

  let nodelabels = nodeGroups.selectAll("text");
  nodelabels.attr("x", (d: any) => d.x)
    .attr("y", (d: any) => d.y);

  select('#alpha_value').style('flex-basis', (sim.alpha() * 100) + '%');

}

const calcEdge = (d: any) => {
  let signal = 0;
  if (d.link_num % 2 === 1 && d.link_num > 0) { signal = 1; }
  if (d.link_num % 2 === 0 && d.link_num < 0) { signal = 1; }
  const dl = Math.abs(d.link_num)
  const divisor = Math.floor(dl / 2) * 2;
  const dr = dl === 1 ? 0 : 1500 / divisor;  //link_num is defined above

  const pathd = `M${d.source.x},${d.source.y}
                 A${dr},${dr} 0 0 ${signal} ${d.target.x},${d.target.y}`;
  return pathd;
};

const calcLoop = (d: any) => {
  const dl = Math.abs(d.link_num)
  const dr = 100 + d.normCount * dl;  //link_num is defined above

  //loop
  //d="M334.5179247605647,472.7245628100564
  //     A73,73 -45 1 1 335.5179247605647,473.7245628100564"
  const pathd = `M${d.source.x},${d.source.y}
                   A${dr},${dr} -45 1 0 ${d.target.x + 1},${d.target.y + 1}`;
  return pathd;

}

export const update = (data: SimData, sim: Simulation<RawNode, RawEdge>) => {

  let nodesParent = select("svg g.nodes");
  let edgesParent = select("svg g.edges");

  edgesParent
    .selectAll("g")
    .data(data.edges)
    .join(
      enter => {
        const edgeGroups = enter.append('g').attr('class', 'edge-g');

        edgeGroups
          .append('path')
          //.attr('d',linkArc)
          //.attr('d', d => {
          //  console.log('XXXXXXXXXXXX d', d);
          //  return 'M '+d.source.x+' '+d.source.y+' L '+ d.target.x +' '+d.target.y
          //})
          .attr('class', 'edgepath')
          .attr('fill-opacity', 0)
          .attr('id', (_, i) => 'edgepath' + i)
          .attr("stroke-width", (d: any) => Math.ceil(d.normCount / 3))
          .attr('opacity', 0.3)
          .attr("data-stroke-width", (d: any) => Math.ceil(d.normCount / 3))
          .attr("data-source", (d: any) => d.source)
          .attr("data-target", (d: any) => d.target)
          .attr("data-label", (d: any) => d.label)
          .style("stroke", (d: any) => d.colorHash)
          //.style("stroke", '#b8b8b8')
          .attr("data-stroke", (d: any) => d.colorHash)
          //.style("pointer-events", "none");
          .style("pointer-events", "visibleStroke")
          .attr('marker-end', 'url(#triangle)')

        const edgelabels = edgeGroups.append('text')
          .style("pointer-events", "none")
          .attr('class', 'edgelabel')
          .attr('id', (_, i) => 'edgelabel' + i)
          .attr('text-anchor', 'middle')
          .attr('dominant-baseline', 'text-after-edge')
          .attr('font-size', 15)
          .attr('fill', '#999')

        edgelabels.append('textPath')
          .attr('xlink:href', (_, i) => '#edgepath' + i)
          .style("pointer-events", "none")
          .attr('startOffset', '50%')
          .attr('text-anchor', 'middle')
          .attr('text-anchor', 'middle')
        //.text(d => d.label)

        return edgeGroups;
      },
      update => update,
      exit => exit.remove()
    );

  nodesParent
    .selectAll("g")
    .data(data.nodes)
    .join(
      enter => {
        const nodeGroups = enter.append('g').attr('class', 'node-g');

        nodeGroups
          .append("circle")
          .attr("r", d => Math.ceil(d.normCount || 0))
          .style("fill", () => '#B3D9CB')
          .attr('data-id', (d) => d.id)
          .attr("data-fill", () => '#B3D9CB')
          .style("pointer-events", "visiblePainted")
          .style('cursor', 'pointer')

        nodeGroups
          .append("text")
          .attr("x", (d: any) => d.x)
          .attr("y", (d: any) => d.y)
          .attr('font-size', d => Math.ceil(d.normCount / 2))
          .attr('class', "nodelabel")
          .text((d: any) => d.name)
          .attr('dominant-baseline', 'middle')
          .style("pointer-events", "none")
          .style('cursor', 'pointer')

        return nodeGroups;
      },
      update => update,
      exit => exit.remove()
    );


  restartSimulation(sim, data);
}
