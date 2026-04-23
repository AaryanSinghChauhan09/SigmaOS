/**
 * SigmaOS Sovereign Event Bus
 * Module 00: Centralized pub/sub mechanism for modular communication.
 */

const EventBus = {
    events: {},

    subscribe(event, callback) {
        if (!this.events[event]) this.events[event] = [];
        this.events[event].push(callback);
        console.log(`Σ EventBus: New subscriber for [${event}]`);
    },

    publish(event, data) {
        if (!this.events[event]) return;
        this.events[event].forEach(callback => callback(data));
        console.log(`Σ EventBus: Published [${event}] with payload.`, data);
    },

    clear(event) {
        if (event) delete this.events[event];
        else this.events = {};
    }

    selfEvolve() {
        const mutations = [
            "Optimizing lattice resonance...",
            "Expanding semantic context...",
            "Hardening silicon primitives...",
            "Refining cross-kernel synthesis..."
        ];
        const mutation = mutations[Math.floor(Math.random() * mutations.length)];
        console.log(`Σ://EVOLUTION [${this.shardId}]> ${mutation}`);
        this.lastMutation = mutation;
    }
};

window.EventBus = EventBus;
