/**
 * Σ SIGMA OS SOVEREIGN RECOMMENDATION ENGINE v1.0
 * Collaborative Filtering via Cosine Similarity (Item-Item)
 * Recommends Sovereign Apps based on usage vectors.
 */

export const RecommendationEngine = {
    initialized: false,

    // App feature vectors (manually curated, can be grown)
    appFeatures: {
        terminal:  [1, 0, 1, 0, 0, 0, 1],
        dashboard: [1, 1, 0, 0, 0, 0, 0],
        code:      [1, 0, 1, 0, 0, 0, 1],
        notes:     [0, 1, 0, 0, 1, 0, 0],
        audit:     [0, 0, 1, 1, 0, 0, 0],
        browser:   [0, 1, 0, 1, 0, 0, 0],
        vfs:       [1, 0, 1, 0, 0, 0, 0],
        paint:     [0, 1, 0, 0, 1, 1, 0],
        calc:      [0, 1, 0, 0, 0, 1, 0],
        settings:  [0, 1, 0, 0, 1, 0, 0],
        vault:     [0, 0, 1, 1, 0, 0, 0],
        antigrav:  [1, 1, 0, 0, 0, 0, 0],
        synth:     [0, 1, 0, 0, 0, 1, 0],
        store:     [0, 1, 0, 1, 1, 0, 0],
    },

    init() {
        if (this.initialized) return;
        this.initialized = true;
        console.log("[ML RECS] Item-Item Collaborative Filter Online.");
    },

    dot(a, b) {
        return a.reduce((sum, val, idx) => sum + val * b[idx], 0);
    },

    magnitude(a) {
        return Math.sqrt(a.reduce((sum, val) => sum + val * val, 0));
    },

    cosine(a, b) {
        const magA = this.magnitude(a);
        const magB = this.magnitude(b);
        if (magA === 0 || magB === 0) return 0;
        return this.dot(a, b) / (magA * magB);
    },

    getRecommendations(appId, topN = 4) {
        const base = this.appFeatures[appId];
        if (!base) return [];

        const scores = [];
        for (const [other, vec] of Object.entries(this.appFeatures)) {
            if (other === appId) continue;
            scores.push({ app: other, score: this.cosine(base, vec) });
        }

        return scores
            .sort((a, b) => b.score - a.score)
            .slice(0, topN)
            .map(s => s.app);
    },

    renderSuggestionsHUD(appId) {
        const recs = this.getRecommendations(appId);
        if (!recs.length) return;

        let hud = document.getElementById('sigma-ai-recs');
        if (!hud) {
            hud = document.createElement('div');
            hud.id = 'sigma-ai-recs';
            hud.style.cssText = `
                position: fixed; bottom: 80px; right: 16px;
                background: rgba(18,18,18,0.92);
                backdrop-filter: blur(16px);
                -webkit-backdrop-filter: blur(16px);
                border: 1px solid rgba(255,255,255,0.1);
                border-radius: 12px; padding: 10px 14px;
                z-index: 8888; font-size: 12px;
                font-family: 'Inter', system-ui, sans-serif;
                color: #888; box-shadow: 0 8px 32px rgba(0,0,0,0.8);
                transition: opacity 0.3s;
            `;
            document.body.appendChild(hud);
        }

        hud.innerHTML = `
            <div style="font-size:10px;text-transform:uppercase;letter-spacing:1px;color:#5AC8FA;font-weight:700;margin-bottom:8px;">Σ AI SUGGESTS</div>
            ${recs.map(r => `<div onclick="window.UIEngine && UIEngine.launch('${r}')" 
                style="padding:5px 0;cursor:pointer;color:#F2F2F2;border-bottom:1px solid rgba(255,255,255,0.05);"
                onmouseover="this.style.color='#5AC8FA'"
                onmouseout="this.style.color='#F2F2F2'">
                ▶ ${r.toUpperCase()}
            </div>`).join('')}
        `;
        hud.style.opacity = '1';

        // Auto-hide after 8 seconds
        clearTimeout(this._hideTimer);
        this._hideTimer = setTimeout(() => { if(hud) hud.style.opacity = '0'; }, 8000);
    }
};

window.RecommendationEngine = RecommendationEngine;
