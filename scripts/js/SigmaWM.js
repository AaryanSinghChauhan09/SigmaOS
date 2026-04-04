"use strict";

/**
 * Σ WINDOW MANAGER (SigmaWM)
 * Handles workspace orchestration, taskbar, and industrial UI logic.
 */
export class SigmaWM {
    constructor(system) {
        this.system = system;
        this.zIndex = 1000;
        this.activeWorkspace = 1;
        this.workspaces = {}; // winId -> wsNum
        this.init();
    }

    init() {
        // Window dragging and control events
        document.addEventListener('mousedown', (e) => {
            const header = e.target.closest('.win-header');
            if (header) {
                const win = header.parentElement;
                this.focus(win.id.replace('win-', ''));
                this.dragWindow(win, e);
            }
        });

        // Global actions for window buttons
        document.addEventListener('click', (e) => {
            const btn = e.target.closest('.win-btn');
            if (btn) {
                const action = btn.getAttribute('data-action');
                const winId = btn.getAttribute('data-win');
                if (action === 'close') this.close(winId);
                if (action === 'minimize') this.minimize(winId);
                if (action === 'maximize') this.maximize(winId);
            }
        });

        // Dock items
        document.querySelectorAll('.dock-item').forEach(item => {
            item.onclick = () => this.open(item.getAttribute('data-window'));
        });

        // Workspace indicators
        document.querySelectorAll('.ws-indicator').forEach(ws => {
            ws.onclick = () => this.switchWorkspace(parseInt(ws.getAttribute('data-ws')));
        });
    }

    dragWindow(win, e) {
        let offsetX = e.clientX - win.offsetLeft;
        let offsetY = e.clientY - win.offsetTop;
        const SNAP_GRID = 20; // pixels
        const onMove = (ev) => {
            let left = ev.clientX - offsetX;
            let top = ev.clientY - offsetY;
            
            // Snap to grid
            left = Math.round(left / SNAP_GRID) * SNAP_GRID;
            top = Math.round(top / SNAP_GRID) * SNAP_GRID;
            
            win.style.left = left + 'px';
            win.style.top = top + 'px';
        };
        const onUp = () => {
            document.removeEventListener('mousemove', onMove);
            document.removeEventListener('mouseup', onUp);
        };
        document.addEventListener('mousemove', onMove);
        document.addEventListener('mouseup', onUp);
    }

    open(id) {
        const win = document.getElementById('win-' + id);
        if (!win) return;
        win.classList.remove('hidden');
        win.classList.remove('minimized');
        this.workspaces[id] = this.activeWorkspace;
        this.focus(id);
        this.updateTaskbar();
    }

    close(id) {
        const win = document.getElementById('win-' + id);
        if (!win) return;
        win.classList.add('hidden');
        this.updateTaskbar();
        this.system.spawnToast(`Shard [${id}] Terminated.`);
    }

    minimize(id) {
        const win = document.getElementById('win-' + id);
        if (!win) return;
        win.classList.add('hidden');
        this.updateTaskbar();
        this.system.spawnToast(`Shard [${id}] Suspended.`);
    }

    maximize(id) {
        const win = document.getElementById('win-' + id);
        if (!win) return;
        win.classList.toggle('maximized');
    }

    focus(id) {
        const win = document.getElementById('win-' + id);
        if (!win) return;
        document.querySelectorAll('.window').forEach(w => w.classList.remove('focused'));
        win.classList.add('focused');
        win.style.zIndex = ++this.zIndex;
    }

    switchWorkspace(ws) {
        this.activeWorkspace = ws;
        document.querySelectorAll('.ws-indicator').forEach(el => {
            el.classList.toggle('active', parseInt(el.getAttribute('data-ws')) === ws);
        });
        this.render();
        this.system.spawnToast(`Switched to Workspace ${ws}`);
    }

    render() {
        document.querySelectorAll('.window').forEach(win => {
            const id = win.id.replace('win-', '');
            const winWS = this.workspaces[id] || 1;
            if (winWS === this.activeWorkspace && !win.classList.contains('hidden')) {
                win.style.display = 'flex';
            } else {
                win.style.display = 'none';
            }
        });
        this.updateTaskbar();
    }

    updateTaskbar() {
        const taskbar = document.getElementById('taskbar');
        if (!taskbar) return;
        taskbar.innerHTML = '';
        
        document.querySelectorAll('.window').forEach(win => {
            if (win.classList.contains('hidden')) return;
            const id = win.id.replace('win-', '');
            const title = win.querySelector('.win-title')?.innerText || id;
            
            const item = document.createElement('div');
            item.className = `top-item status-chip ${win.classList.contains('focused') ? 'active-chip' : ''}`;
            item.innerHTML = `<span>${title.split(' ')[0]}</span>`;
            item.onclick = () => {
                const ws = this.workspaces[id] || 1;
                if (ws !== this.activeWorkspace) this.switchWorkspace(ws);
                this.focus(id);
            };
            taskbar.appendChild(item);
        });
    }

    tile() {
        const windows = Array.from(document.querySelectorAll('.window:not(.hidden)'));
        if (windows.length === 0) return;
        const width = 100 / windows.length;
        windows.forEach((win, i) => {
            win.classList.remove('maximized');
            win.style.width = `calc(${width}% - 10px)`;
            win.style.height = 'calc(100vh - 60px)';
            win.style.left = `${i * width}%`;
            win.style.top = '40px';
        });
    }
}
