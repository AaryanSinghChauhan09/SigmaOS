/**
 * Sovereign Settings Engine (v1.0)
 * Centralized control for OS personalization and security.
 * Integrated with the CLI for 100% parity.
 */

class SovereignSettings extends ZenithComponent {
    constructor() {
        super('settings-view');
        this.config = {
            blur: 25,
            opacity: 0.6,
            accent: '#00f0ff',
            fontScale: 1.0,
            privacyShield: true,
            heartbeat: 2000
        };
        this.init();
    }

    init() {
        this.loadSettings();
        this.applyAll();
        console.log('Σ://UI> Settings Engine Synchronized.');
    }

    set(key, value) {
        if (this.config.hasOwnProperty(key)) {
            this.config[key] = value;
            this.apply(key, value);
            this.saveSettings();
            window.zenith.taskbar.notify(`SETTING UPDATED: ${key}=${value}`, 'OPTIMAL');
        } else {
            console.warn(`Σ://UI> Invalid Setting: ${key}`);
        }
    }

    apply(key, value) {
        const root = document.documentElement;
        switch(key) {
            case 'blur': root.style.setProperty('--mica-blur', `${value}px`); break;
            case 'opacity': root.style.setProperty('--mica-opacity', value); break;
            case 'accent': root.style.setProperty('--acc-cyan', value); break;
            case 'fontScale': root.style.setProperty('font-size', `${value * 16}px`); break;
            case 'privacyShield': Sigma.guard(); break; // Re-fire guard
        }
    }

    applyAll() {
        for (let key in this.config) this.apply(key, this.config[key]);
    }

    saveSettings() {
        localStorage.setItem('sigma_settings', JSON.stringify(this.config));
    }

    loadSettings() {
        const saved = localStorage.getItem('sigma_settings');
        if (saved) this.config = { ...this.config, ...JSON.parse(saved) };
    }
}

window.SovereignSettings = SovereignSettings;
