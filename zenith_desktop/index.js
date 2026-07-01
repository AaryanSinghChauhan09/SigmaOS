// File: zenith_desktop/index.js

import TelemetrySystem from './modules/telemetry.js';
import { stateManager } from './modules/state-manager.js';

// Initialize systems
export const systems = {
    telemetry: new TelemetrySystem(),
    state: stateManager
};

// Start on load
window.addEventListener('load', () => {
    systems.telemetry.start();
    console.log('🚀 SigmaOS Zenith initialized');
});

// Cleanup on unload
window.addEventListener('beforeunload', () => {
    systems.telemetry.stop();
});

// Export for console debugging
window.SigmaOS = systems;
