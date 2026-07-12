## 2026-07-12 - DOM-based XSS in AI Web Interface
**Vulnerability:** The AI web interface embedded user prompt responses directly into the document using `.innerHTML` without escaping or sanitization, causing DOM-based Cross-Site Scripting (XSS).
**Learning:** Legacy inline HTML generation with string concatenation easily leads to HTML injection when dealing with user-controlled inputs in browser environments.
**Prevention:** Always use safe DOM manipulation methods like `textContent` or `innerText`, or run HTML sanitization on raw inputs before using `.innerHTML`.
