## 2026-07-12 - DOM-based XSS in AI Web Interface
**Vulnerability:** The AI web interface embedded user prompt responses directly into the document using `.innerHTML` without escaping or sanitization, causing DOM-based Cross-Site Scripting (XSS).
**Learning:** Legacy inline HTML generation with string concatenation easily leads to HTML injection when dealing with user-controlled inputs in browser environments.
**Prevention:** Always use safe DOM manipulation methods like `textContent` or `innerText`, or run HTML sanitization on raw inputs before using `.innerHTML`.

## 2026-07-14 - Input Path Traversal Prevention in Package Name
**Vulnerability:** User-provided inputs (such as package names) passed directly into file resolving or downloading routines could contain directory traversal characters (`../`) or shell metacharacters, potentially leading to unauthorized local file reads, overwrites, or execution.
**Learning:** Raw input must always be vetted at the entry point of the operation rather than relying on sanitizers inside nested utility layers.
**Prevention:** Implement a strict, early whitelist-based input validator (e.g. allowing only ASCII alphanumerics, dashes, and underscores) to enforce tight boundaries before initiating processing.
