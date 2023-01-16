import { BaseType, select, Selection } from "d3-selection";

export const handleZoom = (g: Selection<BaseType, unknown, HTMLElement, any>) => (e: any) => {
  g.attr('transform', e.transform);
}
export const initZoom = (zoom: any) => {
  select('svg')
    .call(zoom);
}



