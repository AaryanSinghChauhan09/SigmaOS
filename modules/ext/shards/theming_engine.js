/**
 * SigmaOS: Sovereign Theming Engine
 * Inspired by GNOME and KDE Plasma.
 * USP: Dynamic, CSS-variable based theming for the Zenith Dashboard.
 */

const ThemingEngine = {
    themes: {
        sovereign_dark: {
            '--accent-color': '#00ffcc',
            '--bg-color': 'rgba(10, 10, 10, 0.85)',
            '--text-color': '#ffffff',
            '--mica-blur': '20px'
        },
        matrix_green: {
            '--accent-color': '#00ff41',
            '--bg-color': 'rgba(0, 5, 0, 0.95)',
            '--text-color': '#00ff41',
            '--mica-blur': '5px'
        },
        serenity_retro: {
            '--accent-color': '#000080',
            '--bg-color': '#c0c0c0',
            '--text-color': '#000000',
            '--mica-blur': '0px'
        }
    },

    applyTheme(themeName) {
        const theme = this.themes[themeName] || this.themes.sovereign_dark;
        console.log(`Σ://THEME_APPLY> ${themeName}`);
        
        Object.entries(theme).forEach(([prop, value]) => {
            document.documentElement.style.setProperty(prop, value);
        });
        
        UIUtils.appendLog('audit-log', `SYSTEM: Theme profile '${themeName}' applied.`, 'success');
    }
};

if (typeof window !== 'undefined') {
    window.SigmaThemingEngine = ThemingEngine;
}
