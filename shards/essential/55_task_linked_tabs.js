/**
 * SigmaOS Task-Linked Tabs Shard
 * USP/Logic: Attach tabs directly to to-do items or project tasks.
 */

class TaskLinkedTabs {
    constructor() {
        this.shardId = "S" + "55_task_linked_tabs.js".split('_')[0] + "_TaskLinkedTabs";
        this.active = false;
        
        console.log(`Σ://INIT> ${this.shardId} Initializing: Task-Linked Tabs...`);
        this.init();
    }

    init() {
        window.addEventListener('sigma.core.boot', () => {
            this.active = true;
            console.log(`Σ://USP> ${this.shardId} Online. Attach tabs directly to to-do items or project tasks.`);
        });
    }
}

window.SigmaTaskLinkedTabs = new TaskLinkedTabs();
