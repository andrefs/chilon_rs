import { SimData } from "../data/raw-data";
import rangeSlider from 'range-slider-input';
import 'range-slider-input/dist/style.css';

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



export const initSliderValues = (initData: SimData) => {
  let elems = getSliderElems();


  rangeSlider(elems.nodeOccursIn, {
    min: 0,
    max: initData.nodes[0].count,
    value: [initData.nodes.slice(-1)[0].count, initData.nodes[0].count],
    onInput: (value) => {
      elems.minNodeOccursOut.value = value[0].toString();
      elems.maxNodeOccursOut.value = value[1].toString();
    }
  });
  rangeSlider(elems.edgeOccursIn, {
    min: 0,
    max: initData.edges[0].count,
    value: [initData.edges.slice(-1)[0].count, initData.edges[0].count],
    onInput: (value) => {
      elems.minEdgeOccursOut.value = value[0].toString();
      elems.maxEdgeOccursOut.value = value[1].toString();
    }
  });
}



