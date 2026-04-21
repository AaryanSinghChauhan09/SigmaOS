/**
 * SigmaOS Sovereign Theme Engine
 * Module 00: Dynamic silicate styling and aesthetic orchestration.
 */

const ThemeEngine = {
    themes: {
        'zenith': { primary: '#00ffea', secondary: '#7000ff', accent: '#ff00ff' },
        'matrix': { primary: '#00ff41', secondary: '#003b00', accent: '#008f11' },
        'amber':  { primary: '#ffc107', secondary: '#ff8f00', accent: '#ffe082' }
    },

    setTheme(name) {
        const theme = this.themes[name];
        if (!theme) return;

        console.log(`Σ Theme Engine: Transitioning to [${name}] silicate...`);
        const root = document.documentElement;
        
        root.style.setProperty('--acc-cyan', theme.primary);
        root.style.setProperty('--acc-gold', theme.secondary);
        
        UIUtils.appendLog('audit-log', `Theme shifted to [${name}].`, 'success');
    },

    pulseBranding() {
        const logo = document.querySelector('.br-logo');
        if (logo) UIUtils.pulseElement(logo, '0 0 30px var(--acc-cyan)');
    }
};

window.ThemeEngine = ThemeEngine;
