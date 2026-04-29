/**
 * SigmaOS Sovereign Log Processor
 * Module 00: Industrial-grade system log filtering, colorization, and archival.
 */

const LogProcessor = {
    maxEntries: 500,

    process(message, level = 'normal') {
        const timestamp = new Date().toLocaleTimeString();
        const formatted = `<span class="log-time">[${timestamp}]</span> <span class="log-msg log-${level}">${message}</span>`;
        return formatted;
    },

    append(containerId, message, level = 'normal') {
        const container = document.getElementById(containerId);
        if (!container) return;

        const entry = document.createElement('div');
        entry.className = 'log-entry';
        entry.innerHTML = this.process(message, level);
        
        container.appendChild(entry);
        container.scrollTop = container.scrollHeight;

        if (container.children.length > this.maxEntries) {
            container.removeChild(container.firstChild);
        }
        
        // Pulse Effect for Critical/Warning
        if (level === 'danger' || level === 'warning') {
            UIUtils.pulseElement(entry, '0 0 10px var(--acc-cyan)');
        }
    }
};

window.LogProcessor = LogProcessor;
