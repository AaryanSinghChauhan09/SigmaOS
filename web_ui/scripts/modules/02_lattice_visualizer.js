/**
 * SigmaOS Advanced Lattice Visualizer
 * Module 02: High-performance rendering of the Sovereign Suite Grid with dynamic shard interconnects.
 */

const LatticeVisualizer = {
    init(gridId) {
        this.grid = document.getElementById(gridId);
        if (!this.grid) return;
        
        this.svgLayer = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
        this.svgLayer.classList.add('lattice-connections');
        this.grid.appendChild(this.svgLayer);
        
        this.renderLattice();
    },

    renderLattice() {
        const suitsContainer = document.createElement('div');
        suitsContainer.className = 'suites-container';
        this.grid.appendChild(suitsContainer);

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
            
            suitsContainer.appendChild(card);
        });
        console.log("Σ Lattice Visualizer: 33-Suite Grid Materialized.");
    },

    updateSuiteStatus(id, state) {
        const el = document.getElementById(`suite-${id}`);
        if (el) {
            el.classList.toggle('loaded', state === 'active');
            if (state === 'active') {
                UIUtils.pulseElement(el, '0 0 20px var(--acc-cyan)');
                this.drawConnection(id);
            }
        }
    },

    drawConnection(targetId) {
        // Symbolic connection to S00 (Core)
        const core = document.getElementById('suite-S00');
        const target = document.getElementById(`suite-${targetId}`);
        if (!core || !target || core === target) return;

        const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
        const cRect = core.getBoundingClientRect();
        const tRect = target.getBoundingClientRect();
        const gRect = this.grid.getBoundingClientRect();

        line.setAttribute('x1', cRect.left + cRect.width/2 - gRect.left);
        line.setAttribute('y1', cRect.top + cRect.height/2 - gRect.top);
        line.setAttribute('x2', tRect.left + tRect.width/2 - gRect.left);
        line.setAttribute('y2', tRect.top + tRect.height/2 - gRect.top);
        line.setAttribute('stroke', 'var(--acc-cyan)');
        line.setAttribute('stroke-width', '1');
        line.setAttribute('opacity', '0.2');
        line.classList.add('lattice-line');

        this.svgLayer.appendChild(line);
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

window.LatticeVisualizer = LatticeVisualizer;
