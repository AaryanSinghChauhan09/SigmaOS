"use strict";

/**
 * Σ SIGMA UI RENDERER
 * Specialized shard for industrial UI rendering.
 */
export class SigmaUIRenderer {
    constructor(system) {
        this.system = system;
    }

    renderMenu() {
        const grid = document.querySelector('.menu-items-grid');
        if (!grid) return;
        grid.innerHTML = '';
        
        const categories = {
            'System': this.system.store.shards.filter(s => s.domain === 'System' && s.enabled),
            'AI / Lab': this.system.store.shards.filter(s => (s.domain === 'AI' || s.domain === 'DS') && s.enabled),
            'Security': this.system.store.shards.filter(s => s.domain === 'Security' && s.enabled),
            'Design / Media': this.system.store.shards.filter(s => (s.domain === 'Design' || s.domain === 'Media') && s.enabled)
        };

        Object.entries(categories).forEach(([name, shards]) => {
            if (shards.length === 0) return;
            const header = document.createElement('div');
            header.className = 'menu-category-header u-accent-text u-bold';
            header.textContent = name;
            grid.appendChild(header);

            shards.forEach(s => {
                const card = document.createElement('div');
                card.className = 'menu-card';
                card.innerHTML = `<div class="u-font-size-lg">${s.icon}</div><div class="u-font-size-xxs u-margin-t-5">${s.name}</div>`;
                card.onclick = () => {
                    this.system.wm.open(s.id);
                    document.getElementById('sigma-menu').classList.add('hidden');
                };
                grid.appendChild(card);
            });
        });
    }

    renderShardManager() {
        const list = document.getElementById('shard-manager-list');
        if (!list) return;
        list.innerHTML = '';
        this.system.store.shards.forEach(s => {
            const item = document.createElement('div');
            item.className = 'metric-card u-flex-between';
            item.innerHTML = `
                <div>
                    <div class="u-bold">${s.icon} ${s.name}</div>
                    <div class="u-font-size-xxs u-muted-text">${s.description}</div>
                </div>
                <label class="switch">
                    <input type="checkbox" ${s.enabled ? 'checked' : ''} onchange="SIGMA.store.toggle('${s.id}')">
                    <span class="slider"></span>
                </label>
            `;
            list.appendChild(item);
        });
    }

    renderMatrix() {
        const dashboard = document.getElementById('matrix-dashboard');
        if (!dashboard) return;
        dashboard.innerHTML = this.system.store.matrixTools.map(t => `
            <div class="metric-card">
                <div class="metric-header"><span>${t.icon} ${t.name}</span> <span class="u-font-size-xxs u-muted-text">${t.USP}</span></div>
                <div class="u-font-size-xs u-margin-b-10">${t.desc}</div>
                <button class="status-chip" onclick="SIGMA.executeMatrixTool('${t.id}')">Activate Shard</button>
            </div>
        `).join('');
    }

    renderDistros() {
        const list = document.getElementById('distro-selector');
        if (!list) return;
        list.innerHTML = this.system.store.distros.map(d => `
            <div class="distro-card" onclick="SIGMA.launchDistro('${d.id}')">
                <div class="distro-icon">${d.icon}</div>
                <div class="distro-name">${d.name}</div>
            </div>
        `).join('');
    }
}
