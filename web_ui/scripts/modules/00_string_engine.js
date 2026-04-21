/**
 * SigmaOS Sovereign String Engine
 * Module 00: Industrial-grade, zero-dependency string manipulation and sanitization.
 */

const StringEngine = {
    sanitize(str) {
        if (!str) return "";
        return str.replace(/[^\w\s\-\.\/]/gi, '');
    },

    truncate(str, len = 20) {
        if (str.length <= len) return str;
        return str.substring(0, len) + "...";
    },

    hash(str) {
        let h = 0;
        for (let i = 0; i < str.length; i++) {
            h = ((h << 5) - h) + str.charCodeAt(i);
            h |= 0;
        }
        return Math.abs(h).toString(16).toUpperCase();
    },

    formatBytes(bytes) {
        if (bytes === 0) return '0 B';
        const k = 1024;
        const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
    }
};

window.StringEngine = StringEngine;
