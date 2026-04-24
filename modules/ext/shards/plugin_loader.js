/**
 * SigmaOS: Sovereign Plugin Loader
 * Inspired by AwesomeWM and Wayfire.
 * USP: Hot-swappable JS/WASM plugins for extending the Zenith Dashboard.
 */

const PluginLoader = {
    activePlugins: new Map(),

    async loadPlugin(pluginName, url) {
        console.log(`Σ://PLUGIN_LOAD> Fetching ${pluginName}...`);
        
        try {
            const script = document.createElement('script');
            script.src = url;
            script.onload = () => {
                this.activePlugins.set(pluginName, { status: 'active', url });
                UIUtils.appendLog('audit-log', `PLUGIN: ${pluginName} loaded successfully.`, 'success');
            };
            document.head.appendChild(script);
        } catch (err) {
            console.error(`Σ://PLUGIN_ERR> Failed to load ${pluginName}: ${err.message}`);
        }
    },

    unloadPlugin(pluginName) {
        // Plugin cleanup logic
        this.activePlugins.delete(pluginName);
    }
};

if (typeof window !== 'undefined') {
    window.SigmaPluginLoader = PluginLoader;
}
