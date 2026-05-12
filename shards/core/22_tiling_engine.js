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
        
        const w = 100 / cols;
        const h = 100 / rows;

        windows.forEach((win, i) => {
            const col = i % cols;
            const row = Math.floor(i / cols);
            win.style.left = `${col * w}%`;
            win.style.top = `${row * h}%`;
            win.style.width = `${w}%`;
            win.style.height = `${h}%`;
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
