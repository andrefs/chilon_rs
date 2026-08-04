import { Simulation } from "d3-force";
import { Selection } from "d3-selection";
import { RawEdge, RawNode, SimData } from "../data/raw-data";
import { CheckboxValues, getCheckboxValues, initCheckboxes } from "./checkboxes";
import { getSliderValues, initSliders, SliderValues } from "./sliders";


export type ConfigValues = SliderValues & CheckboxValues;

export const getConfigValues = () => {
  return {
    ...getSliderValues(),
    ...getCheckboxValues(),
  }
}

export const debounce = (func: Function, timeout = 100) => {
  let timer: number;
  return (...args: any[]) => {
    clearTimeout(timer);
    timer = setTimeout(() => {
      func.apply(this, args);

    }, timeout);
  }
}


export const initConfig = (
  initData: SimData,
  sim: Simulation<RawNode, RawEdge>,
  svg: Selection<SVGSVGElement, any, HTMLElement, any>,
  tooltip: Selection<HTMLDivElement, unknown, HTMLElement, any>
) => {
  initSliders(initData, sim, svg, tooltip);
  initCheckboxes(initData, sim, svg, tooltip);
}

