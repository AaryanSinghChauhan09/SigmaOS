/**
 * SigmaOS Sovereign Layout Engine
 * Module 04: Zenith Window orchestration and auto-tiling.
 */

const SovereignLayout = {
    tileWindows() {
        const windows = Array.from(document.querySelectorAll('.sovereign-window'));
        if (windows.length === 0) return;

        console.log("Σ Layout Engine: Tiling Lattice Windows...");
        
        const rows = Math.ceil(Math.sqrt(windows.length));
        const cols = Math.ceil(windows.length / rows);
        
        const w = window.innerWidth / cols;
        const h = (window.innerHeight - 60) / rows; // Adjust for dock/header

        windows.forEach((win, i) => {
            const r = Math.floor(i / cols);
            const c = i % cols;
            
            win.style.width = (w - 10) + 'px';
            win.style.height = (h - 10) + 'px';
            win.style.top = (r * h + 15) + 'px';
            win.style.left = (c * w + 5) + 'px';
            
            UIUtils.pulseElement(win, '0 0 20px var(--acc-gold)');
        });
        
        UIUtils.appendLog('audit-log', `Lattice: Tiled ${windows.length} windows.`, 'success');
    },

    cascadeWindows() {
        const windows = Array.from(document.querySelectorAll('.sovereign-window'));
        windows.forEach((win, i) => {
            win.style.top = (50 + (i * 30)) + 'px';
            win.style.left = (150 + (i * 30)) + 'px';
            win.style.width = '600px';
            win.style.height = '400px';
        });
        UIUtils.appendLog('audit-log', `Lattice: Cascaded ${windows.length} windows.`, 'normal');
    }
};

window.SovereignLayout = SovereignLayout;
