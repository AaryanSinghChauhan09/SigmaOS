/**
 * Σ SIGMA OS APP REGISTRY v2.2
 * Centralized Application Management & Lazy Loading Coordinator
 * Improved Modularization, Scalability & Performance
 */

export const AppRegistry = [
    { id: 'dashboard', name: 'Dashboard', icon: '📊', isCore: true, status: 'idle' },
    { id: 'terminal', name: 'TensorShell', icon: '💻', isCore: true, status: 'idle' },
    { id: 'store', name: 'Store', icon: '📦', isCore: true, status: 'idle' },
    { id: 'text_cleaner', name: 'Text Cleaner', icon: '🧹', module: './apps.js', status: 'idle' },
    { id: 'automation', name: 'Automation', icon: '⚡', module: './apps.js', status: 'idle' },
    { id: 'vfs', name: 'Filesystem', icon: '📁', module: './apps.js', status: 'idle' },
    { id: 'collab', name: 'Mesh Collab', icon: '🤝', module: './mesh.js', status: 'idle' },
    { id: 'antigrav', name: 'Antigravity', icon: '🚀', module: './apps.js', status: 'idle' },
    { id: 'settings', name: 'Settings', icon: '⚙️', isCore: true, status: 'idle' },
    { id: 'pdf', name: 'PDF Forge', icon: '📄', module: './apps.js', status: 'idle' },
    { id: 'notes', name: 'Notes', icon: '📝', module: './apps.js', status: 'idle' },
    { id: 'paint', name: 'Neural Paint', icon: '🎨', module: './games.js', status: 'idle' },
    { id: 'code', name: 'Code Lab', icon: '💻', module: './apps.js', status: 'idle' },
    { id: 'pong', name: 'Pong AI', icon: '🏓', module: './games.js', status: 'idle' },
    { id: 'chess', name: 'Chess AI', icon: '♟️', module: './games.js', status: 'idle' },
    { id: 'calc', name: 'Sovereign Calc', icon: '🧮', module: './apps.js', status: 'idle' },
    { id: 'synth', name: 'Sovereign Synth', icon: '🎹', module: './apps.js', status: 'idle' },
    { id: 'analyzer', name: 'Logic Analyzer', icon: '🔍', module: './apps.js', status: 'idle' },
    { id: 'vault', name: 'Sovereign Vault', icon: '🔐', module: './apps.js', status: 'idle' },
    { id: 'type', name: 'Sovereign Type', icon: '⌨️', module: './apps.js', status: 'idle' },
    { id: 'email', name: 'Sovereign Email', icon: '📧', isCore: true, status: 'idle' },
    { id: 'excel', name: 'Sovereign Excel', icon: '📊', isCore: true, status: 'idle' }
];

export const AppLoader = {
    loadedModules: new Set(),

    async loadModule(id) {
        const app = AppRegistry.find(a => a.id === id);
        if (!app || app.isCore || !app.module) return;

        if (this.loadedModules.has(app.module)) {
            app.status = 'active';
            return;
        }

        app.status = 'loading';
        console.log(`[LOADER] Lazy loading system module: ${app.module} for ${app.name}...`);
        try {
            await import(app.module);
            this.loadedModules.add(app.module);
            app.status = 'active';
            console.log(`[LOADER] ${app.module} successfully synchronized for ${app.name}.`);
        } catch (e) {
            app.status = 'error';
            console.error(`[LOADER] FAILED to load ${app.module}:`, e);
        }
    },

    registerApp(appConfig) {
        AppRegistry.push({ ...appConfig, status: 'idle' });
        if (window.UIEngine) window.UIEngine.setupLauncher();
    }
};

window.AppRegistry = AppRegistry;
window.AppLoader = AppLoader;
