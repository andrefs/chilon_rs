import './style.css'
import { rawData } from './data';
import { select } from 'd3-selection';

let svg = select("#app svg");
svg.append('g').attr('class', 'edges')
svg.append('g').attr('class', 'nodes');





console.log('XXXXXX', { rawData, svg });
