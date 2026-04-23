/**
 * SigmaOS Nexus Engine
 * Module 03: App Store logic and Sovereign Package integration.
 */

const NexusEngine = {
    apps: [
        { name: "Sovereign Editor", icon: "📝", category: "Utility", status: "Installed" },
        { name: "Aether Browser", icon: "🌐", category: "Network", status: "Available" },
        { name: "Neural Studio", icon: "🎨", category: "Creative", status: "Available" },
        { name: "Sigma Monitor", icon: "📊", category: "System", status: "Installed" },
        { name: "Zenith Terminal", icon: "🐚", category: "System", status: "Installed" }
    ],

    init(containerId) {
        this.renderApps(containerId);
    },

    renderApps(containerId) {
        const container = document.getElementById(containerId);
        if (!container) return;

        container.innerHTML = '';
        this.apps.forEach(app => {
            const card = document.createElement('div');
            card.className = "app-card glass-panel";
            card.innerHTML = `
                <div class="app-icon">${app.icon}</div>
                <div class="app-info">
                    <span class="app-name">${app.name}</span>
                    <span class="app-cat">${app.category}</span>
                </div>
                <button class="cyber-btn ${app.status === 'Installed' ? 'secondary' : 'primary'}">
                    ${app.status === 'Installed' ? 'OPEN' : 'GET'}
                </button>
            `;
            
            const btn = card.querySelector('button');
            btn.onclick = () => this.handleAction(app, btn);
            container.appendChild(card);
        });
    },

    handleAction(app, btn) {
        if (app.status === 'Installed') {
            UIUtils.appendLog('audit-log', `Launching Sovereign App: ${app.name}`, 'success');
        } else {
            btn.textContent = 'SYNCING...';
            btn.disabled = true;
            UIUtils.appendLog('audit-log', `Downloading Lattice Shards for ${app.name}...`, 'normal');
            
            setTimeout(() => {
                app.status = 'Installed';
                btn.textContent = 'OPEN';
                btn.disabled = false;
                btn.classList.replace('primary', 'secondary');
                UIUtils.appendLog('audit-log', `${app.name} assimilated successfully.`, 'success');
            }, 2000);
        }
    }

    selfEvolve() {
        const mutations = [
            "Optimizing lattice resonance...",
            "Expanding semantic context...",
            "Hardening silicon primitives...",
            "Refining cross-kernel synthesis..."
        ];
        const mutation = mutations[Math.floor(Math.random() * mutations.length)];
        console.log(`Σ://EVOLUTION [${this.shardId}]> ${mutation}`);
        this.lastMutation = mutation;
    }
};

window.NexusEngine = NexusEngine;
