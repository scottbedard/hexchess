import { board } from './constants'

export class Hexboard extends HTMLElement {
  constructor() {
    super();
  }

  connectedCallback() {
    this.innerHTML = '<div>soon...</div>';

    console.log({ board })
  }
}
