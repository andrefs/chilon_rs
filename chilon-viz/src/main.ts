import './style.css'
import { initData, truncateData } from './data';
import { BaseType, select, selectAll, Selection } from 'd3-selection';
import { initSliders } from './events/sliders';
import { forceSimulation, Simulation } from 'd3-force';
import { RawEdge, RawNode, SimData } from './data/raw-data';
import { initSimulation, restartSimulation } from './simulation';
import * as d3Zoom from 'd3-zoom';


const update = (data: SimData, sim: Simulation<RawNode, RawEdge>) => {

  let nodesParent = select("svg g.nodes");
  let edgesParent = select("svg g.edges");

  const nodeGroups = nodesParent
    .selectAll("g")
    .data(data.nodes)
    .enter()
    .append('g').attr('class', 'node-g');
  //nodeGroups.exit().remove();

  let nodes = nodeGroups
    .append("circle")
    .attr("r", d => Math.ceil(d.normCount || 0))
    .style("fill", () => '#B3D9CB')
    .attr('data-id', (d) => d.id)
    .attr("data-fill", () => '#B3D9CB')
    .style("pointer-events", "visiblePainted")
    .style('cursor', 'pointer')

  var nodelabels = nodeGroups
    .append("text")
    .attr("x", (d: any) => d.x)
    .attr("y", (d: any) => d.y)
    .attr('font-size', d => Math.ceil(d.normCount / 2))
    .attr('class', "nodelabel")
    .text((d: any) => d.name)
    .attr('dominant-baseline', 'middle')
    .style("pointer-events", "none")
    .style('cursor', 'pointer')



  const edgeGroups = edgesParent
    .selectAll("g")
    .data(data.edges).enter()
    .append('g').attr('class', 'edge-g')
  //.exit().remove();

  var edgepaths = edgeGroups
    .append('path')
    //.attr('d',linkArc)
    //.attr('d', d => {
    //  console.log('XXXXXXXXXXXX d', d);
    //  return 'M '+d.source.x+' '+d.source.y+' L '+ d.target.x +' '+d.target.y
    //})
    .attr('class', 'edgepath')
    .attr('fill-opacity', 0)
    .attr('id', (_, i) => 'edgepath' + i)
    .attr("stroke-width", (d: any) => Math.ceil(d.normCount / 10))
    .attr('opacity', 0.3)
    .attr("data-stroke-width", (d: any) => Math.ceil(d.normCount / 10))
    .attr("data-source", (d: any) => d.source)
    .attr("data-target", (d: any) => d.target)
    .attr("data-label", (d: any) => d.label)
    .style("stroke", (d: any) => d.colorHash)
    //.style("stroke", '#b8b8b8')
    .attr("data-stroke", (d: any) => d.colorHash)
    //.style("pointer-events", "none");
    .style("pointer-events", "visibleStroke")
    .attr('marker-end', 'url(#triangle)')

  var edgelabels = edgeGroups.append('text')
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

  restartSimulation(sim, data);
}

const handleZoom = (g: Selection<BaseType, unknown, HTMLElement, any>) => (e: any) => {
  g.attr('transform', e.transform);
}
const initZoom = (zoom: any) => {
  select('svg')
    .call(zoom);
}


const start = () => {
  const data = truncateData(initData, 50, 50);
  let { nodeSlider, edgeSlider } = initSliders(data);

  const svg = select<SVGSVGElement, any>("#app svg");
  svg.append('g').attr('class', 'edges')
  svg.append('g').attr('class', 'nodes');

  const allG = selectAll('#app svg g');
  const sim = forceSimulation<RawNode>();
  let zoom = d3Zoom.zoom().scaleExtent([0.1, 5]).on('zoom', handleZoom(allG));

  const width = svg.node()?.getBoundingClientRect().width!;
  const height = svg.node()?.getBoundingClientRect().height!;

  initSimulation(sim, data, width, height);


  update(data, sim);



  initZoom(zoom);


  console.log('XXXXXX', { sim, initData, data, svg });
}

start();


