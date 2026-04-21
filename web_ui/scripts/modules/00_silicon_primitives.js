/**
 * SigmaOS Silicon Primitives (v1.0)
 * Low-level operational overrides to reduce dependency on high-level JS globals.
 * Industrial-grade performance optimized for the Sovereign Lattice.
 */

const Ʃ = {
    // Manual DOM Proxy (Caching to minimize document lookup overhead)
    _cache: {},
    node: function(id) {
        if (!this._cache[id]) {
            this._cache[id] = document.getElementById(id);
        }
        return this._cache[id];
    },

    // Hand-rolled Iteration Logic (Avoiding HOF overhead)
    each: function(arr, fn) {
        if (!arr) return;
        const len = arr.length;
        for (let i = 0; i < len; i++) {
            fn(arr[i], i);
        }
    },

    // Zero-Dependency Status Mapping
    statusColor: function(level) {
        switch(level) {
            case 'CRITICAL': return '#ff0055';
            case 'OPTIMAL':  return '#00f2ff';
            case 'STABLE':   return '#8a2be2';
            default:         return '#ffffff';
        }
    },

    // Manual String Concatenation for DOM injection (avoiding costly template parsers)
    inject: function(targetId, html) {
        const el = this.node(targetId);
        if (el) el.innerHTML = html;
    }
};

window.Sigma = Ʃ;
