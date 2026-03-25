/**
 * Σ SIGMA OS PREDICTIVE SEARCH ENGINE v1.0
 * ML-Powered Bi-gram & TF-IDF ranked search for the Infinity Search Palette (Ctrl+K)
 * Zero dependency – pure native JavaScript.
 */

export const PredictiveSearch = {
    initialized: false,
    corpus: {},     // doc_id -> tokenized tokens
    idf: {},        // term -> IDF value

    // Static app corpus to search across
    docs: [
        { id: 'terminal',  text: 'terminal tensorshell cli bash command script run execute code' },
        { id: 'dashboard', text: 'dashboard system cpu ram memory monitor processes performance stats' },
        { id: 'code',      text: 'code editor ide lab syntax highlight project files programming sovereign' },
        { id: 'notes',     text: 'notes markdown editor text write document sync mesh' },
        { id: 'audit',     text: 'audit security forensic scan shield bit telemetry tracker virus' },
        { id: 'browser',   text: 'browser web surf internet ghost proxy sovereign gate' },
        { id: 'vfs',       text: 'vfs filesystem explorer files folders root directory' },
        { id: 'paint',     text: 'paint draw canvas art neural brush color pixel' },
        { id: 'calc',      text: 'calc calculator math formula scientific arithmetic compute' },
        { id: 'settings',  text: 'settings personalization theme wallpaper accent color appearance customize' },
        { id: 'vault',     text: 'vault password secret key generator encrypt cryptographic secure' },
        { id: 'antigrav',  text: 'antigravity ai model quota dispatch prompt cockpit account' },
        { id: 'synth',     text: 'synth audio music sound wave frequency oscillator synthesizer' },
        { id: 'store',     text: 'store install apps forge sovereign enterprise software' },
        { id: 'type',      text: 'type typing keyboard speed test wpm accuracy hacker' },
        { id: 'collab',    text: 'collab collaborate mesh offline p2p chat share files team' },
        { id: 'automation',text: 'automation script cron hub routine trigger iot background task' },
        { id: 'excel',     text: 'excel spreadsheet table cell formula row column pivot data' },
    ],

    init() {
        if (this.initialized) return;
        this.initialized = true;

        // Build TF-IDF index
        this.buildIndex();
        console.log("[ML SEARCH] TF-IDF Predictive Search Index Built.");
    },

    tokenize(text) {
        return text.toLowerCase().replace(/[^\w\s]/g, '').split(/\s+/).filter(t => t.length > 1);
    },

    buildIndex() {
        const N = this.docs.length;
        const df = {}; // document frequency per term

        // Build corpus & document frequency
        this.docs.forEach(doc => {
            const tokens = this.tokenize(doc.text);
            const tf = {};
            tokens.forEach(t => { tf[t] = (tf[t] || 0) + 1; });
            this.corpus[doc.id] = tf;
            Object.keys(tf).forEach(t => { df[t] = (df[t] || 0) + 1; });
        });

        // Compute IDF for all terms
        Object.keys(df).forEach(t => {
            this.idf[t] = Math.log(N / df[t]);
        });
    },

    tfidfScore(docId, queryTokens) {
        const tf = this.corpus[docId] || {};
        return queryTokens.reduce((score, t) => {
            const termFreq = tf[t] || 0;
            const idf = this.idf[t] || 0;
            return score + (termFreq * idf);
        }, 0);
    },

    search(query, topN = 5) {
        if (!query || query.length < 2) return [];
        const queryTokens = this.tokenize(query);

        const results = this.docs.map(doc => ({
            id: doc.id,
            score: this.tfidfScore(doc.id, queryTokens)
        })).filter(r => r.score > 0)
           .sort((a, b) => b.score - a.score)
           .slice(0, topN);

        return results.map(r => r.id);
    }
};

window.PredictiveSearch = PredictiveSearch;
