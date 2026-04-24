/**
 * =============================================================================
 * Σ SIGMAOS: <sigma-monitor> Web Component
 * =============================================================================
 * Self-contained real-time system vitals monitor.
 * Consumes data ONLY through window.SigmaAPI (versioned API layer).
 *
 * Usage: <sigma-monitor></sigma-monitor>
 * No props needed — auto-subscribes to vitals channel.
 * =============================================================================
 */

class SigmaMonitor extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this._unsub = null;
    }

    connectedCallback() {
        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
                    gap: 12px;
                    font-family: 'Inter', system-ui, sans-serif;
                }
                .card {
                    background: rgba(255,255,255,0.04);
                    border: 1px solid rgba(255,255,255,0.08);
                    border-radius: 14px;
                    padding: 16px 18px;
                    backdrop-filter: blur(16px);
                    transition: transform 0.2s ease, box-shadow 0.2s ease;
                }
                .card:hover {
                    transform: translateY(-2px);
                    box-shadow: 0 8px 24px rgba(0,0,0,0.3);
                }
                .label {
                    font-size: 0.65rem;
                    letter-spacing: 0.12em;
                    text-transform: uppercase;
                    color: rgba(255,255,255,0.45);
                    margin-bottom: 6px;
                }
                .value {
                    font-size: 1.6rem;
                    font-weight: 700;
                    color: #e0e7ff;
                    font-variant-numeric: tabular-nums;
                }
                .bar-container {
                    margin-top: 8px;
                    height: 4px;
                    background: rgba(255,255,255,0.06);
                    border-radius: 2px;
                    overflow: hidden;
                }
                .bar-fill {
                    height: 100%;
                    border-radius: 2px;
                    transition: width 0.6s cubic-bezier(0.4, 0, 0.2, 1);
                    background: linear-gradient(90deg, #6366f1, #a78bfa);
                }
                .bar-fill.warn  { background: linear-gradient(90deg, #f59e0b, #ef4444); }
            </style>
            <div class="card" id="cpu">
                <div class="label">CPU Load</div>
                <div class="value" id="cpuVal">0%</div>
                <div class="bar-container"><div class="bar-fill" id="cpuBar"></div></div>
            </div>
            <div class="card" id="mem">
                <div class="label">Memory</div>
                <div class="value" id="memVal">0 MB</div>
                <div class="bar-container"><div class="bar-fill" id="memBar"></div></div>
            </div>
            <div class="card" id="proc">
                <div class="label">Processes</div>
                <div class="value" id="procVal">0</div>
            </div>
            <div class="card" id="shards">
                <div class="label">Active Shards</div>
                <div class="value" id="shardVal">0</div>
            </div>
            <div class="card" id="uptime">
                <div class="label">Uptime</div>
                <div class="value" id="uptimeVal">0s</div>
            </div>
        `;

        // Subscribe to real-time vitals via the versioned API
        if (window.SigmaAPI) {
            this._unsub = window.SigmaAPI.subscribe('vitals', (v) => this._update(v));
        }
    }

    disconnectedCallback() {
        if (this._unsub) this._unsub();
    }

    _update(v) {
        const $ = (id) => this.shadowRoot.getElementById(id);
        const cpu = Math.round(v.cpu_usage);
        $('cpuVal').textContent  = `${cpu}%`;
        $('cpuBar').style.width  = `${cpu}%`;
        $('cpuBar').className    = cpu > 80 ? 'bar-fill warn' : 'bar-fill';

        const memMB = v.memory_usage;
        const memPct = Math.min(100, (memMB / 512) * 100);
        $('memVal').textContent  = `${memMB} MB`;
        $('memBar').style.width  = `${memPct}%`;
        $('memBar').className    = memPct > 80 ? 'bar-fill warn' : 'bar-fill';

        $('procVal').textContent  = v.process_count;
        $('shardVal').textContent = v.active_shards;

        const secs = Math.floor(v.uptime_ms / 1000);
        const m = Math.floor(secs / 60);
        const s = secs % 60;
        $('uptimeVal').textContent = m > 0 ? `${m}m ${s}s` : `${s}s`;
    }
}

customElements.define('sigma-monitor', SigmaMonitor);
