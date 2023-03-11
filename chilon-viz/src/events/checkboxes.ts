import { Simulation } from "d3-force";
import { Selection } from "d3-selection";
import { filterData } from "../data";
import { RawEdge, RawNode, SimData } from "../data/raw-data";
import { update } from "../simulation";
import { getConfigValues } from "./config";

export const getCheckboxElems = () => {
  return {
    logarithm: document.querySelector<HTMLInputElement>('#cb-logarithm')!,
    loops: document.querySelector<HTMLInputElement>('#cb-loops')!,
    bau: document.querySelector<HTMLInputElement>('#cb-bau')!,
    outer: document.querySelector<HTMLInputElement>('#cb-outer')!,
    datatypes: document.querySelector<HTMLInputElement>('#cb-datatypes')!,
    disconnected: document.querySelector<HTMLInputElement>('#cb-disconnected')!,
  }
}


export type CheckboxValues = {
  logarithm: boolean,
  loops: boolean,
  bau: boolean,
  outer: boolean,
  datatypes: boolean,
  disconnected: boolean
}

export const getCheckboxValues = () => {
  const elems = getCheckboxElems();

  return {
    logarithm: elems.logarithm.checked,
    loops: elems.loops.checked,
    bau: elems.bau.checked,
    outer: elems.outer.checked,
    datatypes: elems.datatypes.checked,
    disconnected: elems.disconnected.checked,
  }
}


export const initCheckboxes = (
  initData: SimData,
  sim: Simulation<RawNode, RawEdge>,
  svg: Selection<SVGSVGElement, any, HTMLElement, any>,
  tooltip: Selection<HTMLDivElement, unknown, HTMLElement, any>
) => {
  const elems = getCheckboxElems();

  for (const [_, elem] of Object.entries(elems)) {
    elem.addEventListener('change', () => {
      const config = getConfigValues();
      const data = filterData(initData, config);
      update(data, sim, svg, tooltip);
    })
  }
}

