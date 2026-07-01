/**
 * SigmaOS Environment Abstraction Layer (S201)
 * Logic: Detects and abstracts the host environment (Browser, Electron/App, Native Simulation, Dual-Boot).
 */

class EnvironmentAbstractionLayer {
    constructor() {
        this.shardId = "S201_EnvironmentAbstractionLayer";
        this.currentEnv = "browser"; // Default
        
        console.log(`Σ://INIT> ${this.shardId} Initializing Environment Detection...`);
        this.detect();
        this.init();
    }

    detect() {
        // Simulation of multi-environment detection
        if (window.sigma_native_host) {
            this.currentEnv = "native";
        } else if (window.process && window.process.type === 'renderer') {
            this.currentEnv = "app";
        } else if (window.location.protocol === 'file:') {
            this.currentEnv = "local-fs";
        } else {
            this.currentEnv = "browser";
        }
        
        console.log(`Σ://ENV> Detected Host: ${this.currentEnv.toUpperCase()}`);
    }

    init() {
        window.addEventListener('sigma.core.boot', () => {
            document.body.setAttribute('data-env', this.currentEnv);
            console.log(`Σ://BOOT> Environment ${this.currentEnv} locked and optimized.`);
            this.registerCLI();
        });
    }

    registerCLI() {
        if(!window.SigmaCLI) window.SigmaCLI = {};
        window.SigmaCLI['env-check'] = (args) => {
            return `Current Execution Environment: ${this.currentEnv.toUpperCase()}`;
        };
        window.SigmaCLI['env-switch'] = (args) => {
            if (!args[0]) return "Usage: env-switch [native|app|browser]";
            this.currentEnv = args[0];
            document.body.setAttribute('data-env', this.currentEnv);
            return `Environment context switched to: ${this.currentEnv.toUpperCase()}`;
        };
    }
}

window.SigmaEnvironmentAbstractionLayer = new EnvironmentAbstractionLayer();
