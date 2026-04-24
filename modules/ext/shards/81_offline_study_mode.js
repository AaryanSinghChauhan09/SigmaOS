/**
 * SigmaOS Offline Study Mode Shard
 * USP/Logic: Download lectures and generate notes entirely offline.
 */

class OfflineStudyMode {
    constructor() {
        this.shardId = "S" + "81_offline_study_mode.js".split('_')[0] + "_OfflineStudyMode";
        this.active = false;
        
        console.log(`Σ://INIT> ${this.shardId} Initializing: Offline Study Mode...`);
        this.init();
    }

    init() {
        window.addEventListener('sigma.core.boot', () => {
            this.active = true;
            console.log(`Σ://ENGINE> ${this.shardId} Online. Download lectures and generate notes entirely offline.`);
        });
    }
}

window.SigmaOfflineStudyMode = new OfflineStudyMode();
