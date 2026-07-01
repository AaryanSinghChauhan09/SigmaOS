// File: zenith_desktop/modules/widgets/cpu-widget.js

export class CpuWidget extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({mode: 'open'});
        this.shadowRoot.innerHTML = `
            <style>
                .widget-title {
                    font-size: 1.1em;
                    margin-bottom: 12px;
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                }
                .accent { color: var(--accent, #00f0ff); font-size: 0.8em; font-weight: 800; text-transform: uppercase; letter-spacing: 1px; }
                .stat-value { font-size: 2.5em; font-weight: 800; margin: 15px 0; font-family: 'JetBrains Mono', monospace; text-shadow: 0 0 20px rgba(255,255,255,0.2); }
                .widget-graph { width: 100%; height: 40px; margin-bottom: 15px; }
                .stat-bar { background: rgba(255,255,255,0.1); height: 4px; border-radius: 2px; overflow: hidden; margin-bottom: 8px; width: 100%; }
                .stat-progress { background: var(--accent, #00f0ff); height: 100%; box-shadow: 0 0 10px var(--accent, #00f0ff); transition: width 0.3s ease; }
            </style>
            <div class="widget-title">
                <span>Silicon Audit</span>
                <span class="accent">16 Cores</span>
            </div>
            <div class="stat-value" id="cpu-load">12%</div>
            <svg viewBox="0 0 300 40" class="widget-graph">
                <polyline id="cpu-graph" fill="none" stroke="var(--accent, #00f0ff)" stroke-width="2"
                    points="0,40 30,30 60,35 90,20 120,25 150,10 180,15 210,5 240,12 270,8 300,20" />
            </svg>
            <div class="stat-bar">
                <div class="stat-progress" id="cpu-progress" style="width: 12%"></div>
            </div>
        `;
        
        this.domCache = {
            cpuLoad: this.shadowRoot.getElementById('cpu-load'),
            cpuProgress: this.shadowRoot.getElementById('cpu-progress')
        };
    }

    connectedCallback() {
        // Assume telemetry event bus triggers 'cpu_update'
        import('../core/eventBus.js').then(({ on }) => {
            on('cpu_update', this.updateStats.bind(this));
        });
    }

    disconnectedCallback() {
        import('../core/eventBus.js').then(({ off }) => {
            off('cpu_update', this.updateStats.bind(this));
        });
    }

    updateStats(cpu) {
        if (this.domCache.cpuLoad) this.domCache.cpuLoad.textContent = cpu + '%';
        if (this.domCache.cpuProgress) this.domCache.cpuProgress.style.width = cpu + '%';
    }
}

customElements.define('sigma-cpu', CpuWidget);
