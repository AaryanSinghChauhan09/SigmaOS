/**
 * =============================================================================
 * Σ SIGMAOS: <sigma-logs> Web Component
 * =============================================================================
 * Real-time kernel log viewer with severity filtering.
 * Consumes data ONLY through window.SigmaAPI (versioned API layer).
 *
 * Usage: <sigma-logs max-entries="100"></sigma-logs>
 * =============================================================================
 */

class SigmaLogs extends HTMLElement {
    static get observedAttributes() { return ['max-entries']; }

    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this._unsub = null;
        this._entries = [];
        this._maxEntries = 100;
        this._filter = 'ALL';
    }

    connectedCallback() {
        this._maxEntries = parseInt(this.getAttribute('max-entries') || '100', 10);

        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: block;
                    font-family: 'Inter', 'JetBrains Mono', monospace;
                    background: rgba(0,0,0,0.3);
                    border: 1px solid rgba(255,255,255,0.06);
                    border-radius: 14px;
                    overflow: hidden;
                }
                .toolbar {
                    display: flex;
                    gap: 8px;
                    padding: 10px 14px;
                    background: rgba(255,255,255,0.03);
                    border-bottom: 1px solid rgba(255,255,255,0.06);
                    align-items: center;
                }
                .toolbar span {
                    font-size: 0.7rem;
                    letter-spacing: 0.1em;
                    text-transform: uppercase;
                    color: rgba(255,255,255,0.4);
                    margin-right: auto;
                }
                .filter-btn {
                    font-size: 0.65rem;
                    padding: 3px 10px;
                    border-radius: 6px;
                    border: 1px solid rgba(255,255,255,0.1);
                    background: transparent;
                    color: rgba(255,255,255,0.5);
                    cursor: pointer;
                    transition: all 0.15s ease;
                }
                .filter-btn:hover, .filter-btn.active {
                    background: rgba(99,102,241,0.2);
                    color: #a5b4fc;
                    border-color: rgba(99,102,241,0.4);
                }
                .log-container {
                    max-height: 320px;
                    overflow-y: auto;
                    padding: 8px 0;
                    scrollbar-width: thin;
                    scrollbar-color: rgba(255,255,255,0.1) transparent;
                }
                .log-entry {
                    display: grid;
                    grid-template-columns: 100px 50px 120px 1fr;
                    gap: 8px;
                    padding: 4px 14px;
                    font-size: 0.72rem;
                    color: rgba(255,255,255,0.55);
                    transition: background 0.15s ease;
                }
                .log-entry:hover {
                    background: rgba(255,255,255,0.03);
                }
                .ts    { color: rgba(255,255,255,0.3); font-variant-numeric: tabular-nums; }
                .level { font-weight: 600; }
                .level.INFO  { color: #6ee7b7; }
                .level.WARN  { color: #fbbf24; }
                .level.DEBUG { color: #93c5fd; }
                .level.ERROR { color: #f87171; }
                .module { color: #a78bfa; }
                .msg    { color: rgba(255,255,255,0.65); }
                .empty-state {
                    padding: 24px;
                    text-align: center;
                    color: rgba(255,255,255,0.25);
                    font-size: 0.8rem;
                }
            </style>
            <div class="toolbar">
                <span>Kernel Logs</span>
                <button class="filter-btn active" data-filter="ALL">All</button>
                <button class="filter-btn" data-filter="INFO">Info</button>
                <button class="filter-btn" data-filter="WARN">Warn</button>
                <button class="filter-btn" data-filter="DEBUG">Debug</button>
            </div>
            <div class="log-container" id="logs">
                <div class="empty-state">Waiting for kernel events...</div>
            </div>
        `;

        // Filter buttons
        this.shadowRoot.querySelectorAll('.filter-btn').forEach(btn => {
            btn.addEventListener('click', () => {
                this.shadowRoot.querySelectorAll('.filter-btn').forEach(b => b.classList.remove('active'));
                btn.classList.add('active');
                this._filter = btn.dataset.filter;
                this._render();
            });
        });

        // Subscribe to live log stream
        if (window.SigmaAPI) {
            window.SigmaAPI.getLogs(this._maxEntries).then(logs => {
                this._entries = logs;
                this._render();
            });
            this._unsub = window.SigmaAPI.subscribe('logs', (entry) => {
                this._entries.push(entry);
                if (this._entries.length > this._maxEntries) this._entries.shift();
                this._render();
            });
        }
    }

    disconnectedCallback() {
        if (this._unsub) this._unsub();
    }

    _render() {
        const container = this.shadowRoot.getElementById('logs');
        const filtered = this._filter === 'ALL'
            ? this._entries
            : this._entries.filter(e => e.level === this._filter);

        if (filtered.length === 0) {
            container.innerHTML = '<div class="empty-state">No matching entries</div>';
            return;
        }

        container.innerHTML = filtered.map(e => {
            const ts = e.ts ? new Date(e.ts).toLocaleTimeString() : '--';
            return `<div class="log-entry">
                <span class="ts">${ts}</span>
                <span class="level ${e.level}">${e.level}</span>
                <span class="module">${e.module}</span>
                <span class="msg">${e.msg}</span>
            </div>`;
        }).join('');

        container.scrollTop = container.scrollHeight;
    }
}

customElements.define('sigma-logs', SigmaLogs);
