/**
 * SigmaOS UI Utilities
 * Module 00: Common styling and logging functions for the Zenith interface.
 */

const UIUtils = {
    getTimestamp() {
        return `[${new Date().toISOString().substring(11, 23)}]`;
    },

    appendLog(containerId, message, type = 'normal') {
        const log = document.getElementById(containerId);
        if (!log) return;
        
        const entry = document.createElement('div');
        entry.className = `log-entry ${type}`;
        entry.innerHTML = `<span class="timestamp">${this.getTimestamp()}</span> ${message}`;
        log.appendChild(entry);
        log.scrollTop = log.scrollHeight;
    },

    pulseElement(el, shadow = '0 0 30px var(--acc-cyan)', duration = 1000) {
        if (!el) return;
        el.style.boxShadow = shadow;
        setTimeout(() => el.style.boxShadow = '', duration);
    }
};

window.UIUtils = UIUtils;
