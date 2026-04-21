/**
 * SigmaOS Sovereign Localization Engine
 * Module 00: Multilingual lattice support and automatic language sync.
 */

const Localization = {
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
};

window.Localization = Localization;
