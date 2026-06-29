/** sigma-persistence-panel — state persistence controls */
export class SigmaPersistencePanel extends HTMLElement {
  connectedCallback() { this.innerHTML = '<div class="sigma-persistence-panel">Persistence</div>'; }
}
customElements.define('sigma-persistence-panel', SigmaPersistencePanel);
