/**
 * SigmaOS Aero Glass UI Shard
 * Transparent window effects, blur, and glassy styling
 */

class AeroGlass {
    constructor() {
        this.shardId = "S53_AeroGlass";
        this.glassWindows = [];
        this.blurIntensity = 10;
        this.transparency = 0.7;

        console.log(`Σ://INIT> ${this.shardId} Initializing Aero Glass UI system...`);
        this.init();
    }

    init() {
        window.addEventListener('sigma.core.boot', () => {
            console.log(`Σ://UI> ${this.shardId} Online. Aero Glass effects enabled.`);
        });
    }

    applyGlass(element, options = {}) {
        const finalOptions = {
            blur: this.blurIntensity,
            transparency: this.transparency,
            ...options
        };

        if (element) {
            element.style.backdropFilter = `blur(${finalOptions.blur}px)`;
            element.style.backgroundColor = `rgba(255, 255, 255, ${finalOptions.transparency})`;
            element.style.boxShadow = '0 8px 32px rgba(0, 0, 0, 0.1)';
            this.glassWindows.push(element);
            console.log(`Σ://UI> ${this.shardId} Applied Aero Glass to element`);
        }
    }

    setBlurIntensity(intensity) {
        if (intensity < 0 || intensity > 50) {
            console.error(`Σ://UI> ${this.shardId} Blur intensity must be between 0 and 50`);
            return;
        }
        this.blurIntensity = intensity;
        this.glassWindows.forEach(element => {
            element.style.backdropFilter = `blur(${intensity}px)`;
        });
        console.log(`Σ://UI> ${this.shardId} Blur intensity set to ${intensity}`);
    }

    setTransparency(value) {
        if (value < 0 || value > 1) {
            console.error(`Σ://UI> ${this.shardId} Transparency must be between 0 and 1`);
            return;
        }
        this.transparency = value;
        this.glassWindows.forEach(element => {
            const currentColor = element.style.backgroundColor;
            element.style.backgroundColor = currentColor.replace(
                /[\d.]+(?=\))$/,
                value.toFixed(2)
            );
        });
        console.log(`Σ://UI> ${this.shardId} Transparency set to ${value}`);
    }

    toggleGlass() {
        this.glassWindows.forEach(element => {
            element.style.display = element.style.display === 'none' ? 'block' : 'none';
        });
    }
}

window.SigmaAeroGlass = new AeroGlass();
