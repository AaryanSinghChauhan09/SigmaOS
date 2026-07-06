// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// zenith_desktop/sigma_theme_store.js — Theme Store Client Stub

const ThemeStore = {
    isOpen: false,
    
    open: function() {
        if (this.isOpen) return;
        this.isOpen = true;
        console.log("Opening Theme Store...");
        // STUB: Fetch themes from backend and render UI overlay
        alert("Theme Store opened! (Stub)");
    },

    close: function() {
        this.isOpen = false;
        console.log("Closing Theme Store...");
    },

    applyTheme: function(themeId) {
        console.log(`Applying theme: ${themeId}`);
        // STUB: Update CSS variables and notify Zenith compositor
    }
};

window.ThemeStore = ThemeStore;
