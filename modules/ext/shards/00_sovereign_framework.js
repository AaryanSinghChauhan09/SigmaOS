/**
 * SigmaOS Sovereign UI Framework (v1.0)
 * Architecture: Object-Oriented Component System
 * Principles: Modularization, DRY, Encapsulation
 */

class SovereignComponent {
    constructor(id, data = {}) {
        this.id = id;
        this.data = data;
        this.element = document.getElementById(id);
    }

    render() {
        console.warn(`Render method not implemented for component: ${this.id}`);
    }

    // Utility to create a glass-panel container
    createContainer(classes = []) {
        const div = document.createElement('div');
        div.className = ['glass-panel', ...classes].join(' ');
        return div;
    }
}

class MissionControl extends SovereignComponent {
    render() {
        if (!this.element) return;
        const grid = this.element.querySelector('.mission-control-grid');
        if (!grid) return;

        grid.innerHTML = '';
        const missions = [
            { name: 'Analytics Dashboard', icon: '📊', desc: '3 Apps Open', theme: 'mission-analytics' },
            { name: 'Dev Environment (WSL Hook)', icon: '💻', desc: 'VS Code + Docker', theme: 'mission-dev' },
            { name: 'Creative Studio', icon: '📝', desc: 'Notes + Figma', theme: 'mission-creative' }
        ];

        missions.forEach(m => {
            const panel = this.createContainer(['mission-panel', m.theme]);
            panel.innerHTML = `
                <div class="mission-icon">${m.icon}</div>
                <h3 class="mission-name">${m.name}</h3>
                <p class="mission-desc">${m.desc}</p>
            `;
            grid.appendChild(panel);
        });
    }
}

class WorkspaceOrchestrator extends SovereignComponent {
    render() {
        const body = this.element.querySelector('.ai-body');
        if (!body) return;

        body.innerHTML = '';
        const grid = document.createElement('div');
        grid.className = 'workspace-grid-3';

        const types = [
            { title: 'Engineering', icon: '🛠️', desc: 'S04, S05, S07 Active', variant: 'workspace-card-cyan' },
            { title: 'Research', icon: '🧠', desc: 'S09, S13, S14 Active', variant: '' },
            { title: 'Security', icon: '🛡️', desc: 'S08, S10, S24 Active', variant: '' }
        ];

        types.forEach(t => {
            const card = this.createContainer(['workspace-card', t.variant].filter(Boolean));
            card.innerHTML = `
                <div class="card-icon-large">${t.icon}</div>
                <h4 class="card-title-spaced">${t.title}</h4>
                <p class="card-desc-tiny">${t.desc}</p>
                <button class="cyber-btn small-btn card-action-full">SWITCH</button>
            `;
            grid.appendChild(card);
        });
        body.appendChild(grid);
    }
}

class AutomationEngineUI extends SovereignComponent {
    render() {
        const body = this.element.querySelector('.ai-body');
        if (!body) return;

        body.innerHTML = `
            <div class="ai-segment">
                <h3 class="segment-title text-purple">Context-Aware Routines</h3>
                <p class="routine-segment-desc">Define logical conditions to alter your workspace state.</p>
                <ul class="workflow-list" id="routine-list"></ul>
                <button class="cyber-btn small-btn mt-10">+ NEW ROUTINE</button>
            </div>
        `;

        const list = body.querySelector('#routine-list');
        const routines = [
            { title: 'SigmaFlow: Neural Audit', desc: '[Idea 421] Sequential DAG-based task execution.', border: 'routine-border-cyan', accent: 'highlight-cyan' },
            { title: 'SigmaAgent: Autonomous Scraper', desc: '[Idea 436] AI-driven multi-step plan execution.', border: 'routine-border-magenta', accent: 'highlight-magenta' }
        ];

        routines.forEach(r => {
            const li = document.createElement('li');
            li.className = `routine-item ${r.border}`;
            li.innerHTML = `
                <div>
                    <strong class="${r.accent}">${r.title}</strong><br>
                    <span class="routine-desc-tiny">${r.desc}</span>
                </div>
                <button class="cyber-btn small-btn">INITIATE</button>
            `;
            list.appendChild(li);
        });
    }
}

class SystemMonitorUI extends SovereignComponent {
    render() {
        const body = this.element.querySelector('.ai-body');
        if (!body) return;

        body.innerHTML = `
            <div class="ai-segment monitor-item-spread">
                <div><h3 class="segment-title">CPU SHARD LATTICE</h3><p class="card-desc-tiny m-0">Load: 3% (Idle)</p></div>
                <div class="progress-track-small"><div class="progress-fill-h100 progress-fill-cyan" style="width:3%;"></div></div>
            </div>
            <div class="ai-segment monitor-item-spread">
                <div><h3 class="segment-title">MEMORY SHARDS</h3><p class="card-desc-tiny m-0">Used: 2.1 GB / 32 GB</p></div>
                <div class="progress-track-small"><div class="progress-fill-h100 progress-fill-magenta" style="width:6%;"></div></div>
            </div>
            <div class="ai-segment">
                <h3 class="segment-title">DISTRIBUTED LATTICE PEERS</h3>
                <div class="peer-list-tiny" id="peer-list"></div>
            </div>
        `;

        const peerList = body.querySelector('#peer-list');
        const peers = [
            { name: 'Peer_L_01 (Local Silicon)', status: 'ACTIVE MASTER', accent: 'highlight-cyan' },
            { name: 'Peer_R_02 (Remote Shard)', status: 'DISCONNECTED', accent: 'card-desc-tiny', disconnected: true },
            { name: 'Peer_Q_03 (Quantum Node)', status: 'SYNCHRONIZING', accent: 'highlight-magenta' }
        ];

        peers.forEach(p => {
            const div = document.createElement('div');
            div.className = `peer-item-spread ${p.disconnected ? 'peer-item-disconnected' : ''}`;
            div.innerHTML = `
                <span>${p.name}</span>
                <span class="${p.accent}">${p.status}</span>
            `;
            peerList.appendChild(div);
        });
    }
}

// Global UI Initializer
const SovereignUI = {
    components: {},
    init() {
        console.log("Σ Sovereign UI Framework: Bootstrapping Components...");
        this.components.missionControl = new MissionControl('mission-control-overlay');
        this.components.workspace = new WorkspaceOrchestrator('workspaces-view');
        this.components.automation = new AutomationEngineUI('automations-view');
        this.components.sysmon = new SystemMonitorUI('sys-monitor-view');

        // Render all
        Object.values(this.components).forEach(c => c.render());
    }
};

window.SovereignUI = SovereignUI;
document.addEventListener('DOMContentLoaded', () => SovereignUI.init());
