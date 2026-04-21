/**
 * SigmaOS UI Utilities
 * Module 00: Common styling and logging functions for the Zenith interface.
 */

const UIUtils = {
    getTimestamp() {
        return `[${new Date().toISOString().substring(11, 23)}]`;
    },

    appendLog(containerId, message, type = 'normal') {
        if (window.LogProcessor) {
            LogProcessor.append(containerId, message, type);
        }
    },

    pulseElement(el, shadow = '0 0 30px var(--acc-cyan)', duration = 1000) {
        if (!el) return;
        el.style.boxShadow = shadow;
        setTimeout(() => el.style.boxShadow = '', duration);
    }
};

window.UIUtils = UIUtils;
