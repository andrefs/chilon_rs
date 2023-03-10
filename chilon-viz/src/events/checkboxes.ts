
export const getCheckboxElems = () => {
  return {
    logarithm: document.querySelector<HTMLInputElement>('#cb-logarithm')!,
    loops: document.querySelector<HTMLInputElement>('#cb-loops')!,
    bau: document.querySelector<HTMLInputElement>('#cb-bau')!,
    outer: document.querySelector<HTMLInputElement>('#cb-outer')!,
    datatypes: document.querySelector<HTMLInputElement>('#cb-datatypes')!,
  }
}


export type CheckboxValues = {
  logarithm: boolean,
  loops: boolean,
  bau: boolean,
  outer: boolean,
  datatypes: boolean,
}

export const getCheckboxValues = () => {
  const elems = getCheckboxElems();
  console.log('XXXXXXX checkboxes', { elems })

  return {
    logarithm: elems.logarithm.checked,
    loops: elems.loops.checked,
    bau: elems.bau.checked,
    outer: elems.outer.checked,
    datatypes: elems.datatypes.checked,
  }
}


