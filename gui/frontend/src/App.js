/**
 * gui/frontend/src/App.js — Zenith GUI Application Root
 * Assembles all components into the Zenith dashboard.
 * Pure vanilla JS — zero framework dependency.
 */
import { ShardManager } from './components/ShardManager.js';
import { SyncPanel }    from './components/SyncPanel.js';
import { Profiles }     from './components/Profiles.js';
import { Status }       from './components/Status.js';

import { useShortcuts } from './hooks/useShortcuts.js';
import { build, sync, getStatus } from './api.js';

class ZenithApp {
    constructor() {
        this.components = {};
        this.activePanel = null;
        this.currentProfile = null;
    }

    async init() {
        this._injectTabs();
        this._mountComponents();
        await this._loadCurrentProfile();
        console.log('Σ://ZENITH_APP> All GUI components mounted.');
    }

    async _loadCurrentProfile() {
        // Fetch current status to determine active profile
        const r = await getStatus();
        if (r.ok) {
            const lines = r.data.split('\n');
            for (const line of lines) {
                if (line.startsWith('Profile:')) {
                    this.currentProfile = line.split(':')[1].trim();
                    break;
                }
            }
        }
        
        // Setup shortcuts based on profile (stub — would fetch full config in real app)
        const shortcuts = {
            'build': 'Ctrl+B',
            'sync': 'Ctrl+S',
            'status': 'Ctrl+T'
        };

        const actions = {
            'build': async () => {
                window.sigmaNotify?.('Initiating build...', 'INFO');
                const r = await build();
                window.sigmaNotify?.(r.data, r.ok ? 'OPTIMAL' : 'WARN');
            },
            'sync': async () => {
                window.sigmaNotify?.('Syncing with GitHub...', 'INFO');
                const r = await sync();
                window.sigmaNotify?.(r.data, r.ok ? 'OPTIMAL' : 'WARN');
            },
            'status': async () => {
                this._activateTab('panel-status', document.querySelector('[data-tab="panel-status"]'));
                this.components.status.refresh();
            }
        };

        this.cleanupShortcuts = useShortcuts(shortcuts, actions);
    }

    _injectTabs() {
        const tabBar = document.querySelector('.panel-tabs');
        if (!tabBar) return;

        const tabs = [
            { id: 'panel-shards',   label: '⬡ SHARDS'   },
            { id: 'panel-sync',     label: '↑ SYNC'      },
            { id: 'panel-profiles', label: '◉ PROFILES'  },
            { id: 'panel-status',   label: '⚡ STATUS'    },
        ];

        tabs.forEach(({ id, label }) => {
            // Mount point div
            if (!document.getElementById(id)) {
                const div = document.createElement('div');
                div.className = 'tab-content';
                div.id = id;
                document.querySelector('.left-wing')?.appendChild(div);
            }

            // Tab button (only if not already present)
            if (!document.querySelector(`[data-tab="${id}"]`)) {
                const btn = document.createElement('button');
                btn.className = 'tab-btn';
                btn.dataset.tab = id;
                btn.textContent = label;
                tabBar.appendChild(btn);
                btn.addEventListener('click', () => this._activateTab(id, btn));
            }
        });
    }

    _activateTab(panelId, btn) {
        document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
        document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
        btn?.classList.add('active');
        document.getElementById(panelId)?.classList.add('active');
        this.activePanel = panelId;
    }

    _mountComponents() {
        this.components.shards   = ShardManager('panel-shards');
        this.components.sync     = SyncPanel('panel-sync');
        this.components.profiles = Profiles('panel-profiles');
        this.components.status   = Status('panel-status');
    }
}

// Global notification helper (wraps existing Zenith taskbar)
window.sigmaNotify = (msg, type = 'INFO') => {
    if (window.zenith?.taskbar?.notify) {
        window.zenith.taskbar.notify(msg, type);
    } else {
        console.log(`[${type}] ${msg}`);
    }
};

export const App = new ZenithApp();
App.init();
