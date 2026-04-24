/**
 * SigmaOS Neural Spotlight (v3.0)
 * Module 02: High-fidelity, Spotlight-inspired universal command and shard orchestration.
 */

const NeuralSearch = {
    isOpen: false,
    results: [],
    activeIndex: 0,

    init() {
        console.log("Σ Neural Spotlight: Sentinel AI v3.0 Online.");
        this.overlay = document.getElementById('command-bar');
        this.input = document.getElementById('command-input');
        this.resultsList = document.getElementById('command-results');

        if (!this.overlay || !this.input) {
            console.error("Σ NeuralSearch: Required DOM elements missing.");
            return;
        }

        this.setupListeners();
    },

    setupListeners() {
        // Global Hotkey (Cmd/Ctrl + Space) - Competitor Grade
        document.addEventListener('keydown', (e) => {
            if ((e.metaKey || e.ctrlKey) && e.code === 'Space') {
                e.preventDefault();
                this.toggle();
            }
            if (e.key === 'Escape' && this.isOpen) {
                this.toggle(false);
            }
        });

        // Search Input logic
        this.input.addEventListener('input', () => this.performSearch());
        this.input.addEventListener('keydown', (e) => this.handleKeyboard(e));
    },

    toggle(forceState) {
        this.isOpen = forceState !== undefined ? forceState : !this.isOpen;
        this.overlay.classList.toggle('hidden', !this.isOpen);
        
        if (this.isOpen) {
            this.input.focus();
            this.input.value = '';
            this.performSearch(); // Show default/recent
        }
    },

    performSearch() {
        const query = this.input.value.toLowerCase().trim();
        this.results = [];

        // 1. Search Shards from Orchestrator
        if (window.ShardOrchestrator && window.ShardOrchestrator.manifest) {
            for (const [name, meta] of window.ShardOrchestrator.manifest) {
                if (name.toLowerCase().includes(query)) {
                    this.results.push({
                        title: name,
                        type: 'SHARD',
                        icon: '💠',
                        action: () => ShardOrchestrator.hotLoadShard(name)
                    });
                }
            }
        }

        // 2. Commands
        const commands = [
            { title: 'Reboot Lattice', type: 'SYSTEM', icon: '🔄', action: () => location.reload() },
            { title: 'Sovereign Blueprint', type: 'CONFIG', icon: '📜', action: () => LatticeConfig.openEditor() },
            { title: 'Lattice Stress Audit', type: 'INDUSTRIAL', icon: '🔥', action: () => SovereignStressTester.openDashboard() },
            { title: 'Toggle Light/Dark', type: 'THEME', icon: '🌓', action: () => ThemeEngine.toggle() },
            { title: 'Purge Dross', type: 'CLEANUP', icon: '🧹', action: () => UIUtils.appendLog('audit-log', 'Neural: Purging memory dross...', 'warning') }
        ];

        commands.forEach(cmd => {
            if (cmd.title.toLowerCase().includes(query)) {
                this.results.push(cmd);
            }
        });

        this.renderResults();
    },

    renderResults() {
        this.resultsList.innerHTML = '';
        this.activeIndex = 0;

        if (this.results.length === 0) {
            this.resultsList.innerHTML = '<div class="spotlight-msg">No sovereign matches found.</div>';
            return;
        }

        this.results.forEach((res, i) => {
            const div = document.createElement('div');
            div.className = `command-result-item ${i === 0 ? 'active' : ''}`;
            div.innerHTML = `
                <span class="res-icon">${res.icon}</span>
                <div class="res-info">
                    <div class="res-title">${res.title}</div>
                    <div class="res-type">${res.type}</div>
                </div>
            `;
            div.onclick = () => this.execute(res);
            this.resultsList.appendChild(div);
        });
    },

    handleKeyboard(e) {
        const items = this.resultsList.querySelectorAll('.command-result-item');
        
        if (e.key === 'ArrowDown') {
            e.preventDefault();
            this.activeIndex = (this.activeIndex + 1) % this.results.length;
            this.updateSelection(items);
        } else if (e.key === 'ArrowUp') {
            e.preventDefault();
            this.activeIndex = (this.activeIndex - 1 + this.results.length) % this.results.length;
            this.updateSelection(items);
        } else if (e.key === 'Enter') {
            if (this.results[this.activeIndex]) {
                this.execute(this.results[this.activeIndex]);
            }
        }
    },

    updateSelection(items) {
        items.forEach((item, i) => {
            item.classList.toggle('active', i === this.activeIndex);
            if (i === this.activeIndex) item.scrollIntoView({ block: 'nearest' });
        });
    },

    execute(item) {
        UIUtils.appendLog('audit-log', `Spotlight: Executing [${item.title}]`, 'info');
        item.action();
        this.toggle(false);
    }
};

window.NeuralSearch = NeuralSearch;
document.addEventListener('DOMContentLoaded', () => NeuralSearch.init());
