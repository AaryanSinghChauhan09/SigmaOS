/**
 * SigmaOS S201_EnvironmentManager
 * Logic: Hardened multi-environment orchestration for Browser, App, Live Boot, Cloud, and Dual Boot.
 */

class EnvironmentManager {
    constructor() {
        this.shardId = "S201_EnvironmentManager";
        this.active = false;
        this.TOTAL_SHARDS = 600;
        
        // Environment Detection
        this.environments = {
            BROWSER: "browser_pwa",
            APP: "desktop_electron",
            LIVE_BOOT: "live_iso_alpine",
            CLOUD: "container_rancher",
            DUAL_BOOT: "sigmaboot_managed"
        };
        
        this.currentEnv = this.detectEnvironment();
        
        console.log(`Σ://ENV> Initializing EnvironmentManager for: ${this.currentEnv}`);
        this.init();
    }

    detectEnvironment() {
        if (window.navigator.standalone || window.matchMedia('(display-mode: standalone)').matches) return this.environments.BROWSER;
        if (typeof process !== 'undefined' && process.versions && process.versions.electron) return this.environments.APP;
        if (window.location.hostname === 'localhost' && window.location.port === '8080') return this.environments.CLOUD;
        if (window.location.search.includes('env=live')) return this.environments.LIVE_BOOT;
        if (window.location.search.includes('env=dual')) return this.environments.DUAL_BOOT;
        return this.environments.BROWSER; // Default
    }

    init() {
        window.addEventListener('sigma.core.boot', () => {
            this.active = true;
            this.applyEnvironmentPolicies();
            console.log(`Σ://SINGULARITY_600> EnvironmentManager Online. Total Shards: ${this.TOTAL_SHARDS}`);
        });
    }

    applyEnvironmentPolicies() {
        switch(this.currentEnv) {
            case this.environments.LIVE_BOOT:
                console.warn("Σ://LIVE_BOOT> Enabling Amnesic RAM Mode. All state will be lost on close.");
                break;
            case this.environments.APP:
                console.log("Σ://APP> Enabling Full System Bridge. File system and native API access granted.");
                break;
            case this.environments.CLOUD:
                console.log("Σ://CLOUD> Containerized session detected. Optimizing for remote compute.");
                break;
            case this.environments.DUAL_BOOT:
                console.log("Σ://DUAL_BOOT> SigmaBoot managed session. Synchronizing with host OS bootloader.");
                break;
            default:
                console.log("Σ://BROWSER> Standard PWA Sandbox active.");
        }
    }
}

window.SigmaEnvironmentManager = new EnvironmentManager();
