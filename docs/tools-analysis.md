# docs/tools-analysis.md (package managers, web UIs, build tooling)

## Problem Description
Tooling (package managers, web dashboards, build scripts) show frequent issues:
- Web dashboards use innerHTML for logs → XSS
- Prototype-polluting functions in JS dependencies → supply-chain risk
- Overwritten properties and global leaks from third-party libs
- Unused loop variables and trailing args causing lint failures
- Python exception handling which catches BaseException or has empty except: blocks
- Insecure build scripts embedding secrets or keys

## Root Cause Analysis
- Many tools evolved to support many environments; backward compatibility resulted in permissive, brittle code.
- JS and Python ecosystem code tends to be permissive; transitive dependencies may introduce prototype-pollution.
- Build scripts copy secrets into images for convenience.

## Proposed Fix
- **Web UI XSS**:
  Replace innerHTML with textContent or use a robust HTML sanitizer library when markup is allowed.
  Prohibit rendering of arbitrary logs as HTML; provide a “render-safe” viewer that escapes.
- **Prototype pollution & overwritten properties**:
  Use object creation patterns that use Object.create(null) for dictionaries or Map/WeakMap for arbitrary keys.
  In JS, freeze prototype where needed and validate inputs before extending objects.
- **Python exception hygiene**:
  Replace except BaseException with explicit exceptions (Exception subclasses). Remove empty except blocks; log and handle appropriately.
- **Build scripts & secrets**:
  Use ephemeral secrets injected at runtime via secure store (Vault/KMS). Never embed secret into images or git.

## Code Snippet (Nim — sanitizer + safe log rendering server-side)

```nim
# name=docs/examples/nim_safe_log_renderer.nim
import strutils, htmlgen

# Simple escape function for logs - prefer a battle-tested sanitizer in prod
proc escapeHtml*(s: string): string =
  result = s.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;").replace("\"", "&quot;")

# Usage: render a log line as escaped text in HTML
proc renderLogLine*(line: string): string =
  return "<div class=\"log-line\">" & escapeHtml(line) & "</div>"
```

## Code Snippet (JavaScript defensive object set)

```javascript
// name=docs/examples/js_defensive_set.js
function safeSet(obj, key, value) {
  if (key === "__proto__" || key === "constructor" || key === "prototype") {
    throw new Error("unsafe key");
  }
  // prefer Map for arbitrary keys if possible:
  if (!(obj instanceof Map)) {
    obj[key] = value;
  } else {
    obj.set(key, value);
  }
}
```

## Validation Steps
1. Add frontend unit tests and end-to-end tests ensuring log viewer escapes markup.
2. Run npm audit, retire prototype-polluting transitive deps, and add dependency pinning.
3. Add pre-commit hooks rejecting commits containing obvious secrets or keys.
