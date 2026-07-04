/**
 * SigmaOS WebShell — LockScreen
 * Phase 1: Full-screen lock overlay with PIN auth.
 * No external libraries. No innerHTML with untrusted data.
 */

(function (global) {
  'use strict';

  // ─── XOR-based PIN hash (no stdlib) ──────────────────────────────────────
  // Simple deterministic hash: XOR each char code with a rotating key derived
  // from the PIN's length, then sum & mod to produce a 32-bit integer string.
  // NOT cryptographic — used only because no crypto stdlib is available here.
  function _hashPin(pin) {
    const KEY = [0x5a, 0x1c, 0xe3, 0x7f];
    let acc = 0;
    for (let i = 0; i < pin.length; i++) {
      acc = ((acc << 5) ^ (pin.charCodeAt(i) ^ KEY[i % KEY.length])) >>> 0;
    }
    // Mix length in to distinguish "1111" from "11"
    acc = ((acc * 31) ^ (pin.length * 0xdeadbeef)) >>> 0;
    return acc.toString(16).padStart(8, '0');
  }

  const LOCKOUT_DURATION_MS  = 30_000; // 30 seconds
  const MAX_FAILED_ATTEMPTS  = 3;
  const DEFAULT_PIN          = '1234'; // set via setPin() before deploying

  class LockScreen {
    /**
     * @param {{ pin?: string }} [options]
     */
    constructor(options = {}) {
      this._pinHash       = _hashPin(options.pin || DEFAULT_PIN);
      this._failedAttempts = 0;
      this._lockedUntil   = 0;   // epoch ms — 0 means not locked out
      this._clockInterval = null;
      this._overlayEl     = null;
      this._timeEl        = null;
      this._dateEl        = null;
      this._pinInput      = null;
      this._statusEl      = null;
      this._locked        = false;

      this._bindHotkey();
    }

    // ─── Public API ──────────────────────────────────────────────────────────

    /** Lock the screen immediately. */
    lock() {
      if (this._locked) return;
      this._locked = true;
      this._renderOverlay();
      this._startClock();
    }

    /** Unlock programmatically (bypasses PIN check — for API use only). */
    unlock() {
      this._dismiss();
    }

    /**
     * Change the stored PIN.
     * @param {string} pin  4–8 digit numeric string
     */
    setPin(pin) {
      if (typeof pin !== 'string' || !/^\d{4,8}$/.test(pin)) {
        throw new Error('PIN must be 4–8 numeric digits');
      }
      this._pinHash = _hashPin(pin);
    }

    /**
     * Verify a candidate PIN string.
     * Returns true if correct AND not in lockout window.
     * @param {string} input
     * @returns {boolean}
     */
    verifyCredential(input) {
      if (typeof input !== 'string') return false;

      const now = Date.now();
      if (now < this._lockedUntil) return false;    // still locked out

      if (_hashPin(input) === this._pinHash) {
        this._failedAttempts = 0;
        return true;
      }

      this._failedAttempts++;
      if (this._failedAttempts >= MAX_FAILED_ATTEMPTS) {
        this._lockedUntil   = now + LOCKOUT_DURATION_MS;
        this._failedAttempts = 0;
      }
      return false;
    }

    // ─── Hotkey ───────────────────────────────────────────────────────────────

    _bindHotkey() {
      document.addEventListener('keydown', (e) => {
        if (e.ctrlKey && (e.key === 'l' || e.key === 'L')) {
          e.preventDefault();
          this.lock();
        }
      });
    }

    // ─── Overlay ─────────────────────────────────────────────────────────────

    _renderOverlay() {
      const overlay = document.createElement('div');
      overlay.id = 'sigma-lock-overlay';
      overlay.setAttribute('role', 'dialog');
      overlay.setAttribute('aria-modal', 'true');
      overlay.setAttribute('aria-label', 'Lock screen');
      overlay.style.cssText = [
        'position:fixed',
        'inset:0',
        'z-index:99999',
        'background:rgba(17,17,27,0.97)',
        'display:flex',
        'flex-direction:column',
        'align-items:center',
        'justify-content:center',
        'gap:24px',
        'font-family:sans-serif',
        'user-select:none',
        'backdrop-filter:blur(12px)',
      ].join(';');

      // ── Clock section ──
      const clockSection = document.createElement('div');
      clockSection.style.cssText = 'text-align:center;';

      const timeEl = document.createElement('div');
      timeEl.id = 'sigma-lock-time';
      timeEl.setAttribute('aria-live', 'off');
      timeEl.setAttribute('aria-label', 'Current time');
      timeEl.style.cssText = [
        'color:#cdd6f4',
        'font-size:72px',
        'font-weight:200',
        'letter-spacing:-2px',
        'line-height:1',
      ].join(';');

      const dateEl = document.createElement('div');
      dateEl.id = 'sigma-lock-date';
      dateEl.setAttribute('aria-label', 'Current date');
      dateEl.style.cssText = [
        'color:#a6adc8',
        'font-size:18px',
        'font-weight:300',
        'margin-top:8px',
      ].join(';');

      clockSection.appendChild(timeEl);
      clockSection.appendChild(dateEl);

      // ── PIN section ──
      const pinSection = document.createElement('div');
      pinSection.style.cssText = [
        'display:flex',
        'flex-direction:column',
        'align-items:center',
        'gap:14px',
      ].join(';');

      const pinLabel = document.createElement('label');
      pinLabel.htmlFor = 'sigma-pin-input';
      pinLabel.textContent = 'Enter PIN to unlock';
      pinLabel.style.cssText = 'color:#a6adc8;font-size:14px;';

      const pinInput = document.createElement('input');
      pinInput.id = 'sigma-pin-input';
      pinInput.type = 'password';
      pinInput.inputMode = 'numeric';
      pinInput.pattern  = '[0-9]*';
      pinInput.maxLength = 8;
      pinInput.autocomplete = 'off';
      pinInput.setAttribute('aria-label', 'PIN');
      pinInput.style.cssText = [
        'width:200px',
        'padding:10px 14px',
        'background:#313244',
        'border:1px solid #45475a',
        'border-radius:8px',
        'color:#cdd6f4',
        'font-size:20px',
        'text-align:center',
        'letter-spacing:4px',
        'outline:none',
        'font-family:monospace',
      ].join(';');

      // Status / error message
      const statusEl = document.createElement('div');
      statusEl.id = 'sigma-lock-status';
      statusEl.setAttribute('aria-live', 'assertive');
      statusEl.setAttribute('aria-atomic', 'true');
      statusEl.style.cssText = [
        'color:#f38ba8',
        'font-size:13px',
        'min-height:20px',
        'text-align:center',
      ].join(';');

      // Submit button
      const submitBtn = document.createElement('button');
      submitBtn.textContent = 'Unlock';
      submitBtn.style.cssText = [
        'padding:8px 32px',
        'background:#89b4fa',
        'color:#1e1e2e',
        'border:none',
        'border-radius:8px',
        'font-size:14px',
        'font-weight:600',
        'cursor:pointer',
        'font-family:sans-serif',
      ].join(';');

      pinSection.appendChild(pinLabel);
      pinSection.appendChild(pinInput);
      pinSection.appendChild(statusEl);
      pinSection.appendChild(submitBtn);

      overlay.appendChild(clockSection);
      overlay.appendChild(pinSection);

      document.body.appendChild(overlay);

      this._overlayEl = overlay;
      this._timeEl    = timeEl;
      this._dateEl    = dateEl;
      this._pinInput  = pinInput;
      this._statusEl  = statusEl;

      // Focus PIN input
      setTimeout(() => pinInput.focus(), 50);

      // Event listeners
      const tryUnlock = () => this._handleSubmit();
      submitBtn.addEventListener('click', tryUnlock);
      pinInput.addEventListener('keydown', (e) => {
        if (e.key === 'Enter') tryUnlock();
      });

      // Trap focus inside overlay
      overlay.addEventListener('keydown', (e) => {
        if (e.key === 'Tab') {
          e.preventDefault();
          pinInput.focus();
        }
        // Block Ctrl+L while already locked
        if (e.ctrlKey && (e.key === 'l' || e.key === 'L')) {
          e.preventDefault();
        }
      });

      // Update clock immediately
      this._updateClock();
    }

    // ─── Clock ────────────────────────────────────────────────────────────────

    _startClock() {
      this._updateClock();
      this._clockInterval = setInterval(() => this._updateClock(), 1000);
    }

    _stopClock() {
      if (this._clockInterval !== null) {
        clearInterval(this._clockInterval);
        this._clockInterval = null;
      }
    }

    _updateClock() {
      if (!this._timeEl || !this._dateEl) return;

      const now = new Date();

      // Time: HH:MM:SS
      const h  = now.getHours().toString().padStart(2, '0');
      const mi = now.getMinutes().toString().padStart(2, '0');
      const s  = now.getSeconds().toString().padStart(2, '0');
      this._timeEl.textContent = h + ':' + mi + ':' + s;

      // Date: Weekday, Month Day Year
      const DAYS   = ['Sunday','Monday','Tuesday','Wednesday','Thursday','Friday','Saturday'];
      const MONTHS = ['January','February','March','April','May','June',
                      'July','August','September','October','November','December'];
      this._dateEl.textContent =
        DAYS[now.getDay()] + ', ' + MONTHS[now.getMonth()] + ' ' +
        now.getDate() + ', ' + now.getFullYear();
    }

    // ─── PIN submission ───────────────────────────────────────────────────────

    _handleSubmit() {
      const input = this._pinInput.value;

      // Check lockout first
      const now = Date.now();
      if (now < this._lockedUntil) {
        const remaining = Math.ceil((this._lockedUntil - now) / 1000);
        this._setStatus('Too many attempts. Try again in ' + remaining + 's.');
        this._pinInput.value = '';
        return;
      }

      if (this.verifyCredential(input)) {
        this._setStatus('');
        this._dismiss();
      } else {
        this._pinInput.value = '';
        const now2 = Date.now();
        if (now2 < this._lockedUntil) {
          const remaining = Math.ceil((this._lockedUntil - now2) / 1000);
          this._setStatus('Too many attempts. Locked for ' + remaining + 's.');
          this._pinInput.disabled = true;
          this._startLockoutTimer();
        } else {
          const left = MAX_FAILED_ATTEMPTS - this._failedAttempts;
          const msg  = left > 0
            ? 'Incorrect PIN. ' + left + ' attempt' + (left === 1 ? '' : 's') + ' remaining.'
            : 'Incorrect PIN.';
          this._setStatus(msg);
          // Shake animation
          this._shakeInput();
        }
      }
    }

    _startLockoutTimer() {
      const tick = () => {
        const remaining = Math.ceil((this._lockedUntil - Date.now()) / 1000);
        if (remaining <= 0) {
          this._setStatus('');
          this._pinInput.disabled = false;
          this._pinInput.focus();
        } else {
          this._setStatus('Locked. Try again in ' + remaining + 's.');
          setTimeout(tick, 1000);
        }
      };
      setTimeout(tick, 1000);
    }

    _shakeInput() {
      if (!this._pinInput) return;
      const el = this._pinInput;
      el.style.transition = 'transform 0.05s ease';
      const steps = [6, -6, 4, -4, 2, -2, 0];
      let i = 0;
      const step = () => {
        if (i >= steps.length) {
          el.style.transform = '';
          return;
        }
        el.style.transform = 'translateX(' + steps[i] + 'px)';
        i++;
        setTimeout(step, 50);
      };
      step();
    }

    _setStatus(msg) {
      if (this._statusEl) {
        this._statusEl.textContent = msg;
      }
    }

    // ─── Dismiss ─────────────────────────────────────────────────────────────

    _dismiss() {
      this._stopClock();
      if (this._overlayEl) {
        this._overlayEl.remove();
        this._overlayEl = null;
        this._timeEl    = null;
        this._dateEl    = null;
        this._pinInput  = null;
        this._statusEl  = null;
      }
      this._locked = false;
    }

    // ─── Public getters (for tests) ───────────────────────────────────────────
    get locked()         { return this._locked; }
    get failedAttempts() { return this._failedAttempts; }
    get lockedUntil()    { return this._lockedUntil; }
  }

  // Export
  if (typeof module !== 'undefined' && module.exports) {
    module.exports = { LockScreen, _hashPin };
  } else {
    global.LockScreen = LockScreen;
  }
}(typeof globalThis !== 'undefined' ? globalThis : this));
