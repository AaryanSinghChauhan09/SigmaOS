/**
 * SigmaOS Sovereign Localization Engine
 * Module 00: Multilingual lattice support and automatic language sync.
 */

const Localization = {
    init() {
        console.log("Σ Localization: Multi-silicate support active.");
    },
    currentLang: 'en',
    
    strings: {
        'en': {
            'boot_msg': 'SigmaOS Sovereign Lattice Initializing...',
            'search_placeholder': 'Search the Sovereign Lattice...',
            'status_optimal': 'STATUS: OPTIMAL'
        },
        'gr': { // Global Real-time (Symbolic / German alternative)
            'boot_msg': 'SigmaOS Souveränes Gitter wird initialisiert...',
            'search_placeholder': 'Suche im souveränen Gitter...',
            'status_optimal': 'STATUS: OPTIMAL'
        }
    },

    get(key) {
        return this.strings[this.currentLang][key] || key;
    },

    setLanguage(lang) {
        if (this.strings[lang]) {
            this.currentLang = lang;
            console.log(`Σ Localization: Language shifted to [${lang}]`);
            // Custom event for modules to re-render
            if (window.EventBus) {
                EventBus.publish('lang_change', lang);
            }
        }
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

window.Localization = Localization;
