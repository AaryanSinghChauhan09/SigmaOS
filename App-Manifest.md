# App Manifest Format

Every SigmaOS application must include a `manifest.json` in its root. This file declares the app's identity, entry point, and the capabilities it needs. The SigmaOS extension validates it before installation and enforces it at runtime.

---

## Full Schema

```json
{
  "name": "string (required)",
  "id": "string (required)",
  "version": "string (required)",
  "description": "string (optional)",
  "start_url": "string (required)",
  "icon": "string (optional)",
  "display": "standalone | fullscreen | minimal-ui | browser",
  "background_color": "#rrggbb",
  "theme_color": "#rrggbb",
  "capabilities": ["string"],
  "permissions": {
    "notifications": true | false,
    "autostart": true | false
  },
  "sigma_version": "string (minimum SigmaOS version required)"
}
```

---

## Field Reference

### `name` *(required)*

Human-readable name shown in the launcher, taskbar, and App Store.

```json
"name": "SigmaCode"
```

---

### `id` *(required)*

Reverse-domain unique identifier. Must be globally unique across the App Store.

```json
"id": "dev.sigmaos.sigmacode"
```

---

### `version` *(required)*

Semver string. Used by the App Store to detect available updates.

```json
"version": "2.1.0"
```

---

### `start_url` *(required)*

The URL the app launches to. Can be relative to the manifest's origin or an absolute URL.

```json
"start_url": "/index.html"
```

---

### `capabilities`

Array of capability strings the app requires. The user is shown a permission prompt listing these capabilities at install time. Any capability not listed here is denied at runtime, even if the user tries to grant it.

```json
"capabilities": [
  "process:spawn",
  "fs:/home/user",
  "bin:ffmpeg",
  "ai:complete",
  "clipboard:read",
  "clipboard:write"
]
```

#### Full capability reference

| Capability string | What it grants |
|---|---|
| `process:spawn` | Call `navigator.sigmaos.process.spawn()` |
| `fs:<absolute-path>` | Read and write access to the given path (recursive) |
| `fs:<absolute-path>:ro` | Read-only access to the given path |
| `bin:<name>` | Execute a specific binary by name inside a sandbox |
| `bin:~/.sigmaos/bin/<name>` | Execute a zero-install binary from the user home |
| `net:none` | Declare no network access (enforced with `--unshare-net`) |
| `net:host` | Full outbound network access |
| `net:<host>:<port>` | Access to a specific host:port only |
| `hw:camera` | Access to `/dev/video*` camera devices |
| `hw:audio` | Access to ALSA/PulseAudio sockets |
| `hw:usb` | Access to USB device nodes |
| `clipboard:read` | Read the shared clipboard |
| `clipboard:write` | Write to the shared clipboard |
| `ai:complete` | Call `navigator.sigmaos.ai.complete()` |
| `ai:predict` | Call `navigator.sigmaos.ai.predict()` |
| `pkg:install` | Call `navigator.sigmaos.pkg.ensure()` |
| `window:create` | Call `navigator.sigmaos.window.create()` |
| `notification:show` | Call `navigator.sigmaos.notification.show()` |

---

### `permissions`

Optional object for additional system-level permissions:

```json
"permissions": {
  "notifications": true,
  "autostart": false
}
```

- **`notifications`**: Allow the app to show notifications via the Notification Center.
- **`autostart`**: Start the app automatically when the user logs in.

---

### `sigma_version`

Minimum SigmaOS version required to run the app. If the installed version is older, the App Store shows a warning.

```json
"sigma_version": "15.2"
```

---

## Complete Example

```json
{
  "name": "WordCounter",
  "id": "dev.example.wordcounter",
  "version": "1.0.0",
  "description": "Counts words in text files using native wc",
  "start_url": "/index.html",
  "icon": "/icon-512.png",
  "display": "standalone",
  "background_color": "#1a1a2e",
  "theme_color": "#6c63ff",
  "capabilities": [
    "process:spawn",
    "fs:/home/user:ro",
    "bin:wc"
  ],
  "permissions": {
    "notifications": false,
    "autostart": false
  },
  "sigma_version": "15.0"
}
```

---

## Validation

You can validate a manifest locally with the SigmaOS SDK validator:

```bash
npx sigma-validate manifest.json
```

Output on success:
```
✓ manifest.json is valid
  name: WordCounter
  id: dev.example.wordcounter
  capabilities: 3 declared
```

Output on failure:
```
✗ manifest.json has 2 errors
  [id] Required field missing
  [capabilities[1]] "fs:home/user" — path must be absolute (start with /)
```

---

## Runtime Enforcement

At runtime, the SigmaOS extension checks every `navigator.sigmaos.*` call against the installed manifest. The check is:

1. Is this origin an installed SigmaOS app? (verified against the app registry)
2. Does the app's manifest include the capability required for this API call?
3. Was the capability granted by the user at install time?

If any check fails, the call rejects with `PermissionDeniedError` before reaching any daemon.

---

*See also: [Writing Your First App](Your-First-App) · [API Reference](API-Reference) · [Security Model](Security-Model)*
