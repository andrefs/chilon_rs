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


export const initSliderValues = (data: RawData) => {
  const minNodeOccursIn = document.querySelector<HTMLInputElement>('#minNodeOccursInput')!;
  const maxNodeOccursIn = document.querySelector<HTMLInputElement>('#maxNodeOccursInput')!;
  const minEdgeOccursIn = document.querySelector<HTMLInputElement>('#minEdgeOccursInput')!;
  const maxEdgeOccursIn = document.querySelector<HTMLInputElement>('#maxEdgeOccursInput')!;
  const maxNodeOccursOut = document.querySelector<HTMLOutputElement>('#maxNodeOccurs')!;
  const maxEdgeOccursOut = document.querySelector<HTMLOutputElement>('#maxEdgeOccurs')!;

  const maxNodes = data.nodes.slice(-1)[0].count.toString();
  const minNodeOccursInValue = data.nodes.slice(-1)[0].count.toString();

  minNodeOccursIn.setAttribute('max', maxNodes);
  maxNodeOccursIn.setAttribute('max', maxNodes);
  maxNodeOccursIn.value = minNodeOccursInValue;
  maxNodeOccursOut.textContent = maxNodes;

  const maxEdges = data.edges[0].count.toString();
  let minEdgeOccursInValue = data.edges.slice(-1)[0].count.toString();

  minEdgeOccursIn.setAttribute('max', maxEdges);
  maxEdgeOccursIn.setAttribute('max', maxEdges);
  maxEdgeOccursIn.value = minEdgeOccursInValue;
  maxEdgeOccursOut.textContent = maxEdges;
}


export const initSliderListeners = () => {
  const minNodeOccursIn = document.querySelector<HTMLInputElement>('#minNodeOccursInput');
  const minNodeOccursOut = document.querySelector<HTMLOutputElement>('#minNodeOccurs');

  const maxNodeOccursIn = document.querySelector<HTMLInputElement>('#maxNodeOccursInput');
  const maxNodeOccursOut = document.querySelector<HTMLOutputElement>('#maxNodeOccurs');

  const minEdgeOccursIn = document.querySelector<HTMLInputElement>('#minEdgeOccursInput');
  const minEdgeOccursOut = document.querySelector<HTMLOutputElement>('#minEdgeOccurs');

  const maxEdgeOccursIn = document.querySelector<HTMLInputElement>('#maxEdgeOccursInput');
  const maxEdgeOccursOut = document.querySelector<HTMLOutputElement>('#maxEdgeOccurs');

  handleMinSlider(minNodeOccursIn, minNodeOccursOut, maxNodeOccursIn, maxNodeOccursOut);
  handleMaxSlider(maxNodeOccursIn, maxNodeOccursOut, minNodeOccursIn, minNodeOccursOut);
  handleMinSlider(minEdgeOccursIn, minEdgeOccursOut, maxEdgeOccursIn, maxEdgeOccursOut);
  handleMaxSlider(maxEdgeOccursIn, maxEdgeOccursOut, minEdgeOccursIn, minEdgeOccursOut);

  minNodeOccursIn!.addEventListener('input', () => handleMinSlider(minNodeOccursIn, minNodeOccursOut, maxNodeOccursIn, maxNodeOccursOut));
  maxNodeOccursIn!.addEventListener('input', () => handleMaxSlider(maxNodeOccursIn, maxNodeOccursOut, minNodeOccursIn, minNodeOccursOut));
  minEdgeOccursIn!.addEventListener('input', () => handleMinSlider(minEdgeOccursIn, minEdgeOccursOut, maxEdgeOccursIn, maxEdgeOccursOut));
  maxEdgeOccursIn!.addEventListener('input', () => handleMaxSlider(maxEdgeOccursIn, maxEdgeOccursOut, minEdgeOccursIn, minEdgeOccursOut));
};

