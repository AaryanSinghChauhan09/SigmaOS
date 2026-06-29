/** sigma-logs — kernel log stream viewer */
export class SigmaLogs extends HTMLElement {
  connectedCallback() { this.innerHTML = '<div class="sigma-logs">SigmaLogs</div>'; }
}
customElements.define('sigma-logs', SigmaLogs);
