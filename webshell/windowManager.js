/**
 * SigmaOS WebShell — WindowManager
 * Tasks 3.1 + 3.2: Window creation, focus, close, drag, resize
 * No external libraries. No window.open() calls.
 */

(function (global) {
  'use strict';

  class WindowManager {
    constructor() {
      /** @type {Map<string, {element: HTMLElement, config: Object}>} */
      this._windows = new Map();
      this._zIndexCounter = 1000;
      this._taskbar = null; // set by TaskBar after construction
    }

    /** Attach a TaskBar instance so WindowManager can call updateTaskbar(). */
    setTaskbar(taskbar) {
      this._taskbar = taskbar;
    }

    /** Generate a unique window id if none is supplied. */
    _genId() {
      return 'wnd-' + Math.random().toString(36).slice(2, 9);
    }

    /**
     * createWindow(config)
     * config: { id?, title, x, y, width, height, content? }
     * Returns the window id.
     */
    createWindow(config) {
      const id = config.id || this._genId();
      if (this._windows.has(id)) {
        console.warn('WindowManager: duplicate window id', id);
        return id;
      }

      const el = document.createElement('div');
      el.className = 'sigma-window';
      el.dataset.windowId = id;

      // Inline styles — position & size
      el.style.position  = 'fixed';
      el.style.left      = (config.x   ?? 100) + 'px';
      el.style.top       = (config.y   ?? 100) + 'px';
      el.style.width     = (config.width  ?? 400) + 'px';
      el.style.height    = (config.height ?? 300) + 'px';
      el.style.zIndex    = String(++this._zIndexCounter);
      el.style.background    = '#1e1e2e';
      el.style.border        = '1px solid #45475a';
      el.style.borderRadius  = '8px';
      el.style.boxShadow     = '0 8px 32px rgba(0,0,0,0.6)';
      el.style.overflow      = 'hidden';
      el.style.display       = 'flex';
      el.style.flexDirection = 'column';
      el.style.minWidth      = '200px';
      el.style.minHeight     = '150px';
      el.style.boxSizing     = 'border-box';

      // Title bar
      const titlebar = document.createElement('div');
      titlebar.className = 'sigma-window-titlebar';
      titlebar.style.cssText = [
        'display:flex',
        'align-items:center',
        'justify-content:space-between',
        'padding:6px 10px',
        'background:#313244',
        'cursor:move',
        'user-select:none',
        'flex-shrink:0',
      ].join(';');

      const titleText = document.createElement('span');
      titleText.textContent = config.title || 'Window';
      titleText.style.cssText = 'color:#cdd6f4;font-size:13px;font-family:sans-serif;';

      const closeBtn = document.createElement('button');
      closeBtn.textContent = '✕';
      closeBtn.style.cssText = [
        'background:#f38ba8',
        'border:none',
        'border-radius:50%',
        'width:14px',
        'height:14px',
        'cursor:pointer',
        'font-size:9px',
        'color:#1e1e2e',
        'display:flex',
        'align-items:center',
        'justify-content:center',
        'padding:0',
      ].join(';');
      closeBtn.setAttribute('aria-label', 'Close window');
      closeBtn.addEventListener('click', () => this.closeWindow(id));

      titlebar.appendChild(titleText);
      titlebar.appendChild(closeBtn);

      // Content area
      const contentArea = document.createElement('div');
      contentArea.className = 'sigma-window-content';
      contentArea.style.cssText = [
        'flex:1',
        'overflow:auto',
        'color:#cdd6f4',
        'font-family:sans-serif',
        'font-size:13px',
        'padding:8px',
      ].join(';');
      if (config.content) {
        contentArea.appendChild(config.content);
      }

      // Resize handle (bottom-right corner)
      const resizeHandle = document.createElement('div');
      resizeHandle.className = 'sigma-resize-handle';
      resizeHandle.style.cssText = [
        'position:absolute',
        'right:0',
        'bottom:0',
        'width:14px',
        'height:14px',
        'cursor:se-resize',
        'background:transparent',
      ].join(';');
      resizeHandle.setAttribute('aria-hidden', 'true');

      el.appendChild(titlebar);
      el.appendChild(contentArea);
      el.appendChild(resizeHandle);

      // Focus on click anywhere in the window
      el.addEventListener('mousedown', () => this.focusWindow(id));

      document.body.appendChild(el);

      this._windows.set(id, { element: el, config: Object.assign({}, config, { id }) });

      this._enableDrag(id, titlebar);
      this._enableResize(id, resizeHandle);

      if (this._taskbar) {
        this._taskbar.addWindow(id, config.title || 'Window');
        this._taskbar.setActive(id);
      }

      return id;
    }

    /**
     * focusWindow(id) — brings window to front.
     */
    focusWindow(id) {
      const entry = this._windows.get(id);
      if (!entry) return;
      entry.element.style.zIndex = String(++this._zIndexCounter);
      if (this._taskbar) {
        this._taskbar.setActive(id);
      }
    }

    /**
     * closeWindow(id) — removes window from DOM and registry.
     */
    closeWindow(id) {
      const entry = this._windows.get(id);
      if (!entry) return;
      entry.element.remove();
      this._windows.delete(id);
      if (this._taskbar) {
        this._taskbar.removeWindow(id);
      }
    }

    /** Internal: enable drag via mousedown on titlebar element. */
    _enableDrag(windowId, titlebarEl) {
      let startX = 0, startY = 0, startLeft = 0, startTop = 0;
      let dragging = false;

      const onMouseDown = (e) => {
        if (e.target.tagName === 'BUTTON') return; // don't drag via close btn
        dragging = true;
        startX = e.clientX;
        startY = e.clientY;
        const entry = this._windows.get(windowId);
        if (!entry) return;
        startLeft = parseInt(entry.element.style.left, 10) || 0;
        startTop  = parseInt(entry.element.style.top,  10) || 0;
        this.focusWindow(windowId);
        e.preventDefault();
      };

      const onMouseMove = (e) => {
        if (!dragging) return;
        const entry = this._windows.get(windowId);
        if (!entry) return;
        const deltaX = e.clientX - startX;
        const deltaY = e.clientY - startY;
        entry.element.style.left = (startLeft + deltaX) + 'px';
        entry.element.style.top  = (startTop  + deltaY) + 'px';
      };

      const onMouseUp = () => {
        dragging = false;
      };

      titlebarEl.addEventListener('mousedown', onMouseDown);
      document.addEventListener('mousemove', onMouseMove);
      document.addEventListener('mouseup', onMouseUp);
    }

    /** Internal: enable resize via bottom-right handle. Min 200×150. */
    _enableResize(windowId, handleEl) {
      const MIN_W = 200;
      const MIN_H = 150;

      let startX = 0, startY = 0, startW = 0, startH = 0;
      let resizing = false;

      const onMouseDown = (e) => {
        resizing = true;
        startX = e.clientX;
        startY = e.clientY;
        const entry = this._windows.get(windowId);
        if (!entry) return;
        startW = parseInt(entry.element.style.width,  10) || MIN_W;
        startH = parseInt(entry.element.style.height, 10) || MIN_H;
        this.focusWindow(windowId);
        e.preventDefault();
        e.stopPropagation();
      };

      const onMouseMove = (e) => {
        if (!resizing) return;
        const entry = this._windows.get(windowId);
        if (!entry) return;
        const newW = Math.max(MIN_W, startW + (e.clientX - startX));
        const newH = Math.max(MIN_H, startH + (e.clientY - startY));
        entry.element.style.width  = newW + 'px';
        entry.element.style.height = newH + 'px';
      };

      const onMouseUp = () => {
        resizing = false;
      };

      handleEl.addEventListener('mousedown', onMouseDown);
      document.addEventListener('mousemove', onMouseMove);
      document.addEventListener('mouseup', onMouseUp);
    }

    /** Expose registry for tests */
    get windows() { return this._windows; }
    get zIndexCounter() { return this._zIndexCounter; }
  }

  // Export
  if (typeof module !== 'undefined' && module.exports) {
    module.exports = { WindowManager };
  } else {
    global.WindowManager = WindowManager;
  }
}(typeof globalThis !== 'undefined' ? globalThis : this));
