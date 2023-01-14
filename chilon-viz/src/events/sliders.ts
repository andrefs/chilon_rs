
const handleMinSlider = (
  minSliderEl: HTMLInputElement | null,
  sliderOutput: HTMLOutputElement | null,
  maxSliderEl: HTMLInputElement | null
) => {
  if (minSliderEl?.value) {
    let n = Math.abs(Number(minSliderEl.value))
    sliderOutput!.textContent = n.toString();

    if (maxSliderEl?.value && Number(maxSliderEl.value) < Number(minSliderEl.value)) {
      maxSliderEl.value = minSliderEl.value;
    }
  }
};

const handleMaxSlider = (maxSliderEl: HTMLInputElement | null, sliderOutput: HTMLOutputElement | null, minSliderEl: HTMLInputElement | null) => {
  if (maxSliderEl?.value) {
    let n = Math.abs(Number(maxSliderEl.value))
    sliderOutput!.textContent = n.toString();

    if (minSliderEl?.value && Number(minSliderEl.value) > Number(maxSliderEl?.value)) {
      minSliderEl.value = maxSliderEl?.value;
    }
  }
};



export const initSliderListeners = () => {
  const minNodeOccursIn = document.querySelector<HTMLInputElement>('#minNodeOccursInput');
  const minNodeOccursOut = document.querySelector<HTMLOutputElement>('#minNodeOccurs');

  const maxNodeOccursIn = document.querySelector<HTMLInputElement>('#maxNodeOccursInput');
  const maxNodeOccursOut = document.querySelector<HTMLOutputElement>('#maxNodeOccurs');

  const minEdgeOccursIn = document.querySelector<HTMLInputElement>('#minEdgeOccursInput');
  const minEdgeOccursOut = document.querySelector<HTMLOutputElement>('#minEdgeOccurs');

  const maxEdgeOccursIn = document.querySelector<HTMLInputElement>('#maxEdgeOccursInput');
  const maxEdgeOccursOut = document.querySelector<HTMLOutputElement>('#maxEdgeOccurs');

  handleMinSlider(minNodeOccursIn, minNodeOccursOut, maxNodeOccursIn);
  handleMaxSlider(maxNodeOccursIn, maxNodeOccursOut, minNodeOccursIn);
  handleMinSlider(minEdgeOccursIn, minEdgeOccursOut, maxEdgeOccursIn);
  handleMaxSlider(maxEdgeOccursIn, maxEdgeOccursOut, minEdgeOccursIn);

  minNodeOccursIn!.addEventListener('input', () => handleMinSlider(minNodeOccursIn, minNodeOccursOut, maxNodeOccursIn));
  maxNodeOccursIn!.addEventListener('input', () => handleMaxSlider(maxNodeOccursIn, maxNodeOccursOut, minNodeOccursIn));
  minEdgeOccursIn!.addEventListener('input', () => handleMinSlider(minEdgeOccursIn, minEdgeOccursOut, maxEdgeOccursIn));
  maxEdgeOccursIn!.addEventListener('input', () => handleMaxSlider(maxEdgeOccursIn, maxEdgeOccursOut, minEdgeOccursIn));
};

