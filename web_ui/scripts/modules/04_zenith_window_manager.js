/**
 * SigmaOS Sovereign Window Manager (v2.0)
 * Module 04: High-fidelity window orchestration, minimize/maximize, bounds-clamping & tile layouts.
 *
 * Architecture Improvements over v1.1:
 *  - Bounds-clamping prevents windows from being dragged off-screen.
 *  - Functional minimize (collapse to taskbar) and maximize (fullscreen) toggles.
 *  - Window registry: tracks all open windows by title+pid.
 *  - EventBus integration for process lifecycle hooks.
 *  - Resizing via corner handle.
 */

document.addEventListener('DOMContentLoaded', () => {
    let zIndexCounter = 1000;
    const windowRegistry = new Map(); // title → { el, pid }

    /**
     * Creates and renders a new sovereign window.
     */
    window.createWindow = (title, content, options = {}) => {
        // Prevent duplicate windows
        if (windowRegistry.has(title)) {
            const existing = windowRegistry.get(title);
            existing.el.style.zIndex = ++zIndexCounter;
            return existing.el;
        }

        const win = document.createElement('div');
        win.className = 'sovereign-window glass-panel';
        win.style.cssText = `
            width: ${options.width || '620px'};
            height: ${options.height || '420px'};
            top: ${options.top || (80 + windowRegistry.size * 30) + 'px'};
            left: ${options.left || (80 + windowRegistry.size * 30) + 'px'};
            z-index: ${++zIndexCounter};
            position: fixed;
        `;

        win.innerHTML = `
            <div class="window-header">
                <div class="window-title">
                    <span class="window-icon">${options.icon || '💠'}</span>
                    <span>${title}</span>
                </div>
                <div class="window-controls">
                    <span class="w-tile" title="Tile Windows">⊞</span>
                    <span class="w-min" title="Minimize">—</span>
                    <span class="w-max" title="Maximize">▢</span>
                    <span class="w-close" title="Close">✕</span>
                </div>
            </div>
            <div class="window-body" style="height: calc(100% - 44px); overflow: auto; padding: 12px; box-sizing: border-box;">
                ${content}
            </div>
            <div class="window-resizer"></div>
        `;

        document.body.appendChild(win);

        // Register PID
        const pid = window.ProcessManager
            ? ProcessManager.registerShard('S02_ZenithUI', title, 'NORMAL')
            : null;
        windowRegistry.set(title, { el: win, pid });

        // Controls
        win.querySelector('.w-close').onclick = () => {
            if (pid && window.ProcessManager) ProcessManager.neutralize(pid);
            windowRegistry.delete(title);
            win.remove();
        };

        let maximized = false;
        let savedCSS = {};
        win.querySelector('.w-max').onclick = () => {
            if (!maximized) {
                savedCSS = { width: win.style.width, height: win.style.height, top: win.style.top, left: win.style.left };
                Object.assign(win.style, { width: '100vw', height: '100vh', top: '0', left: '0' });
                maximized = true;
            } else {
                Object.assign(win.style, savedCSS);
                maximized = false;
            }
        };

        win.querySelector('.w-min').onclick = () => {
            win.style.display = (win.style.display === 'none') ? '' : 'none';
        };

        win.querySelector('.w-tile').onclick = () => {
            tileAllWindows();
        };

        _makeDraggable(win, zIndexCounter);
        _makeResizable(win);

        UIUtils.appendLog('audit-log', `Window: [${title}] manifested (PID=${pid})`, 'success');
        if (window.EventBus) EventBus.publish('window_opened', { title, pid });
        return win;
    };

    function tileAllWindows() {
        const wins = [...windowRegistry.values()].filter(w => w.el.style.display !== 'none');
        const count = wins.length;
        if (count === 0) return;
        const cols = Math.ceil(Math.sqrt(count));
        const rows = Math.ceil(count / cols);
        const W = Math.floor(window.innerWidth / cols);
        const H = Math.floor(window.innerHeight / rows);
        wins.forEach(({ el }, i) => {
            const col = i % cols, row = Math.floor(i / cols);
            Object.assign(el.style, { width: W + 'px', height: H + 'px', top: (row * H) + 'px', left: (col * W) + 'px' });
        });
    }

    function _makeDraggable(el) {
        const header = el.querySelector('.window-header');
        let ox = 0, oy = 0, sx = 0, sy = 0;

        header.addEventListener('mousedown', e => {
            if (e.target.closest('.window-controls')) return;
            e.preventDefault();
            el.style.zIndex = ++zIndexCounter;
            sx = e.clientX; sy = e.clientY;

            const onMove = e => {
                ox = sx - e.clientX; oy = sy - e.clientY;
                sx = e.clientX; sy = e.clientY;

                // Bounds-clamping
                const newTop = Math.max(0, Math.min(window.innerHeight - 60, el.offsetTop - oy));
                const newLeft = Math.max(0, Math.min(window.innerWidth - 100, el.offsetLeft - ox));
                el.style.top = newTop + 'px';
                el.style.left = newLeft + 'px';
            };
            const onUp = () => {
                document.removeEventListener('mousemove', onMove);
                document.removeEventListener('mouseup', onUp);
            };
            document.addEventListener('mousemove', onMove);
            document.addEventListener('mouseup', onUp);
        });
    }

    function _makeResizable(el) {
        const resizer = el.querySelector('.window-resizer');
        if (!resizer) return;
        resizer.addEventListener('mousedown', e => {
            e.preventDefault();
            const startW = parseInt(el.style.width), startH = parseInt(el.style.height);
            const startX = e.clientX, startY = e.clientY;
            const onMove = e => {
                el.style.width = Math.max(300, startW + e.clientX - startX) + 'px';
                el.style.height = Math.max(200, startH + e.clientY - startY) + 'px';
            };
            const onUp = () => {
                document.removeEventListener('mousemove', onMove);
                document.removeEventListener('mouseup', onUp);
            };
            document.addEventListener('mousemove', onMove);
            document.addEventListener('mouseup', onUp);
        });
    }

    window.ZenithWindowManager = { registry: windowRegistry, tileAllWindows };
});
