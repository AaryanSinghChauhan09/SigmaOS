/**
 * Sovereign Theme Engine (v2.0)
 * Implements "CSS Live-Reload" to surpass static competitors.
 * Allows users to personalize SigmaOS in real-time without a reboot.
 */

class ThemeEngine extends ZenithComponent {
    constructor() {
        super('gui-view');
        this.root = document.documentElement;
        this.init();
    }

    init() {
        console.log('Σ://UI> Theme Engine Synced.');
        this.applyTheme('MATRIX');
    }

    applyTheme(preset) {
        switch(preset) {
            case 'MATRIX':
                this.setTokens('#00f0ff', '#8a2be2', '#ff0055');
                break;
            case 'GHOST_MICA':
                this.setTokens('#ffffff', '#cccccc', '#00f0ff');
                break;
            case 'SOVEREIGN_GOLD':
                this.setTokens('#ffcc00', '#ff8800', '#ffffff');
                break;
        }
        window.zenith.taskbar.notify(`THEME APPLIED: ${preset}`, 'OPTIMAL');
    }

    setTokens(cyan, purple, magenta) {
        this.root.style.setProperty('--acc-cyan', cyan);
        this.root.style.setProperty('--acc-purple', purple);
        this.root.style.setProperty('--acc-magenta', magenta);
        
        // Dynamic Glow Update
        this.root.style.setProperty('--glow-cyan', `0 0 20px ${cyan}99`);
        this.root.style.setProperty('--glow-magenta', `0 0 20px ${magenta}99`);
    }

    // Live-Reload Logic for CSS Injection
    injectStyle(css) {
        let styleEl = Sigma.node('dynamic-os-style');
        if (!styleEl) {
            styleEl = document.createElement('style');
            styleEl.id = 'dynamic-os-style';
            document.head.appendChild(styleEl);
        }
        styleEl.textContent = css;
    }
}

window.ThemeEngine = ThemeEngine;
