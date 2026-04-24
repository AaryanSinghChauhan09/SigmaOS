/**
 * SigmaOS Dynamic Island Shard
 * Inspired by Apple's Dynamic Island & Live Activities.
 */

class DynamicIsland {
    constructor() {
        this.shardId = "S44_DynamicIsland";
        this.activeActivities = new Map();
        
        console.log(`Σ://INIT> ${this.shardId} Initializing shape-shifting UI module...`);
        this.init();
    }

    init() {
        window.addEventListener('sigma.core.boot', () => {
            console.log(`Σ://UI> ${this.shardId} Online. Live Activities ready.`);
        });
        
        // Listen for new background processes that need persistent UI representation
        window.addEventListener('sigma.island.spawn', (e) => {
            this.spawnActivity(e.detail.id, e.detail.data);
        });
    }

    spawnActivity(id, data) {
        this.activeActivities.set(id, data);
        console.log(`Σ://ISLAND> Morphing UI to accommodate activity [${id}]:`, data.title);
        // Dispatch UI update to the Zenith dashboard
        window.dispatchEvent(new CustomEvent('sigma.telemetry.pulse', {
            detail: { metric: 'ISLAND_ACTIVE', value: this.activeActivities.size }
        }));
    }
    
    updateActivity(id, progress) {
        if (this.activeActivities.has(id)) {
            console.log(`Σ://ISLAND> Activity [${id}] progress: ${progress}%`);
        }
    }
}

window.SigmaDynamicIsland = new DynamicIsland();
