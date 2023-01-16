import { Simulation } from "d3-force";
import { RawEdge, RawNode } from "../data/raw-data";
import * as d3Drag from 'd3-drag';

export const drag = (simulation: Simulation<RawNode, RawEdge>) => {

  function dragstarted(event: d3Drag.D3DragEvent<any, any, any>) {
    if (!event.active) simulation.alphaTarget(0.3).restart();
    event.subject.fx = event.subject.x;
    event.subject.fy = event.subject.y;
  }

  function dragged(event: d3Drag.D3DragEvent<any, any, any>) {
    event.subject.fx = event.x;
    event.subject.fy = event.y;
  }

  function dragended(event: d3Drag.D3DragEvent<any, any, any>) {
    if (!event.active) simulation.alphaTarget(0);
    event.subject.fx = null;
    event.subject.fy = null;
  }

  return d3Drag.drag()
    .on("start", dragstarted)
    .on("drag", dragged)
    .on("end", dragended);
}
