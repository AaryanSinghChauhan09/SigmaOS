/**
 * SigmaOS Environment Manager Shard
 * USP/Logic: Central detection for Browser, App, Live Boot, Cloud, and Dual Boot environments.
 */

class EnvironmentManager {
    constructor() {
        this.shardId = "S" + "201_environment_manager.js".split('_')[0] + "_EnvironmentManager";
        this.active = false;
        this.environment = "unknown";
        
        console.log(`Σ://INIT> ${this.shardId} Initializing...`);
        this.init();
    }

    init() {
        window.addEventListener('sigma.core.boot', () => {
            this.active = true;
            this.detect();
            console.log(`Σ://PLATFORM> ${this.shardId} Online in ${this.environment} context.`);
            this.registerCLI();
        });
    }

    detect() {
        // Simulation of deep environment probing
        if (navigator.userAgent.includes("Electron")) this.environment = "app";
        else if (window.location.hostname === "localhost" || window.location.hostname === "127.0.0.1") this.environment = "local-dev";
        else if (window.location.protocol === "file:") this.environment = "live-boot";
        else if (window.location.hostname.includes("cloud")) this.environment = "cloud";
        else this.environment = "browser";
    }

    registerCLI() {
        if(!window.SigmaCLI) window.SigmaCLI = {};
        window.SigmaCLI['env-status'] = (args) => {
            return `[Environment Manager] Environment: ${this.environment.toUpperCase()} | Status: Active`;
        };
    }
}

window.SigmaEnvironmentManager = new EnvironmentManager();
