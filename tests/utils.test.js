import { describe, it, expect } from 'vitest'

// Simple mock of the escapeHtml function from zenith_desktop.js
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

describe('SigmaOS JS Utilities', () => {
    it('should escape HTML characters', () => {
        const input = '<script>alert("XSS")</script>';
        const expected = '&lt;script&gt;alert("XSS")&lt;/script&gt;';
        expect(escapeHtml(input)).toBe(expected);
    });

    it('should not alter normal text', () => {
        const input = 'SigmaOS Zenith Desktop';
        expect(escapeHtml(input)).toBe(input);
    });
});
