/**
 * apps/sigmaTerm.js — SigmaOS Terminal Emulator
 * Vanilla JS, no external dependencies.
 * WebSocket PTY client for sigmad-process/pty_server.nim (port 17393 /pty)
 *
 * Features:
 *  - 80×24 character buffer with 1000-line scrollback
 *  - Basic ANSI escape stripping
 *  - Keystroke forwarding within 50 ms
 *  - Resize events sent as JSON {type:'resize',cols,rows}
 *  - Auto-reconnect with 2 s delay on close
 */

(function (root, factory) {
  if (typeof module !== 'undefined' && module.exports) {
    module.exports = factory();          // CommonJS / Node (testing)
  } else {
    root.SigmaTerm = factory();          // Browser global
  }
}(typeof globalThis !== 'undefined' ? globalThis : this, function () {
  'use strict';

  // ---------------------------------------------------------------------------
  // Constants
  // ---------------------------------------------------------------------------
  const PTY_WS_URL      = 'ws://localhost:17393/pty';
  const COLS            = 80;
  const ROWS            = 24;
  const MAX_SCROLLBACK  = 1000;
  const RECONNECT_DELAY = 2000;  // ms
  const INPUT_DEBOUNCE  = 50;    // ms (max latency before flush)

  // ANSI escape sequence pattern: ESC [ ... m  and similar CSI sequences
  // Also strips OSC, ESC alone, and common single-char escapes.
  const ANSI_RE = /\x1b(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~]|\][^\x07]*(?:\x07|\x1b\\))/g;

  // ---------------------------------------------------------------------------
  // Terminal buffer
  // ---------------------------------------------------------------------------
  class TermBuffer {
    constructor (cols, rows) {
      this.cols = cols;
      this.rows = rows;
      // Ring buffer: each entry is a string (one screen line, may be shorter)
      this._lines = [];
      this._cursor = { x: 0, y: 0 };   // logical cursor within current line
    }

    /** Append raw text (ANSI already stripped) into the line buffer. */
    write (text) {
      if (text.length === 0) return;

      for (let i = 0; i < text.length; i++) {
        const ch = text[i];

        if (ch === '\r') {
          this._cursor.x = 0;
          continue;
        }

        if (ch === '\n') {
          this._newline();
          continue;
        }

        if (ch === '\b') {
          if (this._cursor.x > 0) this._cursor.x--;
          continue;
        }

        if (ch === '\x07') continue;  // bell — ignore

        // Ensure we have a current line
        if (this._lines.length === 0) this._lines.push('');
        const idx = this._lines.length - 1;

        // Pad line if cursor is ahead of content
        while (this._lines[idx].length < this._cursor.x) {
          this._lines[idx] += ' ';
        }

        // Insert/overwrite character
        const line = this._lines[idx];
        this._lines[idx] =
          line.substring(0, this._cursor.x) +
          ch +
          line.substring(this._cursor.x + 1);

        this._cursor.x++;

        // Wrap at column width
        if (this._cursor.x >= this.cols) {
          this._newline();
        }
      }

      // Trim scrollback
      while (this._lines.length > MAX_SCROLLBACK) {
        this._lines.shift();
      }
    }

    _newline () {
      this._lines.push('');
      this._cursor.x = 0;
      this._cursor.y++;
    }

    /**
     * Return the last `rows` lines as an array for rendering.
     * Lines shorter than cols are padded with spaces.
     */
    getView () {
      const start = Math.max(0, this._lines.length - this.rows);
      const view  = this._lines.slice(start);
      while (view.length < this.rows) view.push('');
      return view.map(l => l.padEnd(this.cols, ' '));
    }

    /** Total number of lines (including scrollback). */
    get lineCount () { return this._lines.length; }
  }

  // ---------------------------------------------------------------------------
  // SigmaTerm class
  // ---------------------------------------------------------------------------
  class SigmaTerm {
    /**
     * @param {HTMLElement|string} container  — DOM element or CSS selector
     * @param {object}             [options]
     * @param {string}             [options.url]   — WebSocket URL override
     * @param {number}             [options.cols]  — columns (default 80)
     * @param {number}             [options.rows]  — rows (default 24)
     */
    constructor (container, options = {}) {
      if (typeof container === 'string') {
        container = document.querySelector(container);
      }
      if (!container) throw new Error('SigmaTerm: container not found');

      this._options = Object.assign({
        url:  PTY_WS_URL,
        cols: COLS,
        rows: ROWS,
      }, options);

      this._cols = this._options.cols;
      this._rows = this._options.rows;
      this._buf  = new TermBuffer(this._cols, this._rows);
      this._ws   = null;
      this._inputQueue = [];
      this._flushTimer  = null;
      this._reconnTimer = null;
      this._destroyed   = false;

      this._buildDOM(container);
      this._attachKeyboard();
      this._attachResize();
      this._connect();
    }

    // -------------------------------------------------------------------------
    // DOM
    // -------------------------------------------------------------------------
    _buildDOM (container) {
      container.style.cssText =
        'background:#000;color:#e0e0e0;font-family:monospace;' +
        'padding:4px;overflow:auto;position:relative;cursor:text;';

      this._pre = document.createElement('pre');
      this._pre.style.cssText =
        'margin:0;white-space:pre;line-height:1.2;font-size:14px;';
      container.appendChild(this._pre);
      this._container = container;

      // status indicator
      this._statusEl = document.createElement('div');
      this._statusEl.style.cssText =
        'position:absolute;top:2px;right:6px;font-size:10px;' +
        'color:#555;pointer-events:none;';
      this._statusEl.textContent = '●  disconnected';
      container.appendChild(this._statusEl);

      this._render();
    }

    _render () {
      const view = this._buf.getView();
      this._pre.textContent = view.join('\n');
      // Scroll to bottom
      this._container.scrollTop = this._container.scrollHeight;
    }

    _setStatus (msg, color) {
      this._statusEl.textContent = msg;
      this._statusEl.style.color = color || '#555';
    }

    // -------------------------------------------------------------------------
    // WebSocket
    // -------------------------------------------------------------------------
    _connect () {
      if (this._destroyed) return;
      this._setStatus('● connecting…', '#aaa');

      try {
        this._ws = new WebSocket(this._options.url);
        this._ws.binaryType = 'arraybuffer';
      } catch (e) {
        this._scheduleReconnect();
        return;
      }

      this._ws.addEventListener('open', () => {
        this._setStatus('● connected', '#4caf50');
        // Send current terminal size
        this._sendResize();
      });

      this._ws.addEventListener('message', (ev) => {
        let text;
        if (ev.data instanceof ArrayBuffer) {
          text = new TextDecoder().decode(ev.data);
        } else {
          text = String(ev.data);
        }
        this._onData(text);
      });

      this._ws.addEventListener('close', () => {
        this._setStatus('● disconnected', '#f44336');
        this._scheduleReconnect();
      });

      this._ws.addEventListener('error', () => {
        // close event will follow
      });
    }

    _scheduleReconnect () {
      if (this._destroyed) return;
      clearTimeout(this._reconnTimer);
      this._reconnTimer = setTimeout(() => {
        if (!this._destroyed) this._connect();
      }, RECONNECT_DELAY);
    }

    // -------------------------------------------------------------------------
    // Data path: PTY → screen
    // -------------------------------------------------------------------------
    _onData (raw) {
      // Strip ANSI escape sequences
      const clean = raw.replace(ANSI_RE, '');
      this._buf.write(clean);
      this._render();
    }

    // -------------------------------------------------------------------------
    // Data path: keyboard → PTY
    // -------------------------------------------------------------------------
    _attachKeyboard () {
      // Use the container as the keyboard event target (needs tabIndex)
      this._container.tabIndex = 0;
      this._container.addEventListener('keydown', (e) => {
        const seq = this._keyToSeq(e);
        if (seq !== null) {
          e.preventDefault();
          this._enqueueInput(seq);
        }
      });
    }

    /**
     * Map KeyboardEvent → string to send to PTY.
     * Returns null if the key should propagate normally (e.g. F12, Tab in browser).
     */
    _keyToSeq (e) {
      // Control characters
      if (e.ctrlKey && !e.altKey) {
        if (e.key.length === 1) {
          const code = e.key.toUpperCase().charCodeAt(0) - 64;
          if (code >= 1 && code <= 26) return String.fromCharCode(code);
        }
        if (e.key === '[')  return '\x1b';
        if (e.key === '\\') return '\x1c';
        if (e.key === ']')  return '\x1d';
        if (e.key === '^')  return '\x1e';
        if (e.key === '_')  return '\x1f';
      }

      // Special keys → VT100/xterm sequences
      switch (e.key) {
        case 'Enter':      return '\r';
        case 'Backspace':  return '\x7f';
        case 'Tab':        return '\t';
        case 'Escape':     return '\x1b';
        case 'ArrowUp':    return '\x1b[A';
        case 'ArrowDown':  return '\x1b[B';
        case 'ArrowRight': return '\x1b[C';
        case 'ArrowLeft':  return '\x1b[D';
        case 'Home':       return '\x1b[H';
        case 'End':        return '\x1b[F';
        case 'PageUp':     return '\x1b[5~';
        case 'PageDown':   return '\x1b[6~';
        case 'Delete':     return '\x1b[3~';
        case 'Insert':     return '\x1b[2~';
        case 'F1':         return '\x1bOP';
        case 'F2':         return '\x1bOQ';
        case 'F3':         return '\x1bOR';
        case 'F4':         return '\x1bOS';
        case 'F5':         return '\x1b[15~';
        case 'F6':         return '\x1b[17~';
        case 'F7':         return '\x1b[18~';
        case 'F8':         return '\x1b[19~';
        case 'F9':         return '\x1b[20~';
        case 'F10':        return '\x1b[21~';
        case 'F11':        return '\x1b[23~';
        case 'F12':        return '\x1b[24~';
      }

      // Printable characters
      if (e.key.length === 1 && !e.ctrlKey && !e.metaKey) {
        return e.key;
      }

      return null;
    }

    _enqueueInput (seq) {
      this._inputQueue.push(seq);

      // Flush within INPUT_DEBOUNCE ms
      if (this._flushTimer === null) {
        this._flushTimer = setTimeout(() => {
          this._flushInput();
        }, INPUT_DEBOUNCE);
      }
    }

    _flushInput () {
      this._flushTimer = null;
      if (this._inputQueue.length === 0) return;
      const data = this._inputQueue.join('');
      this._inputQueue = [];
      this._sendRaw(data);
    }

    _sendRaw (data) {
      if (!this._ws || this._ws.readyState !== WebSocket.OPEN) return;
      this._ws.send(data);
    }

    _sendResize () {
      if (!this._ws || this._ws.readyState !== WebSocket.OPEN) return;
      const msg = JSON.stringify({ type: 'resize', cols: this._cols, rows: this._rows });
      this._ws.send(msg);
    }

    // -------------------------------------------------------------------------
    // Resize observer
    // -------------------------------------------------------------------------
    _attachResize () {
      if (typeof ResizeObserver === 'undefined') return;

      this._resizeObs = new ResizeObserver(() => {
        this._recalcSize();
      });
      this._resizeObs.observe(this._container);
    }

    _recalcSize () {
      // Measure a single character using an offscreen span
      const probe = document.createElement('span');
      probe.style.cssText = 'position:absolute;visibility:hidden;' +
        'font-family:monospace;font-size:14px;white-space:pre;';
      probe.textContent = 'M';
      document.body.appendChild(probe);
      const charW = probe.getBoundingClientRect().width  || 8.4;
      const charH = probe.getBoundingClientRect().height || 16.8;
      document.body.removeChild(probe);

      const rect = this._container.getBoundingClientRect();
      const newCols = Math.max(10, Math.floor(rect.width  / charW));
      const newRows = Math.max(4,  Math.floor(rect.height / charH));

      if (newCols !== this._cols || newRows !== this._rows) {
        this._cols = newCols;
        this._rows = newRows;
        this._buf.cols = newCols;
        this._buf.rows = newRows;
        this._sendResize();
        this._render();
      }
    }

    // -------------------------------------------------------------------------
    // Public API
    // -------------------------------------------------------------------------

    /** Programmatically send a string to the PTY. */
    sendInput (data) {
      this._sendRaw(data);
    }

    /** Cleanly destroy the terminal (disconnect, stop reconnect). */
    destroy () {
      this._destroyed = true;
      clearTimeout(this._reconnTimer);
      clearTimeout(this._flushTimer);
      if (this._resizeObs) this._resizeObs.disconnect();
      if (this._ws) this._ws.close();
    }

    /** Read-only access to the current scrollback lines. */
    get lines () { return this._buf._lines.slice(); }

    /** Current columns. */
    get cols () { return this._cols; }

    /** Current rows. */
    get rows () { return this._rows; }
  }

  return SigmaTerm;
}));
