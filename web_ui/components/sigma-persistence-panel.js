/**
 * =============================================================================
 * Σ SIGMAOS: <sigma-persistence-panel> Web Component
 * =============================================================================
 * Real-time visualization of the S03_Orchestrator Decentralized Persistence Layer.
 * Shows Write, Checkpoint, Replicate, and Recover events as they flow through
 * the IPC and Persistence layers of the Sovereign Lattice.
 *
 * Usage: <sigma-persistence-panel></sigma-persistence-panel>
 * =============================================================================
 */

const PERSIST_SHARDS = [
    'S01_Genesis', 'S03_Orchestrator', 'S04_HAL', 
    'S05_Memory', 'S07_Scheduling', 'S08_Security', 'S09_Intelligence'
];

const PERSIST_EVENT_TYPES = {
    WRITE:      { label: 'Write',      color: '#10b981', bg: 'rgba(16,185,129,0.15)',  icon: '💾' },
    CHECKPOINT: { label: 'Checkpoint', color: '#60a5fa', bg: 'rgba(96,165,250,0.15)',  icon: '📸' },
    REPLICATE:  { label: 'Replicate',  color: '#fbbf24', bg: 'rgba(251,191,36,0.15)',  icon: '🔁' },
    RECOVER:    { label: 'Recover',    color: '#f87171', bg: 'rgba(248,113,113,0.15)', icon: '♻️' },
    VERIFY:     { label: 'Verify',     color: '#a78bfa', bg: 'rgba(167,139,250,0.15)', icon: '✅' },
};

class SigmaPersistencePanel extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this._events = [];
        this._counters = { WRITE: 0, CHECKPOINT: 0, REPLICATE: 0, RECOVER: 0, VERIFY: 0 };
        this._shardState = {};
        this._interval = null;
        PERSIST_SHARDS.forEach(s => { this._shardState[s] = { active: false, lastEvent: null }; });
    }

    connectedCallback() {
        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: block;
                    font-family: 'Inter', system-ui, sans-serif;
                    background: rgba(0,0,0,0.4);
                    border: 1px solid rgba(167,139,250,0.25);
                    border-radius: 14px;
                    padding: 20px;
                    margin-top: 24px;
                }

                .header {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    margin-bottom: 20px;
                }
                .title {
                    font-size: 0.9rem;
                    text-transform: uppercase;
                    letter-spacing: 0.1em;
                    color: #a78bfa;
                    font-weight: 700;
                }
                .legend {
                    display: flex;
                    gap: 12px;
                }
                .legend-item {
                    display: flex;
                    align-items: center;
                    gap: 4px;
                    font-size: 0.65rem;
                    color: #94a3b8;
                }
                .legend-dot {
                    width: 8px;
                    height: 8px;
                    border-radius: 50%;
                }

                .main-grid {
                    display: grid;
                    grid-template-columns: 1fr 1.6fr;
                    gap: 20px;
                }

                /* Shard Map */
                .shard-map-title {
                    font-size: 0.65rem;
                    text-transform: uppercase;
                    color: #64748b;
                    letter-spacing: 0.08em;
                    margin-bottom: 10px;
                }
                .shard-grid {
                    display: grid;
                    grid-template-columns: 1fr 1fr;
                    gap: 8px;
                }
                .shard-node {
                    background: rgba(255,255,255,0.03);
                    border: 1px solid rgba(255,255,255,0.08);
                    border-radius: 8px;
                    padding: 10px;
                    transition: all 0.3s ease;
                    cursor: pointer;
                }
                .shard-node.active {
                    border-color: var(--node-color, #10b981);
                    background: var(--node-bg, rgba(16,185,129,0.1));
                    box-shadow: 0 0 12px var(--node-color, #10b981);
                }
                .shard-name {
                    font-size: 0.6rem;
                    color: #94a3b8;
                    white-space: nowrap;
                    overflow: hidden;
                    text-overflow: ellipsis;
                }
                .shard-icon {
                    font-size: 1rem;
                    display: block;
                    margin-bottom: 4px;
                }

                /* Counters */
                .counters {
                    display: grid;
                    grid-template-columns: repeat(5, 1fr);
                    gap: 6px;
                    margin-top: 12px;
                }
                .counter-chip {
                    background: rgba(255,255,255,0.03);
                    border: 1px solid rgba(255,255,255,0.08);
                    border-radius: 6px;
                    padding: 6px;
                    text-align: center;
                }
                .counter-val {
                    font-size: 1.1rem;
                    font-weight: 700;
                    color: #e2e8f0;
                }
                .counter-label {
                    font-size: 0.5rem;
                    text-transform: uppercase;
                    color: #64748b;
                    margin-top: 2px;
                }

                /* Event Timeline */
                .timeline-title {
                    font-size: 0.65rem;
                    text-transform: uppercase;
                    color: #64748b;
                    letter-spacing: 0.08em;
                    margin-bottom: 10px;
                }
                .timeline {
                    display: flex;
                    flex-direction: column;
                    gap: 5px;
                    max-height: 280px;
                    overflow-y: auto;
                    scrollbar-width: thin;
                    scrollbar-color: rgba(255,255,255,0.08) transparent;
                }
                .event-row {
                    display: flex;
                    align-items: center;
                    gap: 10px;
                    padding: 7px 10px;
                    border-radius: 6px;
                    font-size: 0.7rem;
                    border-left: 3px solid transparent;
                    background: rgba(255,255,255,0.02);
                    transition: background 0.2s;
                    animation: slideIn 0.3s ease;
                }
                .event-row:hover { background: rgba(255,255,255,0.05); }

                @keyframes slideIn {
                    from { opacity: 0; transform: translateX(-8px); }
                    to   { opacity: 1; transform: translateX(0); }
                }

                .event-icon { font-size: 0.9rem; }
                .event-time { color: #475569; font-size: 0.6rem; min-width: 64px; }
                .event-shard { color: #94a3b8; font-size: 0.65rem; min-width: 90px; }
                .event-badge {
                    font-size: 0.6rem;
                    padding: 2px 7px;
                    border-radius: 10px;
                    font-weight: 600;
                    min-width: 72px;
                    text-align: center;
                }
                .event-detail { color: #64748b; font-size: 0.6rem; flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
            </style>

            <div class="header">
                <span class="title">⬡ Decentralized Persistence Layer</span>
                <div class="legend">
                    ${Object.entries(PERSIST_EVENT_TYPES).map(([k,v]) => `
                        <div class="legend-item">
                            <div class="legend-dot" style="background:${v.color}"></div>${v.label}
                        </div>
                    `).join('')}
                </div>
            </div>

            <div class="main-grid">
                <!-- Left: Shard Map + Counters -->
                <div>
                    <div class="shard-map-title">Lattice Shard Map</div>
                    <div class="shard-grid" id="shardMap">
                        ${PERSIST_SHARDS.map(s => `
                            <div class="shard-node" id="shard-${s.replace(/[^a-z0-9]/gi,'_')}">
                                <span class="shard-icon">⬡</span>
                                <div class="shard-name">${s}</div>
                            </div>
                        `).join('')}
                    </div>
                    <div class="counters" id="counters">
                        ${Object.entries(PERSIST_EVENT_TYPES).map(([k,v]) => `
                            <div class="counter-chip">
                                <div class="counter-val" id="ctr-${k}" style="color:${v.color}">0</div>
                                <div class="counter-label">${v.label}</div>
                            </div>
                        `).join('')}
                    </div>
                </div>

                <!-- Right: Event Timeline -->
                <div>
                    <div class="timeline-title">Persistence Event Timeline</div>
                    <div class="timeline" id="timeline">
                        <div style="text-align:center;color:rgba(255,255,255,0.2);font-size:0.7rem;padding:20px;">
                            Waiting for persistence events...
                        </div>
                    </div>
                </div>
            </div>
        `;

        // Start simulated persistence event stream
        this._startSimulation();

        // Subscribe to real IPC-persistence events from the API
        if (window.SigmaAPI) {
            this._unsub = window.SigmaAPI.subscribe('logs', (entry) => {
                const types = Object.keys(PERSIST_EVENT_TYPES);
                const matched = types.find(t => entry.msg.includes(t.charAt(0) + t.slice(1).toLowerCase()));
                if (matched) this._pushEvent(matched, entry.module || 'Unknown', entry.msg);
            });
        }
    }

    disconnectedCallback() {
        if (this._interval) clearInterval(this._interval);
        if (this._unsub) this._unsub();
    }

    _startSimulation() {
        const eventTypes = Object.keys(PERSIST_EVENT_TYPES);
        this._interval = setInterval(() => {
            const type  = eventTypes[Math.floor(Math.random() * eventTypes.length)];
            const shard = PERSIST_SHARDS[Math.floor(Math.random() * PERSIST_SHARDS.length)];
            const details = {
                WRITE:      `key=state:${Math.floor(Math.random()*9999)}, size=${Math.floor(Math.random()*1024)}B`,
                CHECKPOINT: `chkpt-${Math.random().toString(36).slice(2,10)}`,
                REPLICATE:  `→ ${PERSIST_SHARDS[Math.floor(Math.random()*PERSIST_SHARDS.length)]}`,
                RECOVER:    `from chkpt-${Math.random().toString(36).slice(2,8)}`,
                VERIFY:     `integrity OK (SHA-256 match)`,
            };
            this._pushEvent(type, shard, details[type]);
        }, 900);
    }

    _pushEvent(type, shard, detail) {
        const meta = PERSIST_EVENT_TYPES[type] || PERSIST_EVENT_TYPES.WRITE;
        const now = new Date();
        const timeStr = `${now.getHours().toString().padStart(2,'0')}:${now.getMinutes().toString().padStart(2,'0')}:${now.getSeconds().toString().padStart(2,'0')}`;

        // Update counter
        this._counters[type] = (this._counters[type] || 0) + 1;
        const ctrEl = this.shadowRoot.getElementById(`ctr-${type}`);
        if (ctrEl) ctrEl.textContent = this._counters[type];

        // Flash shard node
        const nodeId = `shard-${shard.replace(/[^a-z0-9]/gi,'_')}`;
        const node = this.shadowRoot.getElementById(nodeId);
        if (node) {
            node.style.setProperty('--node-color', meta.color);
            node.style.setProperty('--node-bg', meta.bg);
            node.classList.add('active');
            node.querySelector('.shard-icon').textContent = meta.icon;
            setTimeout(() => {
                node.classList.remove('active');
                node.querySelector('.shard-icon').textContent = '⬡';
            }, 1400);
        }

        // Add to timeline
        this._events.unshift({ type, shard, detail, timeStr, meta });
        if (this._events.length > 60) this._events.pop();

        const timeline = this.shadowRoot.getElementById('timeline');
        const row = document.createElement('div');
        row.className = 'event-row';
        row.style.borderLeftColor = meta.color;
        row.innerHTML = `
            <span class="event-icon">${meta.icon}</span>
            <span class="event-time">${timeStr}</span>
            <span class="event-shard">${shard}</span>
            <span class="event-badge" style="background:${meta.bg};color:${meta.color}">${meta.label}</span>
            <span class="event-detail">${detail}</span>
        `;
        timeline.insertBefore(row, timeline.firstChild);
        if (timeline.children.length > 50) timeline.removeChild(timeline.lastChild);
    }
}

customElements.define('sigma-persistence-panel', SigmaPersistencePanel);
