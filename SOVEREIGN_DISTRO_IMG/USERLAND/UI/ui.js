/**
 * Σ SIGMA OS UI ENGINE v3.0 [INFINITY CORE]
 * Interface & Windowing Manager
 * Featuring Workspaces, Snapping, and Neural HUD
 */

import { AppRegistry, AppLoader } from './app_registry.js';

export const UIEngine = {
    initialized: false,
    activeWindow: null,
    windowZIndex: 10,
    openApps: [],
    currentWS: 1,

    init() {
        if (this.initialized) return;
        this.initialized = true;
        this.setupLauncher();
        this.setupWindows();
        this.setupTaskbar();
        this.setupStageManager();
        this.startClock();
        this.updateStats();
        SigmaKernel.notify("UI_ENGINE: Logic stream synchronized for Infinity v3.0.", "success");
    },

    switchWS(num) {
        if (this.currentWS === num) return;
        this.currentWS = num;

        // Update HUD
        const btns = document.querySelectorAll('.ws-btn');
        btns.forEach(b => b.classList.toggle('active', parseInt(b.textContent) === num));

        // Logical Transition
        const windows = document.querySelectorAll('.window');
        windows.forEach(win => {
            const winWS = parseInt(win.dataset.ws) || 1;
            if (!win.classList.contains('display-none')) {
                win.style.opacity = winWS === num ? '1' : '0';
                win.style.pointerEvents = winWS === num ? 'auto' : 'none';
                setTimeout(() => {
                    win.classList.toggle('display-none', winWS !== num);
                }, 200);
            } else {
                win.classList.toggle('display-none', winWS !== num);
            }
        });

        SigmaKernel.notify(`Switching to Workspace ${num}`, "info");
    },

    setupLauncher() {
        const grid = document.getElementById('launcher-grid');
        if (!grid) return;

        grid.innerHTML = AppRegistry.map(app => `
            <div class="launcher-app" onclick="UIEngine.launch('${app.id}')">
                <div class="launcher-app-icon">${app.icon}</div>
                <div class="launcher-app-name">${app.name}</div>
            </div>
        `).join('');
    },

    setupWindows() {
        window.openWindow = (id) => this.launch(id);
        window.closeWindow = (id) => this.close(id);
        window.toggleMaximize = (id) => this.maximize(id);
        window.startDrag = (e, id) => this.drag(e, id);
        window.switchWS = (num) => this.switchWS(num);
        window.toggleLauncher = () => {
            const l = document.getElementById('launcher');
            if (l) l.classList.toggle('active');
        };
    },

    async launch(id) {
        if (id === 'terminal') {
            this.openWindow('terminal');
            return;
        }

        const app = AppRegistry.find(a => a.id === id);
        if (!app) return;

        console.log(`[UI] Initiating lazy-load for module '${id}'...`);
        await AppLoader.loadModule(id);

        if (!this.openApps.includes(id)) {
            this.openApps.push(id);
            this.updateTaskbar();
        }

        const win = document.getElementById(`win-${id}`);
        if (win) {
            win.dataset.ws = this.currentWS;
            this.openWindow(id);
        }

        const l = document.getElementById('launcher');
        if (l) l.classList.remove('active');
    },

    openWindow(id) {
        const win = document.getElementById(`win-${id}`);
        if (!win) return;

        const winWS = parseInt(win.dataset.ws) || this.currentWS;
        if (winWS !== this.currentWS) {
            this.switchWS(winWS);
        }

        win.classList.remove('display-none');
        win.style.opacity = '1';
        win.style.pointerEvents = 'auto';
        this.focus(id);
        this.updateStageManager();
        this.updateTaskbar();
        SigmaKernel.notify(`${id.toUpperCase()} active in Logic Workspace ${winWS}`, "info");
    },

    close(id) {
        const win = document.getElementById(`win-${id}`);
        if (win) {
            win.classList.add('display-none');
            win.style.opacity = '0';
        }
        this.openApps = this.openApps.filter(a => a !== id);
        this.updateTaskbar();
        this.updateStageManager();
    },

    maximize(id) {
        const win = document.getElementById(`win-${id}`);
        if (win) win.classList.toggle('maximized');
    },

    focus(id) {
        const win = document.getElementById(`win-${id}`);
        if (!win) return;
        this.windowZIndex++;
        win.style.zIndex = this.windowZIndex;
        this.activeWindow = id;
    },

    drag(e, id) {
        const win = document.getElementById(id);
        if (!win) return;
        this.focus(id.replace('win-', ''));

        if (win.classList.contains('snapped')) {
            win.classList.remove('snapped');
            win.style.width = win.dataset.prevW || '600px';
            win.style.height = win.dataset.prevH || '400px';
        }

        let posX = e.clientX;
        let posY = e.clientY;
        const visualizer = document.getElementById('snap-visualizer');

        const moveHandler = (me) => {
            const deltaX = me.clientX - posX;
            const deltaY = me.clientY - posY;
            posX = me.clientX;
            posY = me.clientY;

            win.style.top = (win.offsetTop + deltaY) + "px";
            win.style.left = (win.offsetLeft + deltaX) + "px";

            if (visualizer) {
                if (me.clientX < 20) this.showSnap(visualizer, 0, 0, '50%', '100%');
                else if (me.clientX > window.innerWidth - 20) this.showSnap(visualizer, '50%', 0, '50%', '100%');
                else if (me.clientY < 50) this.showSnap(visualizer, 0, 0, '100%', '50%');
                else visualizer.style.display = 'none';
            }
        };

        const stopHandler = () => {
            document.removeEventListener('mousemove', moveHandler);
            document.removeEventListener('mouseup', stopHandler);

            if (visualizer && visualizer.style.display !== 'none') {
                this.applySnap(win, visualizer);
                visualizer.style.display = 'none';
            }
        };

        document.addEventListener('mousemove', moveHandler);
        document.addEventListener('mouseup', stopHandler);
    },

    showSnap(el, l, t, w, h) {
        el.style.display = 'block';
        el.style.left = l === '50%' ? '50vw' : l;
        el.style.top = t;
        el.style.width = w === '50%' ? '50vw' : w;
        el.style.height = h === '50%' ? '50vh' : h;
    },

    applySnap(win, viz) {
        win.dataset.prevW = win.style.width;
        win.dataset.prevH = win.style.height;
        win.classList.add('snapped');
        win.style.left = viz.style.left;
        win.style.top = viz.style.top === '0px' ? '40px' : viz.style.top;
        win.style.width = viz.style.width;
        win.style.height = `calc(${viz.style.height} - 40px)`;
        SigmaKernel.notify(`Window snapped: Logical alignment complete.`, 'success');
    },

    setupTaskbar() {
        this.updateTaskbar();
    },

    updateTaskbar() {
        const tb = document.getElementById('taskbar');
        if (!tb) return;

        const fragment = document.createDocumentFragment();
        this.openApps.forEach(id => {
            const item = document.createElement('div');
            item.className = `taskbar-item ${this.activeWindow === id ? 'active' : ''}`;
            item.onclick = () => this.launch(id);
            item.innerHTML = this.getAppIcon(id);
            fragment.appendChild(item);
        });

        tb.innerHTML = '';
        tb.appendChild(fragment);
    },

    updateStageManager() {
        const sm = document.getElementById('stage-manager');
        if (!sm) return;

        const fragment = document.createDocumentFragment();
        this.openApps.forEach(id => {
            const item = document.createElement('div');
            item.className = 'stage-item';
            item.onclick = () => this.launch(id);
            item.innerHTML = `<div class="stage-preview">${this.getAppIcon(id)}</div>`;
            fragment.appendChild(item);
        });

        sm.innerHTML = '';
        sm.appendChild(fragment);
    },

    getAppIcon(id) {
        const app = AppRegistry.find(a => a.id === id);
        return app ? app.icon : '⚙️';
    },

    startClock() {
        const clock = document.getElementById('clock');
        const update = () => {
            const now = new Date();
            const timeStr = now.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
            if (clock.textContent !== timeStr) clock.textContent = timeStr;
        };
        update();
        setInterval(update, 1000);
    },

    updateStats() {
        setInterval(() => {
            requestAnimationFrame(() => {
                const cpu = (Math.random() * 15 + 2).toFixed(1);
                const ram = (Math.random() * 10 + 45).toFixed(1);
                const net = (Math.random() * 5 + 0.8).toFixed(1);

                const setStat = (id, val, suffix) => {
                    const el = document.getElementById(id);
                    if (el) el.textContent = val + suffix;
                };

                const setBar = (id, val) => {
                    const el = document.getElementById(id);
                    if (el) el.style.width = val + '%';
                };

                setStat('dash-cpu', cpu, '%');
                setStat('dash-ram', ram, '%');
                setStat('dash-net', net, ' MB/s');

                setBar('dash-cpu-bar', cpu);
                setBar('dash-ram-bar', ram);
                setBar('dash-net-bar', net);
            });
        }, 2500);
    }
};

window.UIEngine = UIEngine;
window.SystemUI = UIEngine;

window.updateDashPsTable = () => {
    const table = document.getElementById('dash-ps-table');
    if (!table) return;
    const processes = window.SigmaKernel ? window.SigmaKernel.processes : [];
    table.innerHTML = processes.map(p => `
        <tr class="border-bottom">
            <td class="p-5">${p.id}</td>
            <td class="p-5">${p.name}</td>
            <td class="p-5">${p.cpu}%</td>
            <td class="p-5">${p.mem}%</td>
            <td class="p-5"><button class="ag-btn-secondary p-2-5 font-10" onclick="SigmaKernel.notify('Access Denied: Task protected by Shield.', 'error')">KILL</button></td>
        </tr>
    `).join('');
};

window.filterLauncher = () => {
    const q = document.getElementById('launcher-search').value.toLowerCase();
    const items = document.querySelectorAll('.launcher-app');
    items.forEach(it => {
        const name = it.querySelector('.launcher-app-name').textContent.toLowerCase();
        it.style.display = name.includes(q) ? 'flex' : 'none';
    });
};
