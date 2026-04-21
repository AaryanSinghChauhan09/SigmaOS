/**
 * SigmaOS Lattice Visualizer
 * Module 02: High-performance rendering of the Sovereign Suite Grid.
 */

const LatticeVisualizer = {
    init(gridId) {
        this.grid = document.getElementById(gridId);
        if (!this.grid) return;
        this.renderLattice();
    },

    renderLattice() {
        this.grid.innerHTML = '';
        SovereignRegistry.getAllSuites().forEach(suite => {
            const card = document.createElement('div');
            card.className = 'suite-card';
            card.id = `suite-${suite.id}`;
            card.title = suite.desc;
            
            card.innerHTML = `
                <span class="s-id">SUITE // ${suite.id}</span>
                <span class="s-name">${suite.name}</span>
                <div class="s-icon" style="font-size:24px; margin:10px 0;">${suite.icon}</div>
                <div class="s-status"></div>
            `;
            
            this.grid.appendChild(card);
        });
        console.log("Σ Lattice Visualizer: 33-Suite Grid Materialized.");
    },

    updateSuiteStatus(id, state) {
        const el = document.getElementById(`suite-${id}`);
        if (el) {
            el.classList.toggle('loaded', state === 'active');
            if (state === 'active') UIUtils.pulseElement(el, '0 0 20px var(--acc-cyan)');
        }
    }
};

window.LatticeVisualizer = LatticeVisualizer;
