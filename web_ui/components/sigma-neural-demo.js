/** sigma-neural-demo — on-device neural inference demo */
export class SigmaNeuralDemo extends HTMLElement {
  connectedCallback() { this.innerHTML = '<div class="sigma-neural-demo">NeuralDemo</div>'; }
}
customElements.define('sigma-neural-demo', SigmaNeuralDemo);
