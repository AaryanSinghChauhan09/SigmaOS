/**
 * Σ SIGMA OS BEHAVIOR LEARNING ENGINE v1.0
 * Zero-dependency Reinforcement Learning module.
 * Tracks user interaction patterns and builds a live personalized profile
 * using a Markov Chain model and Q-Learning approximation.
 */

export const BehaviorEngine = {
    initialized: false,
    actionLog: [],   // Chronological user actions
    qTable: {},      // Q-Learning reward table: qTable[state][action]
    stateHistory: [],
    maxLog: 500,

    // Markov Chain: tracks transition probabilities (what opens after what)
    transitions: {}, // transitions[appA] -> { appB: count, appC: count }

    init() {
        if (this.initialized) return;
        this.initialized = true;

        // Restore from sessionStorage (persistence across refreshes)
        try {
            const saved = sessionStorage.getItem('sigma_behavior');
            if (saved) {
                const parsed = JSON.parse(saved);
                this.transitions = parsed.transitions || {};
                this.qTable = parsed.qTable || {};
                this.actionLog = parsed.actionLog || [];
            }
        } catch(_) {}

        // Wrap UIEngine.launch to intercept open events
        const self = this;
        if (window.UIEngine) {
            const _orig = window.UIEngine.launch.bind(window.UIEngine);
            window.UIEngine.launch = function(id) {
                self.record(id);
                return _orig(id);
            };
        }
        console.log("[ML BEHAVIOR] Markov Chain Learning Engine Online.");
    },

    record(appId) {
        const last = this.actionLog[this.actionLog.length - 1];
        this.actionLog.push({ app: appId, ts: Date.now() });
        if (this.actionLog.length > this.maxLog) this.actionLog.shift();

        // Build Markov Transition Matrix
        if (last) {
            if (!this.transitions[last.app]) this.transitions[last.app] = {};
            this.transitions[last.app][appId] = (this.transitions[last.app][appId] || 0) + 1;
        }

        // Q-Table Update (reward for opening a known app pair)
        const state = last ? last.app : 'start';
        const action = appId;
        if (!this.qTable[state]) this.qTable[state] = {};
        // Positive reward: user chose this path
        const prevQ = this.qTable[state][action] || 0;
        this.qTable[state][action] = prevQ + 0.1 * (1.0 - prevQ);

        this.persist();
    },

    getSuggestions(currentApp, count = 3) {
        const trans = this.transitions[currentApp];
        if (!trans) return [];
        return Object.entries(trans)
            .sort((a, b) => b[1] - a[1]) // sort by frequency
            .slice(0, count)
            .map(([app]) => app);
    },

    getTopApps(count = 5) {
        const freq = {};
        this.actionLog.forEach(e => { freq[e.app] = (freq[e.app] || 0) + 1; });
        return Object.entries(freq)
            .sort((a, b) => b[1] - a[1])
            .slice(0, count)
            .map(([app, hits]) => ({ app, hits }));
    },

    persist() {
        try {
            sessionStorage.setItem('sigma_behavior', JSON.stringify({
                transitions: this.transitions,
                qTable: this.qTable,
                actionLog: this.actionLog.slice(-100)
            }));
        } catch(_) {}
    }
};

window.BehaviorEngine = BehaviorEngine;
