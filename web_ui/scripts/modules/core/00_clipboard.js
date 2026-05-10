/**
 * SigmaOS Sovereign Clipboard
 * Module 00: Secure cross-app data persistence and lattice-wide sync.
 */

const SovereignClipboard = {
    buffer: "",
    metadata: {
        timestamp: 0,
        type: "text",
        source: "system"
    },

    copy(text, source = "system") {
        this.buffer = text;
        this.metadata.timestamp = Date.now();
        this.metadata.source = source;
        
        console.log(`Σ Clipboard: Copied data from [${source}]`);
        UIUtils.appendLog('audit-log', `Clipboard: Data buffered from ${source}.`, 'success');
        
        // Pulse animation for feedback
        const activeWin = document.querySelector('.sovereign-window.active');
        if (activeWin) UIUtils.pulseElement(activeWin, '0 0 20px var(--acc-cyan)');
    },

    async paste() {
        if (!this.buffer) return "";
        UIUtils.appendLog('audit-log', `Clipboard: Pasting from buffer...`, 'normal');
        return this.buffer;
    },

    clear() {
        this.buffer = "";
        UIUtils.appendLog('audit-log', `Clipboard: Sovereign buffer cleared.`, 'warning');
    }
};

window.SovereignClipboard = SovereignClipboard;
