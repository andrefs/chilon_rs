import './style.css'
import { initData, truncateData } from './data';
import { select, selectAll } from 'd3-selection';
import { forceSimulation } from 'd3-force';
import { RawNode } from './data/raw-data';
import { initSimulation, update } from './simulation';
import * as d3Zoom from 'd3-zoom';
import { handleZoom, initZoom } from './zoom';
import { createTooltip } from './tooltip';
import { initConfig } from './events/config';


declare global {
  interface Window { ChilonViz: any; }
}

window.ChilonViz = window.ChilonViz || {};

const start = () => {
  const data = truncateData(initData, 50, 50);

  const svg = select<SVGSVGElement, any>("#app svg");
  svg.append('g').attr('class', 'edges')
  svg.append('g').attr('class', 'nodes');

  const allG = selectAll('#app svg g');
  const sim = forceSimulation<RawNode>();
  let zoom = d3Zoom.zoom().scaleExtent([0.1, 5]).on('zoom', handleZoom(allG));

  const width = svg.node()?.getBoundingClientRect().width!;
  const height = svg.node()?.getBoundingClientRect().height!;

  const tooltip = createTooltip();

  initSimulation(sim, data, width, height);
  initConfig(initData, sim, svg, tooltip);


  update(data, sim, svg, tooltip);
  initZoom(zoom);


  //addMouseEventListeners(svg, tooltip);

  console.log('XXXXXX end', { sim, initData, data, svg });
}

start();


