import { RawEdge, RawNode, SimData } from "../data/raw-data";
import rangeSlider from 'range-slider-input';
import 'range-slider-input/dist/style.css';
import { filterData, truncateData } from "../data";
import { update } from "../simulation";
import { Simulation } from "d3-force";
import { Selection } from "d3-selection";
import { scaleLog } from "d3-scale";

export const getSliderElems = () => {
  return {
    nodeOccursIn: document.querySelector<HTMLInputElement>('#nodeOccursInput')!,
    edgeOccursIn: document.querySelector<HTMLInputElement>('#edgeOccursInput')!,

    minNodeOccursOut: document.querySelector<HTMLOutputElement>('#minNodeOccurs')!,
    maxNodeOccursOut: document.querySelector<HTMLOutputElement>('#maxNodeOccurs')!,
    minEdgeOccursOut: document.querySelector<HTMLOutputElement>('#minEdgeOccurs')!,
    maxEdgeOccursOut: document.querySelector<HTMLOutputElement>('#maxEdgeOccurs')!,
  }
}

export type SliderValues = {
  minNodeOccurs: number,
  maxNodeOccurs: number,
  minEdgeOccurs: number,
  maxEdgeOccurs: number,
};

export const getSliderValues = () => {
  const elems = getSliderElems();

  return {
    nodeOccurs: Number(elems.nodeOccursIn.value),
    edgeOccurs: Number(elems.edgeOccursIn.value),
  };
}


const debounce = (func: Function, timeout = 100) => {
  let timer: number;
  return (...args: any[]) => {
    clearTimeout(timer);
    timer = setTimeout(() => {
      func.apply(this, args);

    }, timeout);
  }
}



export const initSliders = (
  initData: SimData,
  sim: Simulation<RawNode, RawEdge>,
  svg: Selection<SVGSVGElement, any, HTMLElement, any>,
  tooltip: Selection<HTMLDivElement, unknown, HTMLElement, any>
) => {
  let elems = getSliderElems();

  const data = truncateData(initData, 50, 50);

  let minNodeOccurs = initData.nodes.slice(-1)[0].count;
  let maxNodeOccurs = initData.nodes[0].count;
  let minEdgeOccurs = initData.edges.slice(-1)[0].count;
  let maxEdgeOccurs = initData.edges[0].count;

  const scaleNode = scaleLog()
    .domain([minNodeOccurs, maxNodeOccurs])
    .range([0, 100]);
  const scaleEdge = scaleLog()
    .domain([minEdgeOccurs, maxEdgeOccurs])
    .range([0, 100]);


  elems.minNodeOccursOut.value = minNodeOccurs.toString();
  elems.maxNodeOccursOut.value = maxNodeOccurs.toString();
  elems.minEdgeOccursOut.value = minEdgeOccurs.toString();
  elems.maxEdgeOccursOut.value = maxEdgeOccurs.toString();


  let nodeSlider = rangeSlider(elems.nodeOccursIn, {
    min: 0,
    max: 100,
    value: [scaleNode(data.nodes.slice(-1)[0].count), scaleNode(data.nodes[0].count)],
    onInput: debounce(([_minNO, _maxNO]: [number, number]) => {
      const minNO = scaleNode.invert(_minNO);
      const maxNO = scaleNode.invert(_maxNO);
      minNodeOccurs = minNO;
      maxNodeOccurs = maxNO;
      elems.minNodeOccursOut.value = Math.floor(minNO).toString();
      elems.maxNodeOccursOut.value = Math.floor(maxNO).toString();

      const newData = filterData(initData, {
        minNodeOccurs,
        maxNodeOccurs,
        minEdgeOccurs,
        maxEdgeOccurs
      });

      update(newData, sim, svg, tooltip);

    }, 100)
  });

  let edgeSlider = rangeSlider(elems.edgeOccursIn, {
    min: 0,
    max: 100,
    value: [scaleEdge(data.edges.slice(-1)[0].count), scaleEdge(data.edges[0].count)],
    onInput: debounce(([_minEO, _maxEO]: [number, number]) => {
      const minEO = scaleEdge.invert(_minEO);
      const maxEO = scaleEdge.invert(_maxEO);
      elems.minEdgeOccursOut.value = Math.floor(minEO).toString();
      elems.maxEdgeOccursOut.value = Math.floor(maxEO).toString().toString();

      minEdgeOccurs = minEO;
      maxEdgeOccurs = maxEO;

      const newData = filterData(initData, {
        minNodeOccurs,
        maxNodeOccurs,
        minEdgeOccurs,
        maxEdgeOccurs
      });

      update(newData, sim, svg, tooltip);
    }, 100)
  });

  return { nodeSlider, edgeSlider };
}



