# Daemons

SigmaOS runs three core Go daemons that bridge the kernel C++ world with the Chromium web shell. Each daemon listens on a local HTTP port and is contacted by the Chrome extension's native host.

---

## Daemon Overview

| Daemon | Port | Source | Purpose |
|--------|------|--------|---------|
| `sigmad-process` | `:17382` | `userland/daemons/sigmad-process/main.go` | Bubblewrap shell execution, Alpine package install |
| `sigmad-ai` | `:17383` | `userland/daemons/sigmad-ai/main.go` | Local TinyLlama AI inference |
| `sigmad-sync` | `:17384` | `userland/daemons/sigmad-sync/main.go` | rclone workspace autosync |

---

## sigmad-process

The most complex daemon. Handles all process-related API calls.

### Endpoints

All calls go to `POST /process` with a JSON body containing a `method` field.

#### `shell.exec`

Runs a command inside a bubblewrap namespace. Checks `capabilities.json` before executing.

```json
{
  "method": "shell.exec",
  "cmd": "/usr/bin/ffmpeg",
  "args": ["-i", "pipe:0", "-f", "mp4", "pipe:1"],
  "stdin": "<base64>",
  "caps": ["bin:/usr/bin/ffmpeg", "fs:/tmp"],
  "origin": "https://myapp.com"
}
```

Response:
```json
{ "ok": true, "stdout": "<base64>", "stderr": "<base64>", "code": 0 }
```

#### `pkg.ensure`

Installs Alpine packages into `~/.sigmaos/pkg` using `bwrap + apk`.

```json
{ "method": "pkg.ensure", "packages": ["ffmpeg", "imagemagick"] }
```

#### `pkg.list`

Returns list of installed package names from the APK database.

#### `script.install`

Saves a shell script to `~/.sigmaos/scripts/` with optional autorun metadata.

#### `script.list`

Returns all installed script metadata.

---

## sigmad-ai

Runs TinyLlama 1.1B via `go-llama.cpp` bindings. Model path: `/opt/sigmaos/models/tinyllama-1.1b.gguf`.

### Endpoints

All calls go to `POST /ai`.

#### `summarize`

```json
{ "method": "summarize", "text": "long article...", "maxTokens": 100 }
```

Prepends `"Summarize in 2 sentences:\n"` and passes to model.

#### `complete`

```json
{ "method": "complete", "prompt": "Explain CoW paging:", "maxTokens": 256 }
```

### Model Specs

| Model | Size | Context | Quantization |
|-------|------|---------|-------------|
| TinyLlama 1.1B Chat | 700 MB | 2048 tokens | Q4_K_M |

Download: `wget https://huggingface.co/TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF/resolve/main/tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf -O /opt/sigmaos/models/tinyllama-1.1b.gguf`

---

## sigmad-sync

Uses `rclone` to sync `~/.sigmaos/workspaces/` to a user-configured cloud remote.

### Setup

1. Run `rclone config` in SigmaTerm.
2. Add Google Drive (or any provider) as remote named **`sigmaos`**.
3. Click "Finish Setup" in the welcome wizard.

### Endpoints

#### `POST /sync`

Triggers an immediate `rclone sync`. Auto-syncs every 5 minutes in background.

```json
{ "ok": true, "status": "success" }
```

### Sync Path

- **Local**: `~/.sigmaos/workspaces/`
- **Remote**: `sigmaos:SigmaOS/workspaces/`

---

## Capability Store (`capabilities.json`)

All daemons share the same capability file: `~/.sigmaos/capabilities.json`

```json
[
  { "origin": "https://notes.app", "cap": "fs.read",    "granted": true },
  { "origin": "https://notes.app", "cap": "ai.complete","granted": true },
  { "origin": "https://paint.app", "cap": "process.spawn", "granted": true }
]
```

Manage via **Settings → Capabilities** (`/settings/caps.html`).
