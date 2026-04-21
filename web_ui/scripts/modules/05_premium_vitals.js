/**
 * SigmaOS Zenith Premium Vitals
 * Module 05: Dynamic Dock & Immersive Effects
 */

const PremiumVitals = {
    init() {
        console.log("Σ Zenith Premium Vitals Initializing...");
        this.setupDock();
        this.enhanceBackground();
    },

    setupDock() {
        const dock = document.getElementById('sovereign-dock');
        
        // Show dock when GUI is active
        const observer = new MutationObserver((mutations) => {
            const guiView = document.getElementById('gui-view');
            if (!guiView.classList.contains('hidden')) {
                dock.classList.add('visible');
            } else {
                dock.classList.remove('visible');
            }
        });

        observer.observe(document.getElementById('gui-view'), { attributes: true, attributeFilter: ['class'] });

        // Dock Item Interactions
        const dockItems = document.querySelectorAll('.dock-item');
        dockItems.forEach(item => {
            item.addEventListener('click', () => {
                dockItems.forEach(i => i.classList.remove('active'));
                item.classList.add('active');
                
                const type = item.getAttribute('data-tooltip');
                this.handleDockAction(type);
            });
        });
    },

    handleDockAction(type) {
        console.log(`Dock Action: ${type}`);
        // Integration with other modules
        if (type.includes("Neural Search")) {
            document.getElementById('command-bar').classList.remove('hidden');
            document.getElementById('command-input').focus();
        }
        if (type.includes("Shell")) {
            // Trigger CLI view transition
            document.getElementById('btn-cli').click();
        }
    },

    enhanceBackground() {
        // Multi-layered mouse tracking for orbs
        document.addEventListener('mousemove', (e) => {
            const x = (e.clientX / window.innerWidth - 0.5) * 40;
            const y = (e.clientY / window.innerHeight - 0.5) * 40;
            
            document.querySelectorAll('.orb').forEach((orb, i) => {
                const speed = 1 + (i * 0.5);
                orb.style.transform = `translate(${x * speed}px, ${y * speed}px)`;
            });
        });
    },

    visualizeLattice() {
        console.log("Σ Rendering Advanced Lattice Connections...");
        const metrics = [
            "Quantum-Link: SECURE (QKD-E8)",
            "Bio-Nexus: SYNCED (72BPM)",
            "Omni-Fabric: 16Tb/s FLOW"
        ];
        
        metrics.forEach((m, i) => {
            setTimeout(() => {
                const entry = document.createElement('div');
                entry.className = "log-entry system pulse";
                entry.innerHTML = `<span class="timestamp">[LATTICE]</span> ${m}`;
                document.getElementById('audit-log').prepend(entry);
            }, i * 1500);
        });
    }
};

window.addEventListener('load', () => {
    PremiumVitals.init();
    setTimeout(() => PremiumVitals.visualizeLattice(), 3000);
});
