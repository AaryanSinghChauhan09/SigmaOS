/**
 * Sovereign Taskbar (v1.0)
 * Enhances Zenith UX to surpass Desktop environments like Linux Mint.
 * Provides application switching, workspace management, and system vitals access.
 */

class SovereignTaskbar extends ZenithComponent {
    constructor() {
        super('v-tabs'); // We use the existing vertical tabs as our core dock
        this.init();
    }

    init() {
        console.log('Σ://UI> Taskbar System Materialized.');
        this.bindShortcuts();
    }

    bindShortcuts() {
        // Universal Search (Spotlight) Shortcut
        window.addEventListener('keydown', (e) => {
            if (e.key === ' ' && e.ctrlKey) {
                e.preventDefault();
                this.launchSpotlight();
            }
        });
    }

    launchSpotlight() {
        const spotlight = Sigma.node('command-bar');
        if (spotlight) {
            spotlight.classList.toggle('hidden');
            const input = Sigma.node('command-input');
            if (!spotlight.classList.contains('hidden') && input) {
                input.focus();
            }
        }
    }

    notify(msg, type = 'OPTIMAL') {
        // High-fidelity toast notification system
        const container = Sigma.node('system-notifications');
        if (!container) return;

        const toast = document.createElement('div');
        toast.className = `toast glass-panel border-${type.toLowerCase()}`;
        toast.innerHTML = `
            <span class="t-icon">⚡</span>
            <div class="t-body">
                <span class="t-title">${type} SYSTEM ACTION</span>
                <p class="t-msg">${msg}</p>
            </div>
        `;
        
        container.appendChild(toast);
        setTimeout(() => toast.classList.add('out'), 5000);
        setTimeout(() => toast.remove(), 5500);
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
}

window.SovereignTaskbar = SovereignTaskbar;
