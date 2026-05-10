/**
 * Σ Sovereign Config Bridge (v1.0)
 * ─────────────────────────────────────────────────────────────────────────────
 * Loads sigma_config.json from the server and syncs it with:
 *   1. localStorage (for offline persistence)
 *   2. SovereignSettings engine (live GUI application)
 *   3. CLI (sigma_config.json is the single source of truth)
 *
 * This ensures GUI and CLI always share the same configuration state.
 */

class SigmaConfigBridge {
    constructor() {
        this.config   = {};
        this.POLL_MS  = 10000; // Poll server config every 10s
        this.init();
    }

    async init() {
        await this.loadFromServer();
        this.applyToGUI();
        this.startPolling();
        console.log('Σ://CONFIG> Config Bridge Active. GUI ↔ CLI synced.');
    }

    async loadFromServer() {
        try {
            const res = await fetch('/api/config');
            if (res.ok) {
                this.config = await res.json();
                // Merge with localStorage (server wins)
                const local = JSON.parse(localStorage.getItem('sigma_settings') || '{}');
                this.config = { ...local, ...this.config };
                localStorage.setItem('sigma_settings', JSON.stringify(this.config));
            }
        } catch {
            // Server offline — fall back to localStorage
            const saved = localStorage.getItem('sigma_settings');
            this.config = saved ? JSON.parse(saved) : {};
        }
    }

    applyToGUI() {
        if (!this.config || !window.settings) return;
        const map = {
            theme:       v => window.theme?.applyTheme(v),
            accent:      v => document.documentElement.style.setProperty('--acc-cyan', v),
            blur:        v => document.documentElement.style.setProperty('--mica-blur', `${v}px`),
            opacity:     v => document.documentElement.style.setProperty('--mica-opacity', v),
            fontScale:   v => document.documentElement.style.setProperty('font-size', `${v * 16}px`),
            mode:        v => window.settings?.apply('mode', v),
        };
        Object.entries(this.config).forEach(([k, v]) => {
            if (map[k]) { try { map[k](v); } catch(e) {} }
        });
    }

    set(key, value) {
        this.config[key] = value;
        localStorage.setItem('sigma_settings', JSON.stringify(this.config));
        this.applyToGUI();
        // Persist back to server
        fetch('/api/config', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ key, value })
        }).catch(() => {});
    }

    get(key) {
        return key ? this.config[key] : this.config;
    }

    startPolling() {
        setInterval(async () => {
            await this.loadFromServer();
            this.applyToGUI();
        }, this.POLL_MS);
    }
}

window.SigmaConfigBridge = SigmaConfigBridge;
window.sigmaConfig = new SigmaConfigBridge();
