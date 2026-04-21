/**
 * SigmaOS Zenith Premium Vitals
 * Module 05: Dynamic Dock & Immersive Effects
 */

const PremiumVitals = {
    init() {
        console.log("Σ Zenith Premium Vitals Initializing...");
        this.setupDock();
    },

    setupDock() {
        const dock = document.getElementById('sovereign-dock');
        
        // Show dock when GUI is active
        const observer = new MutationObserver((mutations) => {
            const guiView = document.getElementById('gui-view');
            if (guiView && !guiView.classList.contains('hidden')) {
                dock.classList.add('visible');
            } else if (dock) {
                dock.classList.remove('visible');
            }
        });

        const guiView = document.getElementById('gui-view');
        if (guiView) {
            observer.observe(guiView, { attributes: true, attributeFilter: ['class'] });
        }

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
            const btnCli = document.getElementById('btn-cli');
            if (btnCli) btnCli.click();
        }
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
                const log = document.getElementById('audit-log');
                if (!log) return;
                const entry = document.createElement('div');
                entry.className = "log-entry system pulse";
                entry.innerHTML = `<span class="timestamp">[LATTICE]</span> ${m}`;
                log.prepend(entry);
            }, i * 1500);
        });
    }
};

window.addEventListener('load', () => {
    // Stage 1: Core Utilities
    if (window.AudioEngine) AudioEngine.init();
    if (window.CanvasAnims) CanvasAnims.init();
    
    // Stage 2: Functional Engines
    if (window.EnvEngine) EnvEngine.init();
    if (window.ThemeEngine) ThemeEngine.init();
    if (window.Notifications) Notifications.init();
    if (window.Localization) Localization.init();
    
    // Stage 3: Management Layers
    if (window.TaskManager) TaskManager.init();
    if (window.MissionControl) MissionControl.init();
    if (window.RecoveryHub) RecoveryHub.init();
    if (window.ShardOrchestrator) ShardOrchestrator.init();
    
    // Stage 4: User Interface & Experience
    if (window.DashboardOrchestrator) DashboardOrchestrator.init();
    if (window.ZenithDesktop) ZenithDesktop.init();
    if (window.NeuralInterface) NeuralInterface.init();
    if (window.Accessibility) Accessibility.init();
    
    // Stage 5: Vitals & Analytics
    if (window.AnalyticsEngine) AnalyticsEngine.init();
    if (window.VitalsEngine) VitalsEngine.init();
    if (window.PremiumVitals) PremiumVitals.init();
    
    setTimeout(() => PremiumVitals.visualizeLattice(), 3000);
});
