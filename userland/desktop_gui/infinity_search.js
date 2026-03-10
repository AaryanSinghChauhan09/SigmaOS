/**
 * Σ SIGMA OS INFINITY SEARCH v3.0
 * Universal Command Palette & Global Search
 */

import { AppRegistry } from './app_registry.js';

export const InfinitySearch = {
    active: false,
    selectedIndex: 0,
    results: [],

    init() {
        window.addEventListener('keydown', (e) => {
            if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
                e.preventDefault();
                this.toggle();
            }
            if (e.key === 'Escape' && this.active) {
                this.toggle();
            }
        });

        const input = document.getElementById('infinity-search-input');
        if (input) {
            input.addEventListener('input', (e) => this.search(e.target.value));
            input.addEventListener('keydown', (e) => this.handleKeydown(e));
        }
    },

    toggle() {
        this.active = !this.active;
        const el = document.getElementById('infinity-search');
        if (!el) return;

        if (this.active) {
            el.classList.add('active');
            const input = document.getElementById('infinity-search-input');
            input.value = '';
            input.focus();
            this.search('');
        } else {
            el.classList.remove('active');
        }
    },

    search(query) {
        const q = query.toLowerCase();
        this.results = [];

        // 1. Search Apps
        AppRegistry.forEach(app => {
            if (app.name.toLowerCase().includes(q) || app.id.toLowerCase().includes(q)) {
                this.results.push({
                    type: 'app',
                    id: app.id,
                    name: app.name,
                    icon: app.icon,
                    desc: app.isCore ? 'System Application' : 'Logical Module'
                });
            }
        });

        // 2. Search Shell Commands
        const shellCmds = ['help', 'clear', 'neofetch', 'ps', 'kill', 'top', 'ls', 'cd', 'cat'];
        shellCmds.forEach(cmd => {
            if (cmd.includes(q)) {
                this.results.push({
                    type: 'cmd',
                    id: cmd,
                    name: `Run: ${cmd}`,
                    icon: '🐚',
                    desc: 'TensorShell Command'
                });
            }
        });

        // 3. Search VFS (Simulated)
        if (window.SigmaKernel && q.length > 1) {
            const files = SigmaKernel.ls();
            files.forEach(f => {
                if (f.toLowerCase().includes(q)) {
                    this.results.push({
                        type: 'vfs',
                        id: f,
                        name: f,
                        icon: '📄',
                        desc: `File in ${SigmaKernel.currentPath}`
                    });
                }
            });
        }

        this.selectedIndex = 0;
        this.render();
    },

    handleKeydown(e) {
        if (e.key === 'ArrowDown') {
            e.preventDefault();
            this.selectedIndex = (this.selectedIndex + 1) % this.results.length;
            this.render();
        } else if (e.key === 'ArrowUp') {
            e.preventDefault();
            this.selectedIndex = (this.selectedIndex - 1 + this.results.length) % this.results.length;
            this.render();
        } else if (e.key === 'Enter') {
            e.preventDefault();
            this.execute(this.results[this.selectedIndex]);
        }
    },

    execute(res) {
        if (!res) return;

        if (res.type === 'app') {
            if (window.UIEngine) UIEngine.launch(res.id);
        } else if (res.type === 'cmd') {
            if (window.UIEngine) UIEngine.launch('terminal');
            // Suggest logic to pipe command to shell here
            SigmaKernel.notify(`Executing shell command: ${res.id}`, 'info');
        } else if (res.type === 'vfs') {
            SigmaKernel.notify(`Opening file: ${res.id}`, 'success');
        }

        this.toggle();
    },

    render() {
        const cont = document.getElementById('infinity-search-results');
        if (!cont) return;

        cont.innerHTML = this.results.slice(0, 8).map((res, i) => `
            <div class="search-res-item ${i === this.selectedIndex ? 'selected' : ''}" onclick="InfinitySearch.executeByIndex(${i})">
                <div class="search-res-icon">${res.icon}</div>
                <div class="search-res-meta">
                    <div class="search-res-name">${res.name}</div>
                    <div class="search-res-type">${res.desc}</div>
                </div>
            </div>
        `).join('');
    },

    executeByIndex(i) {
        this.execute(this.results[i]);
    }
};

window.InfinitySearch = InfinitySearch;
