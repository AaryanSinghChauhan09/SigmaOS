/**
 * Zenith UI Module: Command Palette & Search
 * Handles search debouncing and command execution logic.
 */

export class CommandPalette {
    constructor(stateManager) {
        this.stateManager = stateManager;
        this.debounceTimer = null;
    }

    /**
     * Debounced search to prevent O(N) rescanning on every keystroke.
     */
    search(query, callback) {
        if (this.debounceTimer) clearTimeout(this.debounceTimer);
        this.debounceTimer = setTimeout(() => {
            const results = this.stateManager.filterShards(query);
            callback(results);
        }, 150);
    }

    execute(command) {
        console.log(`[ZENITH] Executing command: ${command}`);
        // Integration with SovereignClawGateway via IPC would happen here
    }
}
