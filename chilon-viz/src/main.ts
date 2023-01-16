import './style.css'
import { initData, truncateData } from './data';
//import { select } from 'd3-selection';
import { initSliderValues } from './events/sliders';
//import { forceSimulation } from 'd3-force';
//import { RawNode } from './data/raw-data';
//import * as d3Zoom from 'd3-zoom';




const start = () => {
  const data = truncateData(initData, 5, 5);
  initSliderValues(data);

  //const svg = select<SVGSVGElement, any>("#app svg");
  //svg.append('g').attr('class', 'edges')
  //svg.append('g').attr('class', 'nodes');

  //const sim = forceSimulation<RawNode>();
  //let zoom = d3Zoom.zoom().scaleExtent([0.1, 5]).on('zoom', handleZoom);
  //const width = svg.node()?.getBoundingClientRect().width;
  //const height = svg.node()?.getBoundingClientRect().height;






  //console.log('XXXXXX', { sim, initData, svg });
  console.log('XXXXXX', { initData, data });
}

start();


