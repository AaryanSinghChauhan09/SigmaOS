/**
 * SigmaOS WebShell — NotificationCenter
 * Phase 1: Bell icon in system tray with slide-out panel.
 * No external libraries. No innerHTML with untrusted data.
 */

(function (global) {
  'use strict';

  /**
   * Notification config shape:
   *   { title: string, body?: string, timestamp?: number }
   */

  class NotificationCenter {
    constructor() {
      /** @type {Array<{id: number, title: string, body: string, timestamp: number, read: boolean}>} */
      this._queue     = [];
      this._idCounter = 0;
      this._badgeCount = 0;
      this._panelOpen  = false;

      this._bellEl   = null;
      this._badgeEl  = null;
      this._panelEl  = null;
      this._listEl   = null;

      this._render();
      this._registerAPI();
    }

    // ─── Build DOM ────────────────────────────────────────────────────────────

    _render() {
      // ── Tray icon container (fixed top-right) ──
      const tray = document.createElement('div');
      tray.id = 'sigma-notification-tray';
      tray.setAttribute('role', 'button');
      tray.setAttribute('tabindex', '0');
      tray.setAttribute('aria-label', 'Notifications');
      tray.setAttribute('aria-haspopup', 'true');
      tray.setAttribute('aria-expanded', 'false');
      tray.style.cssText = [
        'position:fixed',
        'top:8px',
        'right:12px',
        'z-index:99990',
        'cursor:pointer',
        'display:flex',
        'align-items:center',
        'justify-content:center',
        'width:34px',
        'height:34px',
        'border-radius:50%',
        'background:#313244',
        'border:1px solid #45475a',
        'user-select:none',
      ].join(';');

      // Bell SVG (inline, no external resources)
      const bellSvg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
      bellSvg.setAttribute('viewBox', '0 0 24 24');
      bellSvg.setAttribute('width',  '18');
      bellSvg.setAttribute('height', '18');
      bellSvg.setAttribute('fill', 'none');
      bellSvg.setAttribute('stroke', '#cdd6f4');
      bellSvg.setAttribute('stroke-width', '2');
      bellSvg.setAttribute('stroke-linecap', 'round');
      bellSvg.setAttribute('stroke-linejoin', 'round');
      bellSvg.setAttribute('aria-hidden', 'true');

      const path1 = document.createElementNS('http://www.w3.org/2000/svg', 'path');
      path1.setAttribute('d', 'M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9');
      const path2 = document.createElementNS('http://www.w3.org/2000/svg', 'path');
      path2.setAttribute('d', 'M13.73 21a2 2 0 0 1-3.46 0');
      bellSvg.appendChild(path1);
      bellSvg.appendChild(path2);

      // Badge
      const badge = document.createElement('span');
      badge.id = 'sigma-notification-badge';
      badge.setAttribute('aria-live', 'polite');
      badge.setAttribute('aria-atomic', 'true');
      badge.style.cssText = [
        'position:absolute',
        'top:-4px',
        'right:-4px',
        'min-width:16px',
        'height:16px',
        'padding:0 4px',
        'background:#f38ba8',
        'color:#1e1e2e',
        'font-size:10px',
        'font-family:sans-serif',
        'font-weight:bold',
        'border-radius:8px',
        'display:none',
        'align-items:center',
        'justify-content:center',
        'box-sizing:border-box',
        'pointer-events:none',
      ].join(';');

      tray.style.position = 'fixed'; // ensure relative for badge
      tray.style.setProperty('position', 'fixed');

      // Make tray relative so badge positions off it
      const wrapper = document.createElement('div');
      wrapper.style.cssText = [
        'position:fixed',
        'top:8px',
        'right:12px',
        'z-index:99990',
        'width:34px',
        'height:34px',
      ].join(';');

      tray.style.position = 'relative'; // override for inner positioning
      tray.style.top = '';
      tray.style.right = '';
      tray.style.zIndex = '';

      tray.appendChild(bellSvg);
      tray.appendChild(badge);
      wrapper.appendChild(tray);

      // ── Slide-out panel ──
      const panel = document.createElement('div');
      panel.id = 'sigma-notification-panel';
      panel.setAttribute('role', 'region');
      panel.setAttribute('aria-label', 'Notification panel');
      panel.style.cssText = [
        'position:fixed',
        'top:50px',
        'right:12px',
        'width:320px',
        'max-height:480px',
        'background:#1e1e2e',
        'border:1px solid #45475a',
        'border-radius:8px',
        'box-shadow:0 8px 32px rgba(0,0,0,0.6)',
        'z-index:99989',
        'display:flex',
        'flex-direction:column',
        'overflow:hidden',
        'transform:translateY(-10px)',
        'opacity:0',
        'pointer-events:none',
        'transition:transform 0.2s ease, opacity 0.2s ease',
      ].join(';');

      // Panel header
      const header = document.createElement('div');
      header.style.cssText = [
        'display:flex',
        'align-items:center',
        'justify-content:space-between',
        'padding:10px 14px',
        'border-bottom:1px solid #313244',
        'flex-shrink:0',
      ].join(';');

      const headerTitle = document.createElement('span');
      headerTitle.textContent = 'Notifications';
      headerTitle.style.cssText = 'color:#cdd6f4;font-size:13px;font-weight:600;font-family:sans-serif;';

      const clearBtn = document.createElement('button');
      clearBtn.textContent = 'Clear all';
      clearBtn.style.cssText = [
        'background:none',
        'border:none',
        'color:#89b4fa',
        'font-size:12px',
        'font-family:sans-serif',
        'cursor:pointer',
        'padding:2px 6px',
        'border-radius:4px',
      ].join(';');
      clearBtn.addEventListener('click', () => this._clearAll());

      header.appendChild(headerTitle);
      header.appendChild(clearBtn);

      // Scrollable list
      const list = document.createElement('div');
      list.id = 'sigma-notification-list';
      list.setAttribute('role', 'list');
      list.style.cssText = [
        'flex:1',
        'overflow-y:auto',
        'padding:8px 0',
      ].join(';');

      const emptyMsg = document.createElement('p');
      emptyMsg.id = 'sigma-notification-empty';
      emptyMsg.textContent = 'No notifications';
      emptyMsg.style.cssText = [
        'color:#6c7086',
        'font-size:13px',
        'font-family:sans-serif',
        'text-align:center',
        'padding:24px 0',
        'margin:0',
      ].join(';');
      list.appendChild(emptyMsg);

      panel.appendChild(header);
      panel.appendChild(list);

      document.body.appendChild(wrapper);
      document.body.appendChild(panel);

      // Store refs
      this._bellEl  = tray;
      this._badgeEl = badge;
      this._panelEl = panel;
      this._listEl  = list;

      // Event listeners
      tray.addEventListener('click', () => this._togglePanel());
      tray.addEventListener('keydown', (e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault();
          this._togglePanel();
        }
      });

      // Close panel when clicking outside
      document.addEventListener('click', (e) => {
        if (this._panelOpen && !panel.contains(e.target) && !wrapper.contains(e.target)) {
          this._closePanel();
        }
      });
    }

    // ─── API ──────────────────────────────────────────────────────────────────

    /**
     * Register as navigator.sigmaos.notification.show
     * Safe to call even if navigator.sigmaos doesn't exist yet.
     */
    _registerAPI() {
      if (typeof navigator === 'undefined') return;

      if (!navigator.sigmaos) {
        try {
          Object.defineProperty(navigator, 'sigmaos', {
            value: {},
            writable: true,
            configurable: true,
          });
        } catch (_) {
          navigator.sigmaos = {};
        }
      }

      if (!navigator.sigmaos.notification) {
        navigator.sigmaos.notification = {};
      }

      navigator.sigmaos.notification.show = (config) => this.show(config);
    }

    /**
     * show(config) — push a notification into the queue.
     * @param {{ title: string, body?: string, timestamp?: number }} config
     */
    show(config) {
      if (!config || typeof config.title !== 'string') {
        console.warn('NotificationCenter.show: config.title must be a string');
        return;
      }

      const entry = {
        id:        ++this._idCounter,
        title:     config.title,
        body:      typeof config.body === 'string' ? config.body : '',
        timestamp: typeof config.timestamp === 'number' ? config.timestamp : Date.now(),
        read:      false,
      };

      this._queue.push(entry);
      this._appendItem(entry);

      if (!this._panelOpen) {
        this._badgeCount++;
        this._updateBadge();
      }
    }

    // ─── Panel control ────────────────────────────────────────────────────────

    _togglePanel() {
      if (this._panelOpen) {
        this._closePanel();
      } else {
        this._openPanel();
      }
    }

    _openPanel() {
      this._panelOpen = true;
      this._panelEl.style.transform     = 'translateY(0)';
      this._panelEl.style.opacity       = '1';
      this._panelEl.style.pointerEvents = 'auto';
      this._bellEl.setAttribute('aria-expanded', 'true');

      // Clear badge and mark all read
      this._badgeCount = 0;
      this._updateBadge();
      this._markAllRead();
    }

    _closePanel() {
      this._panelOpen = false;
      this._panelEl.style.transform     = 'translateY(-10px)';
      this._panelEl.style.opacity       = '0';
      this._panelEl.style.pointerEvents = 'none';
      this._bellEl.setAttribute('aria-expanded', 'false');
    }

    // ─── Badge ────────────────────────────────────────────────────────────────

    _updateBadge() {
      if (this._badgeCount > 0) {
        this._badgeEl.textContent   = this._badgeCount > 99 ? '99+' : String(this._badgeCount);
        this._badgeEl.style.display = 'flex';
        this._bellEl.setAttribute('aria-label', 'Notifications (' + this._badgeCount + ' unread)');
      } else {
        this._badgeEl.style.display = 'none';
        this._bellEl.setAttribute('aria-label', 'Notifications');
      }
    }

    // ─── List rendering ───────────────────────────────────────────────────────

    _appendItem(entry) {
      // Remove empty placeholder if present
      const empty = document.getElementById('sigma-notification-empty');
      if (empty) empty.remove();

      const item = document.createElement('div');
      item.dataset.notifId = String(entry.id);
      item.setAttribute('role', 'listitem');
      item.style.cssText = [
        'padding:10px 14px',
        'border-bottom:1px solid #313244',
        'cursor:default',
        'display:flex',
        'flex-direction:column',
        'gap:3px',
      ].join(';');

      const titleEl = document.createElement('span');
      titleEl.textContent = entry.title;
      titleEl.style.cssText = [
        'color:#cdd6f4',
        'font-size:13px',
        'font-family:sans-serif',
        'font-weight:600',
      ].join(';');

      const meta = document.createElement('span');
      meta.style.cssText = 'display:flex;align-items:center;justify-content:space-between;';

      if (entry.body) {
        const bodyEl = document.createElement('span');
        bodyEl.textContent = entry.body;
        bodyEl.style.cssText = [
          'color:#a6adc8',
          'font-size:12px',
          'font-family:sans-serif',
        ].join(';');
        item.appendChild(titleEl);
        item.appendChild(bodyEl);
      } else {
        item.appendChild(titleEl);
      }

      const timeEl = document.createElement('time');
      timeEl.dateTime = new Date(entry.timestamp).toISOString();
      timeEl.textContent = this._formatTime(entry.timestamp);
      timeEl.style.cssText = [
        'color:#6c7086',
        'font-size:11px',
        'font-family:sans-serif',
      ].join(';');
      item.appendChild(timeEl);

      // Append to list (chronological — newest last)
      this._listEl.appendChild(item);

      // Auto-scroll to bottom to show newest
      this._listEl.scrollTop = this._listEl.scrollHeight;
    }

    _markAllRead() {
      this._queue.forEach((n) => { n.read = true; });
      // Visual indicator: dim unread items (none needed since we clear on open)
    }

    _clearAll() {
      this._queue = [];
      this._badgeCount = 0;
      this._updateBadge();

      // Clear list DOM
      while (this._listEl.firstChild) {
        this._listEl.removeChild(this._listEl.firstChild);
      }

      const empty = document.createElement('p');
      empty.id = 'sigma-notification-empty';
      empty.textContent = 'No notifications';
      empty.style.cssText = [
        'color:#6c7086',
        'font-size:13px',
        'font-family:sans-serif',
        'text-align:center',
        'padding:24px 0',
        'margin:0',
      ].join(';');
      this._listEl.appendChild(empty);
    }

    _formatTime(ts) {
      const d = new Date(ts);
      const h = d.getHours().toString().padStart(2, '0');
      const m = d.getMinutes().toString().padStart(2, '0');
      return h + ':' + m;
    }

    // ─── Public getters (for tests) ───────────────────────────────────────────
    get queue()      { return this._queue; }
    get badgeCount() { return this._badgeCount; }
    get panelOpen()  { return this._panelOpen; }
  }

  // Export
  if (typeof module !== 'undefined' && module.exports) {
    module.exports = { NotificationCenter };
  } else {
    global.NotificationCenter = NotificationCenter;
  }
}(typeof globalThis !== 'undefined' ? globalThis : this));
