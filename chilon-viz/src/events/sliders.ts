import { RawData } from "../data/raw-data";

const handleMinSlider = (
  minSliderEl: HTMLInputElement | null,
  minSliderOutput: HTMLOutputElement | null,
  maxSliderEl: HTMLInputElement | null,
  maxSliderOutput: HTMLOutputElement | null,
) => {
  if (minSliderEl?.value) {
    let n = Math.abs(Number(minSliderEl.value))
    minSliderOutput!.textContent = n.toString();

    if (maxSliderEl?.value && Number(maxSliderEl.value) < Number(minSliderEl.value)) {
      maxSliderEl.value = minSliderEl.value;
      maxSliderOutput!.textContent = maxSliderEl.value;
    }
  }
};

const handleMaxSlider = (
  maxSliderEl: HTMLInputElement | null,
  maxSliderOutput: HTMLOutputElement | null,
  minSliderEl: HTMLInputElement | null,
  minSliderOutput: HTMLOutputElement | null,
) => {
  if (maxSliderEl?.value) {
    let n = Math.abs(Number(maxSliderEl.value))
    maxSliderOutput!.textContent = n.toString();

    if (minSliderEl?.value && Number(minSliderEl.value) > Number(maxSliderEl?.value)) {
      minSliderEl.value = maxSliderEl?.value;
      minSliderOutput!.textContent = minSliderEl.value;
    }
  }
};

export const getSliderElems = () => {
  return {
    minNodeOccursIn: document.querySelector<HTMLInputElement>('#minNodeOccursInput')!,
    maxNodeOccursIn: document.querySelector<HTMLInputElement>('#maxNodeOccursInput')!,
    minEdgeOccursIn: document.querySelector<HTMLInputElement>('#minEdgeOccursInput')!,
    maxEdgeOccursIn: document.querySelector<HTMLInputElement>('#maxEdgeOccursInput')!,
    minNodeOccursOut: document.querySelector<HTMLOutputElement>('#minNodeOccurs')!,
    minEdgeOccursOut: document.querySelector<HTMLOutputElement>('#minEdgeOccurs')!,
    maxNodeOccursOut: document.querySelector<HTMLOutputElement>('#maxNodeOccurs')!,
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
    minNodeOccurs: Number(elems.minNodeOccursIn.value),
    maxNodeOccurs: Number(elems.maxNodeOccursIn.value),
    minEdgeOccurs: Number(elems.minEdgeOccursIn.value),
    maxEdgeOccurs: Number(elems.maxEdgeOccursIn.value),
  };
}


export const initSliderValues = (data: RawData) => {
  let elems = getSliderElems();

  const maxNodes = data.nodes.slice(-1)[0].count.toString();
  const minNodeOccursInValue = data.nodes.slice(-1)[0].count.toString();

  elems.minNodeOccursIn.setAttribute('max', maxNodes);
  elems.maxNodeOccursIn.setAttribute('max', maxNodes);
  elems.maxNodeOccursIn.value = minNodeOccursInValue;
  elems.maxNodeOccursOut.textContent = maxNodes;

  const maxEdges = data.edges[0].count.toString();
  let minEdgeOccursInValue = data.edges.slice(-1)[0].count.toString();

  elems.minEdgeOccursIn.setAttribute('max', maxEdges);
  elems.maxEdgeOccursIn.setAttribute('max', maxEdges);
  elems.maxEdgeOccursIn.value = minEdgeOccursInValue;
  elems.maxEdgeOccursOut.textContent = maxEdges;
}



export const initSliderListeners = () => {
  const elems = getSliderElems();

  handleMinSlider(elems.minNodeOccursIn, elems.minNodeOccursOut, elems.maxNodeOccursIn, elems.maxNodeOccursOut);
  handleMaxSlider(elems.maxNodeOccursIn, elems.maxNodeOccursOut, elems.minNodeOccursIn, elems.minNodeOccursOut);
  handleMinSlider(elems.minEdgeOccursIn, elems.minEdgeOccursOut, elems.maxEdgeOccursIn, elems.maxEdgeOccursOut);
  handleMaxSlider(elems.maxEdgeOccursIn, elems.maxEdgeOccursOut, elems.minEdgeOccursIn, elems.minEdgeOccursOut);

  elems.minNodeOccursIn!.addEventListener('input', () => handleMinSlider(elems.minNodeOccursIn, elems.minNodeOccursOut, elems.maxNodeOccursIn, elems.maxNodeOccursOut));
  elems.maxNodeOccursIn!.addEventListener('input', () => handleMaxSlider(elems.maxNodeOccursIn, elems.maxNodeOccursOut, elems.minNodeOccursIn, elems.minNodeOccursOut));
  elems.minEdgeOccursIn!.addEventListener('input', () => handleMinSlider(elems.minEdgeOccursIn, elems.minEdgeOccursOut, elems.maxEdgeOccursIn, elems.maxEdgeOccursOut));
  elems.maxEdgeOccursIn!.addEventListener('input', () => handleMaxSlider(elems.maxEdgeOccursIn, elems.maxEdgeOccursOut, elems.minEdgeOccursIn, elems.minEdgeOccursOut));
};

