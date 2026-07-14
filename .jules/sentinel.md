## 2026-07-12 - DOM-based XSS in AI Web Interface

**Vulnerability:** The AI web interface embedded user prompt responses directly into the document using `.innerHTML` without escaping or sanitization, causing DOM-based Cross-Site Scripting (XSS).
**Learning:** Legacy inline HTML generation with string concatenation easily leads to HTML injection when dealing with user-controlled inputs in browser environments.
**Prevention:** Always use safe DOM manipulation methods like `textContent` or `innerText`, or run HTML sanitization on raw inputs before using `.innerHTML`.

## 2026-07-14 - Input Path Traversal Prevention in Package Name

**Vulnerability:** User-provided inputs (such as package names) passed directly into file resolving or downloading routines could contain directory traversal characters (`../`) or shell metacharacters, potentially leading to unauthorized local file reads, overwrites, or execution.
**Learning:** Raw input must always be vetted at the entry point of the operation rather than relying on sanitizers inside nested utility layers.
**Prevention:** Implement a strict, early whitelist-based input validator (e.g. allowing only ASCII alphanumerics, dashes, and underscores) to enforce tight boundaries before initiating processing.

## 2026-07-14 - Sovereign Keyring Ed25519 Verification Against Supply-Chain Attacks

**Vulnerability:** Package managers that rely solely on hash verification (SHA-256) are vulnerable to supply-chain attacks where an attacker compromises the distribution server and replaces both the package binary AND its hash. Without cryptographic signature verification against a trusted public key, users have no way to distinguish legitimate packages from tampered ones.
**Learning:** Hash verification alone proves integrity (the file hasn't been corrupted), but NOT authenticity (the file came from a trusted source). Ed25519 signature verification against a Sovereign Keyring (`/etc/sigma/keys`) provides both, creating a two-layer defense: the hash catches bit-rot, the signature catches impersonation.
**Prevention:** Always require both hash verification AND Ed25519 signature verification for all package transactions. Reject packages with `sig:ed25519:unknown` prefixes in production mode. Never ship a package manager that trusts hashes alone.
