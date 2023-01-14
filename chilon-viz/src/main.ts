import './style.css'
import { rawData } from './data';
import { select } from 'd3-selection';
import { initSliderListeners, initSliderValues } from './events/sliders';


initSliderListeners();
initSliderValues(rawData);

let svg = select("#app svg");
svg.append('g').attr('class', 'edges')
svg.append('g').attr('class', 'nodes');





console.log('XXXXXX', { rawData, svg });
