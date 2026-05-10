/**
 * Sovereign Tiling Engine (v1.0)
 * Competitor USP: Advanced Tiling Window Manager (i3/Sway style).
 * Automatically organizes open windows into non-overlapping grids.
 */

class TilingEngine extends ZenithComponent {
    constructor() {
        super('gui-view');
        this.isTiling = false;
        this.init();
    }

    init() {
        console.log('Σ://UI> Tiling Engine Synced. Press Alt+T to toggle.');
    }

    toggle() {
        this.isTiling = !this.isTiling;
        if (this.isTiling) {
            this.applyTiling();
            window.zenith.taskbar.notify('TILING MODE: ACTIVE (GRID)', 'STABLE');
        } else {
            this.resetTiling();
            window.zenith.taskbar.notify('TILING MODE: DISABLED (FLOATING)', 'STABLE');
        }
    }

    applyTiling() {
        const windows = document.querySelectorAll('.window-container');
        if (windows.length === 0) return;

        const cols = Math.ceil(Math.sqrt(windows.length));
        const rows = Math.ceil(windows.length / cols);
        
        const container = document.querySelector('.desktop');
        const cWidth = container?.clientWidth || window.innerWidth;
        const cHeight = container?.clientHeight || window.innerHeight;
        
        const w = Math.floor(cWidth / cols);
        const h = Math.floor(cHeight / rows);
        const minW = 200; // Minimum window width
        const minH = 150; // Minimum window height

        windows.forEach((win, i) => {
            const col = i % cols;
            const row = Math.floor(i / cols);
            win.style.left = `${col * w}px`;
            win.style.top = `${row * h}px`;
            win.style.width = `${Math.max(w - 5, minW)}px`;
            win.style.height = `${Math.max(h - 5, minH)}px`;
            win.classList.add('tiling-active');
        });
    }

    resetTiling() {
        const windows = document.querySelectorAll('.window-container');
        windows.forEach(win => {
            win.style.left = '';
            win.style.top = '';
            win.style.width = '';
            win.style.height = '';
            win.classList.remove('tiling-active');
        });
    }
}

window.TilingEngine = TilingEngine;
