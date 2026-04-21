/**
 * SigmaOS Sovereign Settings Engine
 * Module 01: Centralized configuration and lattice-wide preference sync.
 */

const SettingsEngine = {
    config: {
        animations: true,
        transparency: true,
        reducedMotion: false,
        autoUpdate: true
    },

    init() {
        console.log("Σ Settings Engine: Loading Sovereign Preferences...");
        this.applyAll();
    },

    update(key, value) {
        if (this.config.hasOwnProperty(key)) {
            this.config[key] = value;
            this.apply(key, value);
            UIUtils.appendLog('audit-log', `Settings: [${key}] set to ${value}`, 'success');
        }
    },

    apply(key, value) {
        const root = document.documentElement;
        if (key === 'transparency') {
            root.style.setProperty('--panel-alpha', value ? '0.7' : '1.0');
        }
        if (key === 'animations') {
            root.classList.toggle('disable-anims', !value);
        }
    },

    applyAll() {
        Object.keys(this.config).forEach(key => this.apply(key, this.config[key]));
    }
};

window.SettingsEngine = SettingsEngine;
