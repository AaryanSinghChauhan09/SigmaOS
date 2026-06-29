/** sigma-pool-inspector — memory pool inspection panel */
export class SigmaPoolInspector extends HTMLElement {
  connectedCallback() { this.innerHTML = '<div class="sigma-pool-inspector">PoolInspector</div>'; }
}
customElements.define('sigma-pool-inspector', SigmaPoolInspector);
