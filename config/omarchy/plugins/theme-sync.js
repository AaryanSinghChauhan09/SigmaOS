// SPDX-License-Identifier: MIT
/**
 * omarchy-theme-sync.js - Broadcasts Omarchy / Wallust theme tokens to web apps
 * Inspires by omacom/omarchy-theme-sync
 */
(function() {
  const THEME_CHANNEL = 'omarchy-theme-channel';
  let broadcastChannel = null;

  try {
    broadcastChannel = new BroadcastChannel(THEME_CHANNEL);
  } catch (e) {
    console.debug('BroadcastChannel not supported in current context');
  }

  window.OmarchyThemeSync = {
    /**
     * Broadcast an active theme update to all listening clients & iframes
     */
    broadcast: function(paletteName, colors) {
      const payload = {
        theme: paletteName,
        colors: colors,
        timestamp: Date.now()
      };

      // 1. BroadcastChannel
      if (broadcastChannel) {
        broadcastChannel.postMessage(payload);
      }

      // 2. Custom DOM Event
      window.dispatchEvent(new CustomEvent('omarchy:theme-change', { detail: payload }));

      // 3. LocalStorage Bridge for multi-tab sync
      try {
        localStorage.setItem('omarchy_active_theme', JSON.stringify(payload));
      } catch (e) {}

      // 4. Update CSS root variables
      const root = document.documentElement;
      for (const [key, val] of Object.entries(colors)) {
        root.style.setProperty(`--${key}`, val);
      }
    },

    /**
     * Listen for theme updates
     */
    listen: function(callback) {
      if (broadcastChannel) {
        broadcastChannel.onmessage = (event) => callback(event.data);
      }
      window.addEventListener('omarchy:theme-change', (e) => callback(e.detail));
    }
  };
})();
