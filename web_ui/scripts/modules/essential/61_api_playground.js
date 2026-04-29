/**
 * SigmaOS API Playground Shard
 * USP/Logic: Built-in lightweight REST client for quick testing.
 */

class APIPlayground {
    constructor() {
        this.shardId = "S" + "61_api_playground.js".split('_')[0] + "_APIPlayground";
        this.active = false;
        
        console.log(`Σ://INIT> ${this.shardId} Initializing: API Playground...`);
        this.init();
    }

    init() {
        window.addEventListener('sigma.core.boot', () => {
            this.active = true;
            console.log(`Σ://USP> ${this.shardId} Online. Built-in lightweight REST client for quick testing.`);
        });
    }
}

window.SigmaAPIPlayground = new APIPlayground();
