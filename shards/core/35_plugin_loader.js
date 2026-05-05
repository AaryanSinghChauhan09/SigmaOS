/**
 * Σ SigmaOS Plugin Loader (v1.0)
 * ─────────────────────────────────────────────────────────────────────────────
 * Dynamically discovers and loads plugins from the /plugins/ directory.
 * Each plugin must expose a `SigmaPlugin` object:
 *
 *   window.SigmaPlugin = {
 *     name: "my-plugin",
 *     version: "1.0.0",
 *     mount(container) { ... },  // Called when plugin tab is created
 *     unmount() { ... }           // Called on plugin removal
 *   };
 *
 * Plugins are registered into the Sovereign Event Bus and can publish/subscribe
 * to lattice events without touching core shard code.
 */

class SigmaPluginLoader {
    constructor() {
        this.registry = new Map();   // name → { meta, instance }
        this.tabContainer = null;
        this.contentContainer = null;
    }

    init() {
        this.tabContainer     = document.querySelector('.panel-tabs');
        this.contentContainer = document.querySelector('.left-wing');
        this._loadFromConfig();
        console.log('Σ://PLUGIN> Plugin Loader Initialized.');
    }

    /** Load plugin manifest from sigma_config.json via API */
    _loadFromConfig() {
        fetch('/api/plugins')
            .then(r => r.json())
            .then(plugins => plugins.forEach(p => this.load(p)))
            .catch(() => console.info('Σ://PLUGIN> No plugin API — running standalone.'));
    }

    /**
     * Dynamically inject a plugin JS file into the page.
     * @param {Object} meta - { name, entry, enabled }
     */
    load(meta) {
        if (!meta.enabled) return;
        if (this.registry.has(meta.name)) {
            console.warn(`Σ://PLUGIN> '${meta.name}' already loaded.`); return;
        }

        const script = document.createElement('script');
        script.src = meta.entry;
        if (meta.sri) script.integrity = meta.sri;
        script.crossOrigin = "anonymous";
        
        const timeout = setTimeout(() => {
            console.error(`Σ://PLUGIN> Timeout loading ${meta.name}`);
            script.remove();
        }, 10000);

        script.onload = () => {
            clearTimeout(timeout);
            if (window.SigmaPlugin) {
                this._mount(meta, window.SigmaPlugin);
                window.SigmaPlugin = undefined; 
            }
        };
        script.onerror = () => {
            clearTimeout(timeout);
            console.error(`Σ://PLUGIN> Failed to load: ${meta.entry}`);
        };
        document.body.appendChild(script);
    }

    _mount(meta, instance) {
        this.registry.set(meta.name, { meta, instance });

        // Create tab button
        if (this.tabContainer) {
            const btn = document.createElement('button');
            btn.className = 'tab-btn tab-plugin';
            btn.dataset.tab = `plugin-${meta.name}`;
            btn.textContent = (meta.icon || '🔌') + ' ' + meta.name.toUpperCase();
            btn.title = `Plugin: ${meta.name} v${meta.version || '?'}`;
            this.tabContainer.appendChild(btn);

            btn.addEventListener('click', () => this._activate(meta.name));
        }

        // Create content pane
        if (this.contentContainer) {
            const pane = document.createElement('div');
            pane.className = 'tab-content';
            pane.id = `plugin-${meta.name}`;
            pane.innerHTML = `<div class="plugin-mount-target"></div>`;
            this.contentContainer.appendChild(pane);

            instance.mount(pane.querySelector('.plugin-mount-target'));
        }

        // Publish to Event Bus
        if (window.SigmaEventBus) {
            window.SigmaEventBus.publish('plugin:loaded', { name: meta.name });
        }

        console.log(`Σ://PLUGIN> Mounted: ${meta.name} v${meta.version || '?'}`);
    }

    _activate(name) {
        // Reuse the tab system
        document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
        document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
        const btn  = document.querySelector(`[data-tab="plugin-${name}"]`);
        const pane = document.getElementById(`plugin-${name}`);
        if (btn)  btn.classList.add('active');
        if (pane) pane.classList.add('active');
    }

    unload(name) {
        const entry = this.registry.get(name);
        if (!entry) return;
        
        // Lifecycle: Unmount
        entry.instance.unmount?.();
        
        // DOM Cleanup
        document.getElementById(`plugin-${name}`)?.remove();
        document.querySelector(`[data-tab="plugin-${name}"]`)?.remove();
        
        // Memory Cleanup
        this.registry.delete(name);
        console.log(`Σ://PLUGIN> Unloaded: ${name}`);
    }

    list() {
        return Array.from(this.registry.values()).map(e => e.meta);
    }
}

window.SigmaPluginLoader = SigmaPluginLoader;
window.pluginLoader = new SigmaPluginLoader();
