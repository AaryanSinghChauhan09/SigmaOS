/**
 * SigmaOS Apps — SigmaNotes
 * Phase 1: Two-pane markdown editor with live preview.
 * Vanilla JS, no external libraries.
 */

(function (root, factory) {
  if (typeof module !== 'undefined' && module.exports) {
    module.exports = factory();
  } else {
    root.SigmaNotes = factory();
  }
}(typeof globalThis !== 'undefined' ? globalThis : this, function () {
  'use strict';

  // ─── Allowed HTML tags for sanitization ─────────────────────────────────
  const ALLOWED_TAGS = new Set([
    'p', 'b', 'i', 'h1', 'h2', 'h3', 'h4', 'h5', 'h6',
    'a', 'code', 'br',
  ]);

  // ─── Markdown → HTML string renderer (scratch-built) ────────────────────

  /**
   * renderMarkdown(text) → HTML string
   *
   * Supported syntax (processed in order):
   *  1. # Heading 1 … ###### Heading 6
   *  2. **bold**
   *  3. _italic_
   *  4. `code`
   *  5. [text](url)  — href sanitized (only http/https/mailto)
   *  6. Blank lines → paragraph breaks
   *  7. Non-blank lines joined with <br>
   */
  function renderMarkdown(text) {
    if (typeof text !== 'string') return '';

    const lines = text.split('\n');
    const blocks = [];        // array of {type, content}
    let i = 0;

    while (i < lines.length) {
      const line = lines[i];

      // Heading
      const headingMatch = line.match(/^(#{1,6})\s+(.*)/);
      if (headingMatch) {
        blocks.push({ type: 'heading', level: headingMatch[1].length, content: headingMatch[2] });
        i++;
        continue;
      }

      // Blank line — flush any pending paragraph
      if (line.trim() === '') {
        i++;
        continue;
      }

      // Collect consecutive non-blank, non-heading lines as a paragraph
      const paraLines = [];
      while (i < lines.length && lines[i].trim() !== '' && !lines[i].match(/^#{1,6}\s/)) {
        paraLines.push(lines[i]);
        i++;
      }
      if (paraLines.length > 0) {
        blocks.push({ type: 'paragraph', lines: paraLines });
      }
    }

    let html = '';
    for (const block of blocks) {
      if (block.type === 'heading') {
        const level = block.level;
        const tag = 'h' + level;
        html += '<' + tag + '>' + _inlineRender(block.content) + '</' + tag + '>';
      } else if (block.type === 'paragraph') {
        const inner = block.lines.map((l) => _inlineRender(l)).join('<br>');
        html += '<p>' + inner + '</p>';
      }
    }

    return html;
  }

  /**
   * _inlineRender(text) — handle inline Markdown within a single string.
   * Processes: **bold**, _italic_, `code`, [text](url)
   * Returns an HTML string (no user-controlled tags introduced).
   */
  function _inlineRender(text) {
    // We build output token by token via a simple state machine / regex scan.
    // Using a single-pass replacement chain is safe here because each rule
    // operates on a disjoint syntax marker.

    // 1. Escape any raw HTML angle brackets first (prevents injection)
    text = text
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;');

    // 2. `code` — process before bold/italic so backtick wins
    text = text.replace(/`([^`]+)`/g, function (_, inner) {
      return '<code>' + inner + '</code>';
    });

    // 3. **bold**
    text = text.replace(/\*\*([^*]+)\*\*/g, function (_, inner) {
      return '<b>' + inner + '</b>';
    });

    // 4. _italic_
    text = text.replace(/_([^_]+)_/g, function (_, inner) {
      return '<i>' + inner + '</i>';
    });

    // 5. [text](url)
    text = text.replace(/\[([^\]]+)\]\(([^)]+)\)/g, function (_, linkText, rawUrl) {
      const safeUrl = _sanitizeUrl(rawUrl);
      if (!safeUrl) return linkText; // strip unsafe link but keep text
      return '<a href="' + safeUrl + '">' + linkText + '</a>';
    });

    return text;
  }

  /** Allow only http, https, and mailto URLs. */
  function _sanitizeUrl(url) {
    const trimmed = (url || '').trim();
    if (/^https?:\/\//i.test(trimmed) || /^mailto:/i.test(trimmed)) {
      return trimmed;
    }
    return null;
  }

  // ─── HTML sanitizer ──────────────────────────────────────────────────────

  /**
   * sanitizeHTML(html) → DocumentFragment
   *
   * Parses the rendered HTML string into a DocumentFragment using DOMParser,
   * then recursively walks the tree and removes or replaces any nodes whose
   * tag name is not in ALLOWED_TAGS.  Only attributes on <a> (href) are kept.
   *
   * Returns a DocumentFragment safe to insert into the DOM.
   */
  function sanitizeHTML(htmlString) {
    const frag = document.createDocumentFragment();

    let doc;
    try {
      const parser = new DOMParser();
      doc = parser.parseFromString('<div id="__root">' + htmlString + '</div>', 'text/html');
    } catch (_) {
      // DOMParser unavailable (e.g. old Node test env) — safe fallback
      const p = document.createElement('p');
      p.textContent = htmlString;
      frag.appendChild(p);
      return frag;
    }

    const root = doc.getElementById('__root');
    if (!root) return frag;

    _walkSanitize(root, frag, document);
    return frag;
  }

  /**
   * Recursively copy allowed nodes from `src` into `dest`.
   * @param {Element|DocumentFragment} src
   * @param {Node} dest
   * @param {Document} targetDoc
   */
  function _walkSanitize(src, dest, targetDoc) {
    for (const child of Array.from(src.childNodes)) {
      if (child.nodeType === Node.TEXT_NODE) {
        dest.appendChild(targetDoc.createTextNode(child.nodeValue));
        continue;
      }

      if (child.nodeType !== Node.ELEMENT_NODE) continue;

      const tag = child.tagName.toLowerCase();

      if (!ALLOWED_TAGS.has(tag)) {
        // Discard the element but recurse into its children
        _walkSanitize(child, dest, targetDoc);
        continue;
      }

      const el = targetDoc.createElement(tag);

      // Only keep href on <a> elements, and re-validate it
      if (tag === 'a') {
        const href = child.getAttribute('href');
        const safe = href ? _sanitizeUrl(href) : null;
        if (safe) {
          el.setAttribute('href', safe);
          el.setAttribute('target', '_blank');
          el.setAttribute('rel', 'noopener noreferrer');
        }
      }

      _walkSanitize(child, el, targetDoc);
      dest.appendChild(el);
    }
  }

  // ─── Debounce utility ─────────────────────────────────────────────────────

  function debounce(fn, ms) {
    let timer = null;
    return function (...args) {
      clearTimeout(timer);
      timer = setTimeout(() => fn.apply(this, args), ms);
    };
  }

  // ─── SigmaNotes class ────────────────────────────────────────────────────

  class SigmaNotes {
    /**
     * @param {HTMLElement} container  — element to render into
     */
    constructor(container) {
      if (!container) throw new Error('SigmaNotes: container is required');
      this._container = container;
      this._textareaEl = null;
      this._previewEl  = null;
      this._statusEl   = null;
      this._content    = '';

      this._buildDOM();
      this._bindEvents();
    }

    // ─── DOM ────────────────────────────────────────────────────────────────

    _buildDOM() {
      const c = this._container;
      c.style.cssText = [
        'display:grid',
        'grid-template-rows:auto 1fr',
        'height:100%',
        'background:#1e1e2e',
        'color:#cdd6f4',
        'font-family:sans-serif',
        'box-sizing:border-box',
        'overflow:hidden',
      ].join(';');

      // ── Toolbar ──
      const toolbar = document.createElement('div');
      toolbar.setAttribute('role', 'toolbar');
      toolbar.setAttribute('aria-label', 'Formatting toolbar');
      toolbar.style.cssText = [
        'display:flex',
        'align-items:center',
        'gap:4px',
        'padding:6px 8px',
        'background:#181825',
        'border-bottom:1px solid #313244',
        'flex-shrink:0',
      ].join(';');

      const buttons = [
        { label: 'Bold',    aria: 'Insert bold',    action: () => this._insertBold()    },
        { label: 'Italic',  aria: 'Insert italic',  action: () => this._insertItalic()  },
        { label: 'H1',      aria: 'Insert heading', action: () => this._insertHeading() },
        { label: 'Link',    aria: 'Insert link',    action: () => this._insertLink()    },
        { label: 'Code',    aria: 'Insert code',    action: () => this._insertCode()    },
      ];

      buttons.forEach(({ label, aria, action }) => {
        const btn = document.createElement('button');
        btn.textContent = label;
        btn.setAttribute('aria-label', aria);
        btn.style.cssText = [
          'background:#313244',
          'border:1px solid #45475a',
          'border-radius:4px',
          'color:#cdd6f4',
          'font-size:12px',
          'font-family:monospace',
          'padding:3px 10px',
          'cursor:pointer',
        ].join(';');
        btn.addEventListener('click', (e) => {
          e.preventDefault();
          action();
        });
        toolbar.appendChild(btn);
      });

      // ── Panes wrapper ──
      const panes = document.createElement('div');
      panes.style.cssText = [
        'display:grid',
        'grid-template-columns:1fr 1fr',
        'height:100%',
        'overflow:hidden',
      ].join(';');

      // Editor pane
      const textarea = document.createElement('textarea');
      textarea.id = 'sigma-notes-editor';
      textarea.setAttribute('aria-label', 'Markdown editor');
      textarea.setAttribute('spellcheck', 'false');
      textarea.style.cssText = [
        'width:100%',
        'height:100%',
        'background:#181825',
        'color:#cdd6f4',
        'font-size:14px',
        'font-family:monospace',
        'padding:12px',
        'border:none',
        'border-right:1px solid #313244',
        'resize:none',
        'outline:none',
        'box-sizing:border-box',
        'tab-size:2',
        'line-height:1.6',
      ].join(';');

      // Preview pane
      const preview = document.createElement('div');
      preview.id = 'sigma-notes-preview';
      preview.setAttribute('aria-label', 'Preview');
      preview.setAttribute('aria-live', 'polite');
      preview.style.cssText = [
        'width:100%',
        'height:100%',
        'padding:12px 16px',
        'overflow-y:auto',
        'box-sizing:border-box',
        'background:#1e1e2e',
        'color:#cdd6f4',
        'font-size:14px',
        'line-height:1.7',
      ].join(';');

      panes.appendChild(textarea);
      panes.appendChild(preview);

      c.appendChild(toolbar);
      c.appendChild(panes);

      this._textareaEl = textarea;
      this._previewEl  = preview;
    }

    // ─── Events ──────────────────────────────────────────────────────────────

    _bindEvents() {
      const debouncedRender = debounce(() => this._renderMarkdown(), 500);

      this._textareaEl.addEventListener('input', () => {
        this._content = this._textareaEl.value;
        debouncedRender();
      });
    }

    // ─── Render ───────────────────────────────────────────────────────────────

    _renderMarkdown() {
      const html    = renderMarkdown(this._content);
      const safeFrag = sanitizeHTML(html);

      // Clear preview and insert sanitized fragment
      while (this._previewEl.firstChild) {
        this._previewEl.removeChild(this._previewEl.firstChild);
      }
      this._previewEl.appendChild(safeFrag);
    }

    // ─── Toolbar actions — insert markdown at cursor ──────────────────────────

    _insertWrapped(before, after, placeholder) {
      const el    = this._textareaEl;
      const start = el.selectionStart;
      const end   = el.selectionEnd;
      const sel   = el.value.slice(start, end) || placeholder;

      const replacement = before + sel + after;
      el.setRangeText(replacement, start, end, 'select');

      // Select just the user-visible text portion
      el.selectionStart = start + before.length;
      el.selectionEnd   = start + before.length + sel.length;
      el.focus();

      this._content = el.value;
      this._renderMarkdown();
    }

    _insertBold()    { this._insertWrapped('**', '**', 'bold text');   }
    _insertItalic()  { this._insertWrapped('_',  '_',  'italic text'); }
    _insertCode()    { this._insertWrapped('`',  '`',  'code');        }

    _insertHeading() {
      const el    = this._textareaEl;
      const start = el.selectionStart;
      // Find start of line
      const lineStart = el.value.lastIndexOf('\n', start - 1) + 1;
      const lineEnd   = el.value.indexOf('\n', start);
      const end       = lineEnd === -1 ? el.value.length : lineEnd;
      const lineText  = el.value.slice(lineStart, end);

      // Cycle through heading levels 1-6 then remove
      const existingMatch = lineText.match(/^(#{1,6})\s/);
      let newLine;
      if (!existingMatch) {
        newLine = '# ' + lineText;
      } else if (existingMatch[1].length < 6) {
        newLine = existingMatch[1] + '# ' + lineText.slice(existingMatch[1].length + 1);
      } else {
        newLine = lineText.slice(7); // strip "###### "
      }

      el.setRangeText(newLine, lineStart, end, 'end');
      el.focus();
      this._content = el.value;
      this._renderMarkdown();
    }

    _insertLink() {
      const el    = this._textareaEl;
      const start = el.selectionStart;
      const end   = el.selectionEnd;
      const sel   = el.value.slice(start, end) || 'link text';
      const replacement = '[' + sel + '](https://)';
      el.setRangeText(replacement, start, end, 'select');
      // Place cursor inside the URL parentheses
      el.selectionStart = start + sel.length + 3;   // after "["text"]("
      el.selectionEnd   = start + replacement.length - 1; // before ")"
      el.focus();
      this._content = el.value;
      this._renderMarkdown();
    }

    // ─── Public API ───────────────────────────────────────────────────────────

    /** Load content programmatically. */
    setContent(text) {
      this._content = String(text);
      this._textareaEl.value = this._content;
      this._renderMarkdown();
    }

    /** Get current raw markdown content. */
    getContent() {
      return this._content;
    }

    // Expose internals for tests
    get textarea()    { return this._textareaEl; }
    get preview()     { return this._previewEl;  }
    get _renderFn()   { return renderMarkdown;   }
    get _sanitizeFn() { return sanitizeHTML;     }
  }

  // Expose stand-alone utilities for testing
  SigmaNotes.renderMarkdown = renderMarkdown;
  SigmaNotes.sanitizeHTML   = sanitizeHTML;

  return SigmaNotes;
}));
