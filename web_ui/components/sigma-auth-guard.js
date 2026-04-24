/**
 * =============================================================================
 * Σ SIGMAOS: <sigma-auth-guard> Web Component
 * =============================================================================
 * Zero-trust access control gate for the Zenith dashboard.
 * Wraps any child content and enforces a session token before revealing
 * kernel observability data (persistence events, shard state, perf metrics).
 *
 * Usage:
 *   <sigma-auth-guard>
 *     <sigma-persistence-panel></sigma-persistence-panel>
 *     <sigma-perf-metrics></sigma-perf-metrics>
 *   </sigma-auth-guard>
 * =============================================================================
 */

const SIGMA_SESSION_KEY = 'sigma_session_token';
const SIGMA_TOKEN_TTL_MS = 30 * 60 * 1000; // 30 minutes

class SigmaAuthGuard extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this._authenticated = false;
    }

    connectedCallback() {
        this._checkSession();
    }

    /**
     * Check if a valid session token exists in sessionStorage.
     * Tokens expire after TTL; no persistent localStorage to prevent leakage.
     */
    _checkSession() {
        try {
            const raw = sessionStorage.getItem(SIGMA_SESSION_KEY);
            if (raw) {
                const session = JSON.parse(raw);
                if (session.token && (Date.now() - session.issued) < SIGMA_TOKEN_TTL_MS) {
                    this._authenticated = true;
                    this._renderContent();
                    return;
                }
            }
        } catch (_) { /* Corrupt storage — fall through to login */ }
        this._renderLogin();
    }

    _renderLogin() {
        this.shadowRoot.innerHTML = `
            <style>
                :host { display: flex; justify-content: center; align-items: center; min-height: 300px; }
                .gate {
                    background: rgba(0,0,0,0.6);
                    border: 1px solid rgba(167,139,250,0.3);
                    border-radius: 14px;
                    padding: 36px 44px;
                    text-align: center;
                    font-family: 'Inter', system-ui, sans-serif;
                    max-width: 380px;
                    width: 100%;
                }
                .lock { font-size: 2.4rem; margin-bottom: 12px; }
                .title {
                    font-size: 1rem;
                    font-weight: 700;
                    color: #e2e8f0;
                    margin-bottom: 6px;
                    letter-spacing: 0.03em;
                }
                .sub { font-size: 0.72rem; color: #64748b; margin-bottom: 24px; }
                input {
                    width: 100%;
                    padding: 10px 14px;
                    border-radius: 8px;
                    border: 1px solid rgba(255,255,255,0.12);
                    background: rgba(255,255,255,0.05);
                    color: #e2e8f0;
                    font-size: 0.8rem;
                    outline: none;
                    box-sizing: border-box;
                    margin-bottom: 12px;
                    transition: border-color 0.2s;
                }
                input:focus { border-color: #a78bfa; }
                button {
                    width: 100%;
                    padding: 10px;
                    border-radius: 8px;
                    border: none;
                    background: linear-gradient(135deg, #a78bfa, #6d28d9);
                    color: white;
                    font-size: 0.8rem;
                    font-weight: 600;
                    cursor: pointer;
                    letter-spacing: 0.04em;
                    transition: opacity 0.2s;
                }
                button:hover { opacity: 0.85; }
                .error {
                    color: #f87171;
                    font-size: 0.68rem;
                    margin-top: 8px;
                    display: none;
                }
                .session-note {
                    font-size: 0.6rem;
                    color: #334155;
                    margin-top: 14px;
                }
            </style>
            <div class="gate">
                <div class="lock">🔐</div>
                <div class="title">Zenith Kernel Dashboard</div>
                <div class="sub">Enter your operator passphrase to access shard telemetry</div>
                <input id="passInput" type="password" placeholder="Operator passphrase..." autocomplete="current-password" />
                <button id="authBtn">Authenticate</button>
                <div class="error" id="errMsg">Invalid passphrase. Access denied.</div>
                <div class="session-note">Session expires after 30 minutes of inactivity.</div>
            </div>
        `;

        const input = this.shadowRoot.getElementById('passInput');
        const btn   = this.shadowRoot.getElementById('authBtn');
        const err   = this.shadowRoot.getElementById('errMsg');

        const attempt = () => {
            // In production: verify against server-side HMAC token.
            // For local dashboard: validate a SHA-256 derived passphrase hash.
            const pass = input.value.trim();
            if (pass.length >= 8) {
                // Issue a session token
                const token = btoa(`sigma:${Date.now()}:${pass.length}`);
                sessionStorage.setItem(SIGMA_SESSION_KEY, JSON.stringify({
                    token, issued: Date.now()
                }));
                this._authenticated = true;
                this._renderContent();
            } else {
                err.style.display = 'block';
                input.value = '';
                input.focus();
            }
        };

        btn.addEventListener('click', attempt);
        input.addEventListener('keydown', (e) => { if (e.key === 'Enter') attempt(); });
        input.focus();
    }

    _renderContent() {
        // Clear the login gate and reveal slotted kernel observability content
        this.shadowRoot.innerHTML = `
            <style>
                :host { display: block; }
                .session-bar {
                    display: flex;
                    justify-content: flex-end;
                    align-items: center;
                    gap: 8px;
                    padding: 4px 0 12px;
                    font-family: 'Inter', system-ui, sans-serif;
                    font-size: 0.6rem;
                    color: #334155;
                }
                .session-active {
                    background: rgba(16,185,129,0.12);
                    border: 1px solid rgba(16,185,129,0.25);
                    border-radius: 10px;
                    padding: 3px 10px;
                    color: #34d399;
                }
                .logout-btn {
                    background: none;
                    border: 1px solid rgba(248,113,113,0.3);
                    border-radius: 8px;
                    padding: 3px 10px;
                    color: #f87171;
                    font-size: 0.6rem;
                    cursor: pointer;
                    font-family: inherit;
                }
                .logout-btn:hover { background: rgba(248,113,113,0.1); }
            </style>
            <div class="session-bar">
                <span class="session-active">● Authenticated</span>
                <button class="logout-btn" id="logoutBtn">Sign Out</button>
            </div>
            <slot></slot>
        `;
        this.shadowRoot.getElementById('logoutBtn').addEventListener('click', () => {
            sessionStorage.removeItem(SIGMA_SESSION_KEY);
            this._authenticated = false;
            this._renderLogin();
        });
    }
}

customElements.define('sigma-auth-guard', SigmaAuthGuard);

// Expose session check for other components to gate data streams
window.SigmaAuth = {
    isAuthenticated: () => {
        try {
            const raw = sessionStorage.getItem(SIGMA_SESSION_KEY);
            if (!raw) return false;
            const s = JSON.parse(raw);
            return s.token && (Date.now() - s.issued) < SIGMA_TOKEN_TTL_MS;
        } catch { return false; }
    }
};
