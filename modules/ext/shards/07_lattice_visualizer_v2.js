/**
 * Lattice Visualizer v2.0 (High Fidelity)
 * A dynamic SVG-based component for real-time suite state visualization.
 * Uses Silicon Primitives for performance-critical DOM updates.
 */

class LatticeVisualizer extends ZenithComponent {
    constructor() {
        super('lattice-grid');
        this.svgNamespace = "http://www.w3.org/2000/svg";
        this.suites = [];
        this.initLattice();
    }

    initLattice() {
        if (!this.element) return;
        
        // Clear existing grid
        this.element.innerHTML = '';
        
        // Create 33 suites with individual pulse logic
        for (let i = 1; i <= 33; i++) {
            const suiteId = `S${i.toString().padStart(2, '0')}`;
            const card = document.createElement('div');
            card.className = 'suite-card glass-panel';
            card.id = `card-${suiteId}`;
            
            card.innerHTML = `
                <div class="suite-header">
                    <span class="s-id">${suiteId}</span>
                    <div class="s-indicator"></div>
                </div>
                <div class="s-label">SYNCHRONIZING...</div>
                <div class="s-load-bar"><div class="s-load-fill"></div></div>
            `;
            
            this.element.appendChild(card);
            this.suites.push({ id: suiteId, element: card });
        }
        
        this.startHeartbeat();
    }

    startHeartbeat() {
        // Randomly update suites to simulate "sentient" activity
        setInterval(() => {
            const randomSuite = this.suites[Math.floor(Math.random() * this.suites.length)];
            this.pulseSuite(randomSuite);
        }, 800);
    }

    pulseSuite(suite) {
        const fill = suite.element.querySelector('.s-load-fill');
        const label = suite.element.querySelector('.s-label');
        const load = Math.floor(Math.random() * 40) + 60; // 60-100%
        
        if (fill) fill.style.width = `${load}%`;
        if (label) label.textContent = `LATTICE LOAD: ${load}%`;
        
        suite.element.classList.add('pulse-active');
        setTimeout(() => suite.element.classList.remove('pulse-active'), 400);
    }
}

window.LatticeVisualizer = LatticeVisualizer;
