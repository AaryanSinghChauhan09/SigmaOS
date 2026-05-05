/**
 * SigmaOS Silicon Primitives (v1.0)
 * LOW-LEVEL OVERRIDES: ELIMINATING HIGH-LEVEL JS DEPENDENCIES.
 * Minimal reliance on prototype methods (forEach, map) and browser globals.
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

    // Raw Hand-rolled Iteration (Zero HOF/Callback overhead where possible)
    each: function(arr, fn) {
        if (!arr) return;
        const len = arr.length;
        for (let i = 0; i < len; i++) {
            fn(arr[i], i);
        }
    },

    // Sovereign Memory Manager (UI Heap)
    Heap: class {
        constructor(size) {
            this.p = new Array(size);
            this.i = 0;
            this.active = 0; // Track active allocations
            console.log('Σ://HEAP> Allocated ' + size + ' UI slots.');
        }
        alloc(o) {
            if (this.active >= this.p.length) {
                console.warn('Σ://HEAP> HEAP OVERFLOW — object loss risk!');
                this.i = 0; // Force wrap with warning
            }
            this.p[this.i++] = o;
            this.active++;
            return this.i - 1;
        }
        free(idx) {
            if (idx >= 0 && idx < this.p.length) {
                this.p[idx] = null;
                this.active--;
            }
        }
    },

    // RAW Attribute proxy
    attr: function(el, k, v) {
        if (v === undefined) return el.getAttribute(k);
        el.setAttribute(k, v);
    }
};

window.Sigma = Ʃ;
window.uiHeap = new Ʃ.Heap(100);

/**
 * ZenithComponent (Base)
 * High-performance UI lifecycle primitives.
 */
class ZenithComponent {
    constructor(id) {
        this.id = id;
        this.element = Sigma.node(id);
        this.hIdx = window.uiHeap.alloc(this);
    }

    show() {
        if (this.element) this.element.classList.remove('hidden');
    }

    hide() {
        if (this.element) this.element.classList.add('hidden');
    }
}

window.ZenithComponent = ZenithComponent;
