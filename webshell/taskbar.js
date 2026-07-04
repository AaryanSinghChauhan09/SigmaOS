/**
 * SigmaOS WebShell — TaskBar
 * Task 3.4: Persistent taskbar with focus management.
 * No external libraries.
 */

(function (global) {
  'use strict';

  const TASKBAR_HEIGHT = 40;

  class TaskBar {
    /**
     * @param {WindowManager} windowManager
     */
    constructor(windowManager) {
      this._wm = windowManager;
      /** @type {Map<string, HTMLElement>} */
      this._buttons = new Map();
      this._el = null;
      this._render();

      // Tell the WindowManager about us
      if (this._wm && typeof this._wm.setTaskbar === 'function') {
        this._wm.setTaskbar(this);
      }
    }

    /** Create and attach the fixed taskbar element. */
    _render() {
      const bar = document.createElement('div');
      bar.id = 'sigma-taskbar';
      bar.setAttribute('role', 'toolbar');
      bar.setAttribute('aria-label', 'Taskbar');
      bar.style.cssText = [
        'position:fixed',
        'bottom:0',
        'left:0',
        'right:0',
        'height:' + TASKBAR_HEIGHT + 'px',
        'background:#181825',
        'border-top:1px solid #313244',
        'display:flex',
        'align-items:center',
        'padding:0 8px',
        'gap:4px',
        'z-index:9999',
        'box-sizing:border-box',
      ].join(';');

      document.body.appendChild(bar);
      this._el = bar;

      // Ensure body has bottom padding so windows aren't hidden behind taskbar
      if (document.body) {
        document.body.style.paddingBottom = TASKBAR_HEIGHT + 'px';
      }
    }

    /**
     * addWindow(id, title) — adds a button entry to the taskbar.
     */
    addWindow(id, title) {
      if (this._buttons.has(id)) return;

      const btn = document.createElement('button');
      btn.dataset.taskbarId = id;
      btn.textContent = title || id;
      btn.title = title || id;
      btn.setAttribute('aria-label', 'Focus window: ' + (title || id));
      btn.style.cssText = [
        'background:#313244',
        'border:1px solid #45475a',
        'border-radius:4px',
        'color:#cdd6f4',
        'font-size:12px',
        'font-family:sans-serif',
        'padding:4px 10px',
        'cursor:pointer',
        'max-width:160px',
        'overflow:hidden',
        'text-overflow:ellipsis',
        'white-space:nowrap',
        'height:28px',
        'flex-shrink:0',
      ].join(';');

      btn.addEventListener('click', () => {
        if (this._wm) {
          this._wm.focusWindow(id);
        }
        this.setActive(id);
      });

      this._el.appendChild(btn);
      this._buttons.set(id, btn);
    }

    /**
     * removeWindow(id) — removes button entry from taskbar.
     */
    removeWindow(id) {
      const btn = this._buttons.get(id);
      if (!btn) return;
      btn.remove();
      this._buttons.delete(id);
    }

    /**
     * setActive(id) — applies active CSS class to the focused window's button.
     */
    setActive(id) {
      this._buttons.forEach((btn, btnId) => {
        if (btnId === id) {
          btn.style.background = '#45475a';
          btn.style.borderColor = '#89b4fa';
          btn.setAttribute('aria-pressed', 'true');
        } else {
          btn.style.background = '#313244';
          btn.style.borderColor = '#45475a';
          btn.setAttribute('aria-pressed', 'false');
        }
      });
    }

    /** Expose internal element and buttons for tests */
    get element() { return this._el; }
    get buttons() { return this._buttons; }
  }

  // Export
  if (typeof module !== 'undefined' && module.exports) {
    module.exports = { TaskBar };
  } else {
    global.TaskBar = TaskBar;
  }
}(typeof globalThis !== 'undefined' ? globalThis : this));
