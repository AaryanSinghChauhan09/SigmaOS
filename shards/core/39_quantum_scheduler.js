/**
 * SigmaOS Quantum Scheduler Shard
 * Implements a predictive scheduling algorithm for the simulated Sovereign OS.
 */

class QuantumScheduler {
    constructor() {
        this.shardId = "S39_QuantumScheduler";
        this.active = false;
        this.taskQueue = [];
        this.cycleRate = 16; // ms
        
        console.log(`Σ://INIT> ${this.shardId} Initializing predictive multi-threading...`);
        this.init();
    }

    init() {
        window.addEventListener('sigma.core.boot', () => {
            this.active = true;
            this.startQuantumLoop();
            console.log(`Σ://KERNEL> ${this.shardId} Online. Prediction engine engaged.`);
        });
        
        // Listen for task offloading
        window.addEventListener('sigma.task.spawn', (e) => {
            this.queueTask(e.detail);
        });
    }

    queueTask(task) {
        // Predictive Priority Weighting (PPW) Algorithm
        // Considers: urgency, estimated duration, and current queue depth.
        const loadFactor = this.taskQueue.length / 10;
        const complexity = task.duration ? task.duration / 100 : 1;
        
        task.priority = task.urgent ? 0 : Math.max(1, Math.floor(complexity + loadFactor));
        
        this.taskQueue.push(task);
        this.taskQueue.sort((a, b) => a.priority - b.priority);
        
        console.log(`Σ://SCHED> Task [${task.id || 'ANON'}] queued with PPW Priority: ${task.priority}`);
    }

    startQuantumLoop() {
        setInterval(() => {
            if (!this.active || this.taskQueue.length === 0) return;
            
            // Execute highest priority task slice
            const currentTask = this.taskQueue.shift();
            
            // Dispatch UI telemetry
            window.dispatchEvent(new CustomEvent('sigma.telemetry.pulse', {
                detail: { 
                    metric: 'SCHEDULER_LOAD', 
                    value: this.taskQueue.length,
                    active_task: currentTask.id || 'SYSTEM'
                }
            }));
            
            // Simulated Adaptive Slice Execution
            const sliceTime = Math.max(4, 20 - (currentTask.priority * 2));
            
            setTimeout(() => {
                // Re-queue if not finished (simulated)
                if (Math.random() > 0.7) {
                    currentTask.priority++; // Decay priority to prevent starvation
                    this.taskQueue.push(currentTask);
                    this.taskQueue.sort((a, b) => a.priority - b.priority);
                }
            }, sliceTime);
        }, this.cycleRate);
    }
}

// Instantiate shard
window.SigmaQuantumScheduler = new QuantumScheduler();
