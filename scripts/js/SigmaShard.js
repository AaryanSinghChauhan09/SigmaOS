"use strict";

/**
 * Σ SOVEREIGN TOOL BASE CLASS (OOPS Core)
 * Encapsulation and Abstraction for all SigmaOS Shards.
 */
export class SigmaShard {
    constructor(id, name, system) {
        this.id = id;
        this.name = name;
        this.system = system;
    }

    /**
     * Virtual method for primary shard execution.
     * To be overridden by specialized shards (Polymorphism).
     */
    execute() {
        console.warn(`Shard [${this.id}] execute() not implemented.`);
    }

    /**
     * Virtual method for UI rendering.
     */
    render() {
        console.warn(`Shard [${this.id}] render() not implemented.`);
    }

    log(msg) {
        this.system.spawnToast(`${this.name}: ${msg}`);
    }
}
