/**
 * Sovereign Zenith Orchestrator (v3.0)
 * Implements a hardened, OOP-based modular architecture.
 * Eliminates legacy procedural hooks for industrial-grade UI management.
 */

class ZenithComponent {
    constructor(id) {
        this.id = id;
        this.element = document.getElementById(id);
    }

    show() {
        if (this.element) this.element.classList.remove('hidden');
    }

    hide() {
        if (this.element) this.element.classList.add('hidden');
    }

    toggle() {
        if (this.element) this.element.classList.toggle('hidden');
    }
}

class MissionControl extends ZenithComponent {
    constructor() {
        super('mission-control-overlay');
        this.setupListeners();
    }

    setupListeners() {
        const openBtn = document.getElementById('btn-open-mission-control');
        const closeBtn = document.getElementById('btn-close-mission-control');

        if (openBtn) openBtn.addEventListener('click', () => this.show());
        if (closeBtn) closeBtn.addEventListener('click', () => this.hide());
    }

    show() {
        super.show();
        console.log('Σ://UI> Mission Control Lattice Synchronized.');
    }
}

class ZenithSystem {
    constructor() {
        this.version = "EXTINCTION-1 (APEX)";
        this.components = {};
    }

    boot() {
        console.log(`Σ://BOOT> Initializing Sovereign Zenith v${this.version}`);
        
        // Initialize Core Components
        this.components.missionControl = new MissionControl();
        
        // Initialize Theme & Effects
        this.applyIndustrialHardenings();
        
        console.log('Σ://BOOT> UI Modularization Complete. Silicon Pulse Steady.');
    }

    applyIndustrialHardenings() {
        // Handle CSS compatibility for Chromium / Safari
        this.injectPlatformDetection();
    }

    injectPlatformDetection() {
        const isChromium = !!window.chrome;
        const isSafari = /^((?!chrome|android).)*safari/i.test(navigator.userAgent);
        
        if (isChromium) document.documentElement.setAttribute('data-browser', 'chromium');
        if (isSafari) document.documentElement.setAttribute('data-browser', 'safari');
    }
}

// Export to window for global access (legacy script support)
window.ZenithSystem = ZenithSystem;
