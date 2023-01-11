
/*****************
 * DEFINING STUFF *
 *****************/


const normalizeCounts = (elems) => {
  let [max, min] = elems.reduce((acc, cur) => {
    let [max, min] = acc;
    let count = cur.count;

    return [count > max ? count : max, count < min ? count : min];
  }, [elems[0]?.count || 0, elems[0]?.count || 0]);

  let delta = max - min;

  for (let elem of elems) {
    elem.normCount = Math.floor(((elem.count - min) / delta) * 50) + 50;
  }
}

window.debFilterData = debounce(() => filterData());


const update = (data) => {

  let nodesParent = d3.select("svg g.nodes");
  let edgesParent = d3.select("svg g.edges");

  const nodeGroups = nodesParent
    .selectAll("g")
    .data(data.nodes);
  nodeGroups
    .enter()
    .append('g').attr('class', 'node-g');
  nodeGroups.exit().remove();

  var nodes = nodeGroups
    .append("circle")
    .attr("r", d => Math.ceil(d.normCount || 0))
    .style("fill", (d, i) => '#B3D9CB')
    .attr('data-id', (d, i) => d.id)
    .attr("data-fill", (d, i) => '#B3D9CB')
    .style("pointer-events", "visiblePainted")
    .style('cursor', 'pointer')

  var nodelabels = nodeGroups
    .append("text")
    .attr("x", d => d.x)
    .attr("y", d => d.y)
    .attr('font-size', d => Math.ceil(d.normCount / 2))
    .attr('class', "nodelabel")
    .text(d => d.name)
    .attr('dominant-baseline', 'middle')
    .style("pointer-events", "none")
    .style('cursor', 'pointer')



  const edgeGroups = edgesParent
    .selectAll("g")
    .data(data.links);
  edgeGroups
    .enter()
    .append('g').attr('class', 'edge-g')

  edgeGroups.exit().remove();

  var edgepaths = edgeGroups
    .append('path')
    //.attr('d',linkArc)
    //.attr('d', d => {
    //  console.log('XXXXXXXXXXXX d', d);
    //  return 'M '+d.source.x+' '+d.source.y+' L '+ d.target.x +' '+d.target.y
    //})
    .attr('class', 'edgepath')
    .attr('fill-opacity', 0)
    .attr('id', (d, i) => 'edgepath' + i)
    .attr("stroke-width", d => Math.ceil(d.normCount / 10))
    .attr('opacity', 0.3)
    .attr("data-stroke-width", d => Math.ceil(d.normCount / 10))
    .attr("data-source", d => d.source)
    .attr("data-target", d => d.target)
    .attr("data-label", d => d.label)
    .style("stroke", (d) => colorHash[d.label])
    //.style("stroke", '#b8b8b8')
    .attr("data-stroke", (d) => colorHash[d.label])
    //.style("pointer-events", "none");
    .style("pointer-events", "visibleStroke")
    .attr('marker-end', 'url(#triangle)')

  var edgelabels = edgeGroups.append('text')
    .style("pointer-events", "none")
    .attr('class', 'edgelabel')
    .attr('id', (d, i) => 'edgelabel' + i)
    .attr('text-anchor', 'middle')
    .attr('dominant-baseline', 'text-after-edge')
    .attr('font-size', 15)
    .attr('fill', '#999')

  edgelabels.append('textPath')
    .attr('xlink:href', (d, i) => '#edgepath' + i)
    .style("pointer-events", "none")
    .attr('startOffset', '50%')
    .attr('text-anchor', 'middle')
    .attr('text-anchor', 'middle')
  //.text(d => d.label)

  restartSimulation(window.simulation);
}


window.filterData = () => {
  const minNodes = d3.select('#minNodeOccursInput').node().value;
  const maxNodes = d3.select('#maxNodeOccursInput').node().value;
  const minEdges = d3.select('#minPredicateOccursInput').node().value;
  const maxEdges = d3.select('#maxPredicateOccursInput').node().value;


  window.data.nodes = originalData.nodes.filter(n => n.count >= minNodes && n.count <= maxNodes);
  window.data.links = originalData.links.filter(n => n.count >= minEdges && n.count <= maxEdges);
  console.log('XXXXXXXXXXX filterData', { minNodes, maxNodes, minEdges, maxEdges, windowData: window.data });

  update(window.data);
};

const setSliders = (nodes, links) => {
  d3.select('#maxNodeOccursInput').node().max = nodes.totalMaxSlider;
  d3.select('#maxNodeOccursInput').node().value = nodes.maxSlider;
  d3.select('#maxNodeOccurs').text(nodes.maxSlider);

  d3.select('#minNodeOccursInput').node().min = nodes.totalMinSlider;
  d3.select('#minNodeOccursInput').node().value = nodes.minSlider;
  d3.select('#minNodeOccurs').text(nodes.minSlider);

  d3.select('#maxPredicateOccursInput').node().max = links.totalMaxSlider;
  d3.select('#maxPredicateOccursInput').node().value = links.maxSlider;
  d3.select('#maxPredicateOccurs').text(links.maxSlider);

  d3.select('#minPredicateOccursInput').node().min = links.totalMinSlider;
  d3.select('#minPredicateOccursInput').node().value = links.minSlider;
  d3.select('#minPredicateOccurs').text(links.minSlider);

  filterData();
};

const calcInitValues = (data) => {
  let NODES = 50;
  let EDGES = 50;

  window.data = {
    nodes: data.nodes.slice(0, 50),
    links: data.links.slice(0, 50)
  };

  let nodes = {
    minSlider: data.nodes.slice(-1)[0].count,
    totalMinSlider: 0,
    maxSlider: data.nodes[0].count,
    totalMaxSlider: data.nodes[0].count,
  };

  let links = {
    minSlider: data.links.slice(-1)[0].count,
    totalMinSlider: 0,
    maxSlider: data.links[0].count,
    totalMaxSlider: data.links[0].count,
  };



  setSliders(nodes, links);
}


//const setSliders = (data) => {
//  const totalNodes = data.nodes.length;
//  const totalEdges = data.links.length;
//  d3.select('#maxNodeCountInput').node().max = totalNodes;
//  d3.select('#maxNodeCountInput').node().value = totalNodes;
//  d3.select('#maxNodeCount').text(totalNodes);
//
//  d3.select('#minNodeCountInput').node().max = totalNodes - 1;
//
//  d3.select('#maxPredicateCountInput').node().max = totalEdges;
//  d3.select('#maxPredicateCountInput').node().value = totalEdges;
//  d3.select('#maxPredicateCount').text(totalEdges)
//
//  d3.select('#minPredicateCountInput').node().max = totalEdges - 1;
//};

/**********
 * Colors *
 **********/

const RGB2Color = (r, g, b) => '#' + byte2Hex(r) + byte2Hex(g) + byte2Hex(b);
const byte2Hex = n => {
  const nybHexString = "0123456789ABCDEF";
  return String(nybHexString.substr((n >> 4) & 0x0F, 1)) +
    nybHexString.substr(n & 0x0F, 1);
};
const makeColorGradient = (frequency1, frequency2, frequency3, phase1, phase2, phase3, center, width, len) => {
  const colors = []
  if (len == undefined) { len = 50; }
  if (center == undefined) { center = 128; }
  if (width == undefined) { width = 127; }

  for (let i = 0; i < len; ++i) {
    const red = Math.sin(frequency1 * i + phase1) * width + center;
    const grn = Math.sin(frequency2 * i + phase2) * width + center;
    const blu = Math.sin(frequency3 * i + phase3) * width + center;
    colors.push(RGB2Color(red, grn, blu));
  }
  return colors;
};

const genColors = numColors => {
  let center = 128;
  let width = 127;
  let frequency = 2.4;
  return makeColorGradient(frequency, frequency, frequency, 0, 2, 4, center, width, numColors);
};


/**********
 * Zoom   *
 **********/

function initZoom() {
  d3.select('svg')
    .call(zoom);
}

function handleZoom(e) {
  d3.selectAll('svg g')
    .attr('transform', e.transform);
}

const calcEdge = (d) => {
  let signal = 0;
  if (d.linknum % 2 === 1 && d.linknum > 0) { signal = 1; }
  if (d.linknum % 2 === 0 && d.linknum < 0) { signal = 1; }
  const dl = Math.abs(d.linknum)
  const divisor = Math.floor(dl / 2) * 2;
  const dr = dl === 1 ? 0 : 1500 / divisor;  //linknum is defined above

  const pathd = `M${d.source.x},${d.source.y}
                 A${dr},${dr} 0 0 ${signal} ${d.target.x},${d.target.y}`;
  return pathd;
};

const calcLoop = (d) => {
  const dl = Math.abs(d.linknum)
  const dr = 40 + d.normCount * dl;  //linknum is defined above

  //loop
  //d="M334.5179247605647,472.7245628100564
  //     A73,73 -45 1 1 335.5179247605647,473.7245628100564"
  const pathd = `M${d.source.x},${d.source.y}
                   A${dr},${dr} -45 1 0 ${d.target.x + 1},${d.target.y + 1}`;
  return pathd;

}

function ticked() {
  let edgepaths = d3.selectAll("svg g.edge-g path");
  edgepaths.attr('d', function(d) {
    const sId = d.source?.id ?? d.source;
    const tId = d.target?.id ?? d.target;
    return sId === tId ? calcLoop(d) : calcEdge(d);
  });

  let nodeGroups = d3.selectAll("svg g.node-g");

  let nodes = nodeGroups.selectAll("circle");
  nodes.attr("cx", d => d.x)
    .attr("cy", d => d.y)

  let nodelabels = nodeGroups.selectAll("text");
  nodelabels.attr("x", d => d.x)
    .attr("y", d => d.y);

  d3.select('#alpha_value').style('flex-basis', (simulation.alpha() * 100) + '%');
}


/*****************
 * RUNNING STUFF *
 *****************/

window.originalData = {{ data | json_encode(pretty = true) | safe }};

normalizeCounts(originalData.nodes);
normalizeCounts(originalData.links);
calcInitValues(originalData);
window.data.links = [...originalData.links].sort(function(a, b) {
  const aFields = [a.source, a.target].sort();
  const bFields = [b.source, b.target].sort();
  if (aFields[0] > bFields[0]) { return 1; }
  else if (aFields[0] < bFields[0]) { return -1; }
  else {
    if (aFields[1] > bFields[1]) { return 1; }
    if (aFields[1] < bFields[1]) { return -1; }
    else { return 0; }
  }
});

//any links with duplicate source and target get an incremented 'linknum'
for (let i = 0; i < data.links.length; i++) {
  data.links[i].count /= 500;
  //if (i != 0 &&
  //  data.links[i].source === data.links[i-1].source &&
  //  data.links[i].target === data.links[i-1].target) {
  //    data.links[i].linknum = data.links[i-1].linknum + 1;
  //  }
  //else { data.links[i].linknum = 1; }


  if (i === 0) {
    data.links[0].linknum = 1;
    continue;
  }

  const aSrc = data.links[i].source;
  const aTgt = data.links[i].target;
  const bSrc = data.links[i - 1].source;
  const bTgt = data.links[i - 1].target;
  const label = data.links[i].label;

  if (aSrc === bSrc && aTgt === bTgt) {
    data.links[i].linknum = Math.abs(data.links[i - 1].linknum) + 1;
  }
  else if (aSrc === bTgt && aTgt === bSrc) {
    //const signal = -Math.sign(data.links[i-1].linknum);
    //data.links[i].linknum = (Math.abs(data.links[i-1].linknum) + 1)*signal;
    data.links[i].linknum = -(Math.abs(data.links[i - 1].linknum) + 1);
  }
  else { data.links[i].linknum = 1; }
};


window.data.nodes = [...originalData.nodes].map(n => {
  n.count = Math.ceil(5 + 10 * Math.log2(n.count));
  return n;
});
const resources = new Set(window.data.nodes.map(n => n.name));
const predicates = new Set(window.data.links.flatMap(l => [l.label]))




//const nodeColors = genColors(data.nodes.length);
const edgeColors = genColors(data.links.length);
//const colorEntries = [...Array.from(resources).map((r, i) => [r, nodeColors[i]]),
//                      ...Array.from(predicates).map((p, i) => [p, edgeColors[i]])];
const colorHash = Object.fromEntries(Array.from(predicates).map((p, i) => ([p, edgeColors[i]])));


/**********
 * Zoom   *
 **********/

let zoom = d3.zoom()
  .scaleExtent([0.1, 5])
  //.translateExtent([[0, 0], [width, height]])
  .on('zoom', handleZoom);



var svg = d3.select("svg")
const width = svg.node().getBoundingClientRect().width;
const height = svg.node().getBoundingClientRect().height;


/**********
 * Simulation  *
 **********/


function debounce(func, timeout = 100) {
  let timer;
  return (...args) => {
    clearTimeout(timer);
    timer = setTimeout(() => {
      func.apply(this, args);

    }, timeout);
  }
}

function restartSimulation(simulation) {
  if (!simulation) { return; }
  console.log('XXXXXXXXXXX restartSimulation', { simulation, windowData: window.data, originalData: window.originalData })
  simulation.nodes(window.data.nodes);
  simulation.force("linkForce", d3.forceLink(data.links).distance(300).strength(2));
  simulation.alpha(1).restart();
  simulation.on('tick', ticked);
}

function initSimulation(simulation, data) {
  if (!simulation) { return; }
  console.log('XXXXXXXXXXX initSimulation', { windowData: window.data, simulation })
  simulation.nodes(data.nodes);
  simulation.force("linkForce", d3.forceLink(data.links).distance(300).strength(2));
  simulation.force("charge", d3.forceManyBody().strength(-800).distanceMin(200).distanceMax(400));
  simulation.force('collision', d3.forceCollide().radius(d => d.normCount + 4));
  simulation.force('center', d3.forceCenter(width / 2, height / 2));
  simulation.on("tick", ticked);
}


const nodeDistance = 300;
window.simulation = d3.forceSimulation();


svg.append('g').attr('class', 'edges')
svg.append('g').attr('class', 'nodes');
update(window.data);

initSimulation(window.simulation, window.data);


/**********
 * Tooltip *
 **********/

const tooltip = d3.select('#main')
  .append("div")
  .classed("tooltip", true)
  .style("opacity", 0) // start invisible

//    /**********
//     * Events *
//     **********/
//    
//    const circle = svg.selectAll('circle');
//    circle
//      .on('mouseover', function(event, d) {
//        const hlNodes = new Set();
//        edgepaths.each(function(d) {
//          const s = d.source;
//          const t = d.target;
//          const circleId = event.target.getAttribute('data-id')
//          if (circleId && t.id == circleId || s.id == circleId) {
//            hlNodes.add(String(s.id));
//            hlNodes.add(String(t.id));
//          }
//          const strokeWidth = this.getAttribute('data-stroke-width')
//          const stroke = this.getAttribute('data-stroke')
//          d3.select(this)
//            .transition()
//            .duration(200)
//            .style('opacity', s.id == circleId || t.id == circleId ? 1 : 0.3)
//            .style('stroke', s.id == circleId || t.id == circleId ? stroke : '#b8b8b8')
//            .style('stroke-width', s.id == circleId || t.id == circleId ? strokeWidth : 1)
//        });
//    
//        nodes.each(function(n) {
//          d3.select(this)
//            .transition()
//            .duration(200)
//            .style('fill', n => {
//              return hlNodes.has(String(n.id)) ? this.getAttribute('data-fill') : '#b8b8b8'
//            })
//    
//    
//          tooltip.transition()
//            .duration(200)
//            .style("opacity", 1) // show the tooltip
//          tooltip.html(d.name)
//            .style("left", (event.clientX + 20) + "px")
//            .style("top", (event.clientY - 20) + "px");
//    
//        });
//        //edges
//        //  .style('stroke', link_d => link_d.source === d.id || link_d.target === d.id ? '#69b3b2' : '#b8b8b8')
//        //  .style('stroke-width', link_d => link_d.source === d.id || link_d.target === d.id ? 4 : 1)
//      })
//      .on('mouseout', function(d) {
//        circle.each(function(c) {
//          d3.select(this)
//            .transition()
//            .duration(200)
//            .style('fill', this.getAttribute('data-fill'))
//        });
//        edgepaths.each(function(e) {
//          d3.select(this)
//            .transition()
//            .duration(200)
//            .style('opacity', 0.3)
//            .style('stroke', this.getAttribute('data-stroke'))
//            //.style("stroke", '#b8b8b8')
//            .style('stroke-width', this.getAttribute('data-stroke-width'))
//        })
//        tooltip.transition()
//          .duration(200)
//          .style("opacity", 0)
//      });
//    
//    edgepaths
//      .on('mouseover', function(event, d) {
//        const hlNodes = new Set();
//        edgepaths.each(function({ label, source, target }) {
//          const strokeWidth = this.getAttribute('data-stroke-width')
//          const stroke = this.getAttribute('data-stroke')
//    
//          if (d.label == label) {
//            hlNodes.add(String(source.id));
//            hlNodes.add(String(target.id));
//          }
//          d3.select(this)
//            .transition()
//            .duration(200)
//            .style('opacity', 1)
//            .style('stroke', label == d.label ? stroke : '#b8b8b8')
//            .style('stroke-width', label == d.label ? strokeWidth : 1)
//        });
//    
//        nodes.each(function(n) {
//          d3.select(this)
//            .transition()
//            .duration(200)
//            .style('fill', n => {
//              return hlNodes.has(String(n.id)) ? this.getAttribute('data-fill') : '#b8b8b8'
//            })
//        });
//    
//    
//        tooltip.transition()
//          .duration(200)
//          .style("opacity", 1) // show the tooltip
//        tooltip.html(d.label)
//          .style("left", (event.clientX + 20) + "px")
//          .style("top", (event.clientY - 20) + "px");
//      })
//      .on('mouseout', function(event, d) {
//        edgepaths.each(function(e) {
//          d3.select(this)
//            .transition()
//            .duration(200)
//            .style('opacity', 0.3)
//            .style('stroke', this.getAttribute('data-stroke'))
//            //.style("stroke", '#b8b8b8')
//            .style('stroke-width', this.getAttribute('data-stroke-width'))
//        })
//    
//        circle.each(function(c) {
//          d3.select(this)
//            .transition()
//            .duration(200)
//            .style('fill', this.getAttribute('data-fill'))
//        });
//    
//        tooltip.transition()
//          .duration(200)
//          .style("opacity", 0)
//      });


/**********
 * Tick  *
 **********/

initZoom();


