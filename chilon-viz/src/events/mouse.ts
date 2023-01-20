import { BaseType, select, Selection } from "d3-selection";

export const addMouseEventListeners = (svg: Selection<SVGSVGElement, any, HTMLElement, any>, tooltip: Selection<HTMLDivElement, unknown, HTMLElement, any>) => {
  const nodes = svg.selectAll('g.nodes g.node-g circle');
  const circle = svg.selectAll('g.nodes g.node-g circle');
  let edgepaths = svg.selectAll("g.edges g.edge-g path");
  addNodeEventListeners(circle, edgepaths, tooltip);
  addEdgeEventListeners(circle, edgepaths, tooltip);
}

const addNodeEventListeners = (
  circle: Selection<BaseType, unknown, SVGSVGElement, any>,
  edgepaths: Selection<BaseType, unknown, SVGSVGElement, any>,
  tooltip: Selection<HTMLDivElement, unknown, HTMLElement, any>
) => {
  circle
    .on('mouseover', function(event, d) {
      const hlNodes = new Set();
      edgepaths.each(function(d: any) {
        const s = d.source;
        const t = d.target;
        const circleName = event.target.getAttribute('data-id')
        if (circleName && t.name === circleName || s.name === circleName) {
          hlNodes.add(String(s.name));
          hlNodes.add(String(t.name));
        }
        const strokeWidth = (this as Element).getAttribute('data-stroke-width')
        const stroke = (this as Element).getAttribute('data-stroke')
        select(this as Element)
          .transition()
          .duration(200)
          .style('opacity', s.name == circleName || t.name == circleName ? 1 : 0.3)
          .style('stroke', s.name == circleName || t.name == circleName ? stroke : '#b8b8b8')
          .style('stroke-width', s.name == circleName || t.name == circleName ? strokeWidth : 1)
      });

      circle.each(function(n) {
        select(this)
          .transition()
          .duration(200)
          .style('fill', (n: any) => {
            return hlNodes.has(n.name) ? this.getAttribute('data-fill') : '#b8b8b8'
          })


        tooltip.transition()
          .duration(200)
          .style("opacity", 1) // show the tooltip
        tooltip.html(d.name)
          .style("left", (event.clientX + 20) + "px")
          .style("top", (event.clientY - 20) + "px");

      });
      //edges
      //  .style('stroke', link_d => link_d.source === d.id || link_d.target === d.id ? '#69b3b2' : '#b8b8b8')
      //  .style('stroke-width', link_d => link_d.source === d.id || link_d.target === d.id ? 4 : 1)
    })
    .on('mouseout', function(d) {
      circle.each(function(c) {
        select(this)
          .transition()
          .duration(200)
          .style('fill', this.getAttribute('data-fill'))
      });
      edgepaths.each(function(e) {
        select(this)
          .transition()
          .duration(200)
          .style('opacity', 0.3)
          .style('stroke', this.getAttribute('data-stroke'))
          //.style("stroke", '#b8b8b8')
          .style('stroke-width', this.getAttribute('data-stroke-width'))
      })
      tooltip.transition()
        .duration(200)
        .style("opacity", 0)
    });

}


const addEdgeEventListeners = (
  circle: Selection<BaseType, unknown, SVGSVGElement, any>,
  edgepaths: Selection<BaseType, unknown, SVGSVGElement, any>,
  tooltip: Selection<HTMLDivElement, unknown, HTMLElement, any>
) => {

  edgepaths
    .on('mouseover', function(event, d) {
      const hlNodes = new Set();
      edgepaths.each(function({ label, source, target }) {
        const strokeWidth = this.getAttribute('data-stroke-width')
        const stroke = this.getAttribute('data-stroke')

        if (d.label == label) {
          hlNodes.add(String(source.id));
          hlNodes.add(String(target.id));
        }
        select(this)
          .transition()
          .duration(200)
          .style('opacity', 1)
          .style('stroke', label == d.label ? stroke : '#b8b8b8')
          .style('stroke-width', label == d.label ? strokeWidth : 1)
      });

      circle.each(function(n) {
        select(this)
          .transition()
          .duration(200)
          .style('fill', n => {
            return hlNodes.has(String(n.id)) ? this.getAttribute('data-fill') : '#b8b8b8'
          })
      });


      tooltip.transition()
        .duration(200)
        .style("opacity", 1) // show the tooltip
      tooltip.html(d.label)
        .style("left", (event.clientX + 20) + "px")
        .style("top", (event.clientY - 20) + "px");
    })
    .on('mouseout', function(event, d) {
      edgepaths.each(function(e) {
        select(this)
          .transition()
          .duration(200)
          .style('opacity', 0.3)
          .style('stroke', this.getAttribute('data-stroke'))
          //.style("stroke", '#b8b8b8')
          .style('stroke-width', this.getAttribute('data-stroke-width'))
      })

      circle.each(function(c) {
        select(this)
          .transition()
          .duration(200)
          .style('fill', this.getAttribute('data-fill'))
      });

      tooltip.transition()
        .duration(200)
        .style("opacity", 0)
    });
}
