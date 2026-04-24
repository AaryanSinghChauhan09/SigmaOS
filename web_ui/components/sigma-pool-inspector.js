/**
 * Σ <sigma-pool-inspector> — Per-module memory pool dashboard
 */
class SigmaPoolInspector extends HTMLElement {
    constructor() { super(); this.attachShadow({ mode: 'open' }); this._interval = null; }

    connectedCallback() {
        this.shadowRoot.innerHTML = `
<style>
:host { display:block; font-family:'Inter',system-ui,sans-serif; }
.hdr { font-size:.7rem; letter-spacing:.1em; text-transform:uppercase; color:rgba(255,255,255,.4); margin-bottom:12px; }
.grid { display:grid; grid-template-columns:repeat(auto-fill,minmax(220px,1fr)); gap:10px; }
.card { background:rgba(255,255,255,.03); border:1px solid rgba(255,255,255,.06); border-radius:12px; padding:14px; transition:border-color .2s; }
.card:hover { border-color:rgba(99,102,241,.3); }
.card.danger { border-color:rgba(239,68,68,.4); background:rgba(239,68,68,.04); }
.name { font-size:.8rem; font-weight:600; color:#c7d2fe; margin-bottom:8px; }
.stat { display:flex; justify-content:space-between; font-size:.68rem; color:rgba(255,255,255,.45); margin-bottom:3px; }
.stat .v { color:rgba(255,255,255,.7); font-variant-numeric:tabular-nums; }
.bar { margin-top:8px; height:6px; background:rgba(255,255,255,.05); border-radius:3px; overflow:hidden; }
.fill { height:100%; border-radius:3px; transition:width .5s; background:linear-gradient(90deg,#22c55e,#16a34a); }
.fill.warn { background:linear-gradient(90deg,#f59e0b,#d97706); }
.fill.crit { background:linear-gradient(90deg,#ef4444,#dc2626); }
</style>
<div class="hdr">Memory Pool Isolation Inspector</div>
<div class="grid" id="g"></div>`;
        this._refresh();
        this._interval = setInterval(() => this._refresh(), 3000);
    }

    disconnectedCallback() { if (this._interval) clearInterval(this._interval); }

    async _refresh() {
        if (!window.SigmaAPI) return;
        const pools = await window.SigmaAPI.getPools();
        const g = this.shadowRoot.getElementById('g');
        g.innerHTML = pools.map(p => {
            const pct = Math.round((p.used_blocks / p.total_blocks) * 100);
            const cls = pct > 90 ? 'crit' : (pct > 70 ? 'warn' : '');
            return `<div class="card${pct>90?' danger':''}">
                <div class="name">${p.name}</div>
                <div class="stat"><span>Used/Total</span><span class="v">${p.used_blocks}/${p.total_blocks}</span></div>
                <div class="stat"><span>Peak</span><span class="v">${p.peak_blocks}</span></div>
                <div class="stat"><span>Allocs/Frees</span><span class="v">${p.alloc_calls}/${p.free_calls}</span></div>
                <div class="bar"><div class="fill ${cls}" style="width:${pct}%"></div></div>
            </div>`;
        }).join('');
    }
}
customElements.define('sigma-pool-inspector', SigmaPoolInspector);
