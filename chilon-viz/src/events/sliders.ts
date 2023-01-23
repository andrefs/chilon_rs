import { RawEdge, RawNode, SimData } from "../data/raw-data";
import rangeSlider from 'range-slider-input';
import 'range-slider-input/dist/style.css';
import { filterData, truncateData } from "../data";
import { update } from "../simulation";
import { Simulation } from "d3-force";
import { Selection } from "d3-selection";

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

  let minNodeOccurs = 0;
  let maxNodeOccurs = initData.nodes[0].count;
  let minEdgeOccurs = 0;
  let maxEdgeOccurs = initData.edges[0].count;


  let nodeSlider = rangeSlider(elems.nodeOccursIn, {
    min: minNodeOccurs,
    max: maxNodeOccurs,
    value: [data.nodes.slice(-1)[0].count, data.nodes[0].count],
    onInput: debounce(([minNO, maxNO]: [number, number]) => {
      minNodeOccurs = minNO;
      maxNodeOccurs = maxNO;
      elems.minNodeOccursOut.value = minNO.toString();
      elems.maxNodeOccursOut.value = maxNO.toString();

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
    min: minEdgeOccurs,
    max: maxEdgeOccurs,
    value: [data.edges.slice(-1)[0].count, data.edges[0].count],
    onInput: debounce(([minEO, maxEO]: [number, number]) => {
      elems.minEdgeOccursOut.value = minEO.toString();
      elems.maxEdgeOccursOut.value = maxEO.toString();

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



