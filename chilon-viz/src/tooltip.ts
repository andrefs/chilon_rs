import { select } from "d3-selection"

export const createTooltip = () => {
  return select('#app')
    .append("div")
    .classed("tooltip", true)
    .style("opacity", 0) // start invisible

}
