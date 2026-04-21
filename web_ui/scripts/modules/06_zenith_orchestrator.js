/**
 * Sovereign Zenith Orchestrator (v4.0)
 * Hardened to utilize Silicon Primitives (Ʃ), minimizing high-level JS overhead.
 */

class ZenithComponent {
    constructor(id) {
        this.id = id;
        this.element = Sigma.node(id);
    }

    show() {
        if (this.element) this.element.classList.remove('hidden');
    }

    hide() {
        if (this.element) this.element.classList.add('hidden');
    }
}

class MissionControl extends ZenithComponent {
    constructor() {
        super('mission-control-overlay');
        this.bindEvents();
    }

    bindEvents() {
        const openBtn = Sigma.node('btn-open-mission-control');
        const closeBtn = Sigma.node('btn-close-mission-control');

        if (openBtn) openBtn.onclick = () => this.show(); 
        if (closeBtn) closeBtn.onclick = () => this.hide();
    }
}

class ZenithSystem {
    constructor() {
        this.version = "EXTINCTION-1 (APEX)";
        this.pulse = setInterval(() => this.heartbeat(), 1000);
    }

    boot() {
        console.log(`Σ://BOOT> Sovereign v${this.version} Active.`);
        this.missionControl = new MissionControl();
        this.hardenEnvironments();
    }

    hardenEnvironments() {
        const doc = document.documentElement;
        if (!!window.chrome) doc.setAttribute('data-browser', 'chromium');
        if (/^((?!chrome|android).)*safari/i.test(navigator.userAgent)) {
            doc.setAttribute('data-browser', 'safari');
        }
    }

    heartbeat() {
        // Low-level status pulse logic
    }
}

window.ZenithSystem = ZenithSystem;
