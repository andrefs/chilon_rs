import './style.css'
import { rawData } from './data';
import { select } from 'd3-selection';
import { initSliderListeners, initSliderValues } from './events/sliders';
import { forceSimulation } from 'd3-force';
import { RawNode } from './data/raw-data';


initSliderListeners();
initSliderValues(rawData);

let svg = select("#app svg");
svg.append('g').attr('class', 'edges')
svg.append('g').attr('class', 'nodes');

const sim = forceSimulation<RawNode>();




console.log('XXXXXX', { sim, rawData, svg });
