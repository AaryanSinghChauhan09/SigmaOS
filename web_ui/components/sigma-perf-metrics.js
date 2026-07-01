/** sigma-perf-metrics — performance counter dashboard */
export class SigmaPerfMetrics extends HTMLElement {
  connectedCallback() { this.innerHTML = '<div class="sigma-perf-metrics">PerfMetrics</div>'; }
}
customElements.define('sigma-perf-metrics', SigmaPerfMetrics);
