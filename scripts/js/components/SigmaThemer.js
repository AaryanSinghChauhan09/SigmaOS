"use strict";

/**
 * Σ SIGMA THEMER
 * Specialized shard for industrial UI theme orchestration.
 */
export class SigmaThemer {
    constructor(system) {
        this.system = system;
        this.THEMES = {
            'ZENITH': { accent: '#00d2ff', bg: '#0f0f14' },
            'KALI': { accent: '#33ff00', bg: '#000000' },
            'UBUNTU': { accent: '#dd4814', bg: '#221f1f' },
            'NORD': { accent: '#88c0d0', bg: '#2e3440' },
            'DRACULA': { accent: '#ff79c6', bg: '#282a36' }
        };
    }

    apply(mode) {
        document.body.className = `mode-${mode.toLowerCase()}`;
        localStorage.setItem('sigma-theme', mode);
        const config = this.THEMES[mode];
        if (config) {
            document.documentElement.style.setProperty('--accent-primary', config.accent);
            this.system.spawnToast(`System Mode Switched: ${mode}`);
        }
    }

    loadInitial() {
        const theme = localStorage.getItem('sigma-theme') || 'ZENITH';
        this.apply(theme);
    }
}
