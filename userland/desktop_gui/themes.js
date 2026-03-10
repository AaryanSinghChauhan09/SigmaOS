/**
 * Σ SIGMA OS THEME ENGINE v3.0 [INFINITY CORE]
 * Personalization & Component DNA Morphing
 */

export const ThemeEngine = {
    initialized: false,
    themes: {
        'default': { accent: '#5AC8FA', bg: '#050505', text: '#F2F2F2', surface: '#121212' },
        'midnight': { accent: '#7D5FFF', bg: '#020205', text: '#E0E0FF', surface: '#0A0A15' },
        'forest': { accent: '#53D769', bg: '#030803', text: '#D0FFD0', surface: '#0A150A' },
        'rose': { accent: '#FF2D55', bg: '#0A0204', text: '#FFE0E5', surface: '#150A0F' },
        'ocean': { accent: '#00D2FF', bg: '#02050A', text: '#E0F0FF', surface: '#0A121A' }
    },

    init() {
        if (this.initialized) return;
        this.initialized = true;
        this.applyTheme('default');
        console.log("Theme Engine Ready.");
    },

    setTheme(name) {
        if (this.themes[name]) {
            this.applyTheme(name);
            console.log(`[THEME] System DNA morphing to ${name.toUpperCase()} protocol.`);
            SigmaKernel.notify(`THEME: System morphology updated to [${name}].`, "info");
        }
    },

    applyTheme(name) {
        const theme = this.themes[name];
        const root = document.documentElement;
        root.style.setProperty('--accent', theme.accent);
        root.style.setProperty('--bg', theme.bg);
        root.style.setProperty('--text', theme.text);
        root.style.setProperty('--surface', theme.surface);

        // Update accent glow
        const glow = theme.accent + '33'; // 20% alpha
        root.style.setProperty('--accent-glow', glow);

        root.className = `theme-${name}`;
    },

    setAccent(color) {
        document.documentElement.style.setProperty('--accent', color);
        document.documentElement.style.setProperty('--accent-glow', color + '33');
        console.log(`[THEME] Manual accent override initiated: ${color}`);
        SigmaKernel.notify(`THEME: Hue override active.`, "success");
    },

    morphDNA(competitor) {
        console.log(`[THEME] Absorbing USP from competitor: ${competitor}... Synchronizing...`);
        let accent = '#5AC8FA';
        let font = getComputedStyle(document.documentElement).getPropertyValue('--font');

        switch (competitor) {
            case 'Windows':
                accent = '#0078D4';
                font = "Segoe UI, sans-serif";
                break;
            case 'macOS':
                accent = '#FFD60A';
                font = "-apple-system, BlinkMacSystemFont, sans-serif";
                break;
            case 'Linux':
                accent = '#BB86FC';
                font = "monospace";
                break;
        }

        this.setAccent(accent);
        document.body.style.fontFamily = font;
        SigmaKernel.notify(`DNA_MORPH: SigmaOS now simulating ${competitor} UI/UX mechanics. Competitor USP absorbed.`, 'success');
    }
};

window.ThemeEngine = ThemeEngine;
window.setTheme = (name) => ThemeEngine.setTheme(name);
window.setAccent = (color) => ThemeEngine.setAccent(color);
window.morphDNA = (comp) => ThemeEngine.morphDNA(comp);
window.setWallpaper = () => {
    const url = document.getElementById('wallpaper-url').value;
    if (url) {
        document.body.style.backgroundImage = `url('${url}')`;
        document.body.style.backgroundSize = 'cover';
        document.body.style.backgroundPosition = 'center';
        SigmaKernel.notify('THEME: Background layer injected.', 'success');
    } else {
        document.body.style.backgroundImage = 'none';
        SigmaKernel.notify('THEME: Background layer cleared.', 'info');
    }
};
