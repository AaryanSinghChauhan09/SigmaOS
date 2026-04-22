/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN STATE STORE (FRONTEND PATTERN)
 * =========================================================================
 * Mission: Centralized state management for the Zenith Dashboard.
 * Principles: Flux architecture, Immutable updates, Observable state.
 *
 * Implements a "Sovereign Store" to handle global UI state.
 * =========================================================================
 */

const SovereignStore = (() => {
    let state = {
        version: "v3250.4",
        shards: 443,
        principles: 13,
        user: "AaryanSinghChauhan09",
        status: "STABLE",
        telemetry: {
            cpu: 42,
            mem: 2.1,
            entropy: 12
        },
        logs: []
    };

    const listeners = [];

    return {
        getState: () => ({...state}),
        
        dispatch: (action) => {
            console.log(`[STORE]: Dispatching action ${action.type}`);
            switch(action.type) {
                case 'UPDATE_TELEMETRY':
                    state.telemetry = {...state.telemetry, ...action.payload};
                    break;
                case 'ADD_LOG':
                    state.logs = [...state.logs, action.payload].slice(-50);
                    break;
                case 'SET_STATUS':
                    state.status = action.payload;
                    break;
                default:
                    console.warn(`Unknown action: ${action.type}`);
            }
            // Notify listeners
            listeners.forEach(fn => fn(state));
        },

        subscribe: (fn) => {
            listeners.push(fn);
            return () => {
                const idx = listeners.indexOf(fn);
                if (idx > -1) listeners.splice(idx, 1);
            };
        }
    };
})();

// Export for use in index.js
window.SovereignStore = SovereignStore;
