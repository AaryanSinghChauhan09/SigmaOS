/**
 * SigmaOS Rolling Release Channel Shard
 * USP/Logic: Opt-in bleeding edge module updates vs stable branch.
 */

class RollingReleaseChannel {
    constructor() {
        this.shardId = "S" + "150_rolling_release_channel.js".split('_')[0] + "_RollingReleaseChannel";
        this.active = false;
        
        console.log(`Σ://INIT> ${this.shardId} Initializing: Rolling Release Channel...`);
        this.init();
    }

    init() {
        window.addEventListener('sigma.core.boot', () => {
            this.active = true;
            console.log(`Σ://LINUX_MODULARITY> ${this.shardId} Online. Opt-in bleeding edge module updates vs stable branch.`);
            this.registerCLI();
        });
    }

    registerCLI() {
        if(!window.SigmaCLI) window.SigmaCLI = {};
        window.SigmaCLI['os-release'] = (args) => {
            return `[Rolling Release Channel] Executing ${args.join(' ')}...`;
        };
    }
}

window.SigmaRollingReleaseChannel = new RollingReleaseChannel();
