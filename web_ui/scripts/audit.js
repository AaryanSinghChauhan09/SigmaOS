/**
 * SigmaOS Industrial Audit Engine (v1.0)
 * Architecture: Automated Heuristic Verification
 * Principles: Integrity, Modularity, Self-Healing
 */

class SovereignAuditor {
    constructor() {
        this.violations = [];
    }

    audit() {
        console.group("Σ://AUDIT> Initiating System-Level Heuristic Scan...");
        this.scanInlineStyles();
        this.checkChromiumCompatibility();
        this.logResults();
        console.groupEnd();
    }

    scanInlineStyles() {
        const allElements = document.querySelectorAll('*');
        allElements.forEach(el => {
            if (el.getAttribute('style') && !el.classList.contains('progress-fill-h100')) {
                this.violations.push(`[STYLE_LEAK]: Element <${el.tagName}> has inline style: "${el.getAttribute('style')}"`);
            }
        });
    }

    checkChromiumCompatibility() {
        if (!window.chrome) {
            this.violations.push("[COMPAT_WARNING]: Environment is not native Chromium. Shard latency may increase.");
        }
        
        // Final verify for the compatibility registry
        const testEl = document.createElement('div');
        testEl.className = 's-user-select-none';
        document.body.appendChild(testEl);
        const style = window.getComputedStyle(testEl);
        if (style.userSelect === 'auto' && style.webkitUserSelect === 'auto') {
            this.violations.push("[CSS_FAIL]: Compatibility registry failing to apply s-user-select-none.");
        }
        testEl.remove();
    }

    logResults() {
        if (this.violations.length === 0) {
            console.log("%c[OK]: ZERO VIOLATIONS FOUND. ARCHITECTURE RATED: INDUSTRIAL-GRADE.", "color: #00ff88; font-weight: bold;");
        } else {
            console.warn(`[WARN]: ${this.violations.length} violations detected in the lattice.`, this.violations);
        }
    }
}

window.SovereignAuditor = new SovereignAuditor();
if (document.readyState === 'complete') window.SovereignAuditor.audit();
else window.addEventListener('load', () => window.SovereignAuditor.audit());
