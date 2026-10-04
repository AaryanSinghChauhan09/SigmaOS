# Shell and Userspace

SigmaOS ships a modern, AI-augmented shell and complete userspace toolkit — all written in Rust (with Zig for low-level primitives). The shell (`sigma-sh`) is POSIX-compatible, blazing fast, and supports natural language command translation powered by the on-device AI runtime.

---

## sigma-sh — The SigmaOS Shell

### Features
- POSIX sh compliance (dash-compatible scripts run unchanged)
- Bash-compatible extensions: arrays, `[[ ]]`, `$((...))`
- Fish-style autosuggestions from history
- Zsh-style tab completion with `fzf`-powered fuzzy search
- **Natural language mode**: `sigma-sh --nl "compress all jpegs in ~/Photos"`
- Async job control: background jobs with live stdout capture
- Built-in `cd` history stack (`cd -2`, `cd -3`)
- Structured output: commands can return typed JSON

### Configuration (`~/.config/sigma/sh/config.toml`)
```toml
[shell]
prompt_style = "powerline"      # minimal | powerline | starship-compat
history_size = 100_000
fuzzy_completion = true
natural_language = true         # requires sigma-ai daemon
syntax_highlighting = true
vi_mode = false                 # false = emacs bindings
```

### Natural Language Examples
```bash
$ sigma-sh> find all files modified today and larger than 1MB
# Translates to: find ~ -mtime 0 -size +1M

$ sigma-sh> kill the process using port 8080
# Translates to: fuser -k 8080/tcp

$ sigma-sh> show disk usage sorted by size
# Translates to: du -sh /* 2>/dev/null | sort -rh | head -20
```

---

## Userspace Toolkit

SigmaOS replaces GNU coreutils, util-linux, and procps with Rust implementations:

### Core Utilities (`src/userspace/`)

| Tool | Replaces | Notes |
|------|---------|-------|
| `sigma-ls` | `ls` / `exa` | Color, git status, icons |
| `sigma-cat` | `cat` / `bat` | Syntax highlighting, line nums |
| `sigma-grep` | `grep` / `ripgrep` | PCRE2, parallel, `-F` literal |
| `sigma-find` | `find` / `fd` | Faster, gitignore-aware |
| `sigma-sed` | `sed` | POSIX + extended regex |
| `sigma-awk` | `awk` / `nawk` | Full POSIX awk |
| `sigma-sort` | `sort` | Parallel merge sort |
| `sigma-ps` | `ps` / `htop` | Process tree, cgroup info |
| `sigma-du` | `du` / `dust` | Tree view, color by size |
| `sigma-df` | `df` | Filesystem usage, SMART status |
| `sigma-tar` | `tar` | zstd default compression |
| `sigma-curl` | `curl` | HTTP/3, TLS 1.3, DoH |
| `sigma-ss` | `ss` / `netstat` | Socket statistics |

---

## Terminal Emulator (`sigma-term`)

- GPU-accelerated rendering (Vulkan / Metal / OpenGL)
- VTE-compatible for compatibility with GTK apps expecting libvte
- Ligature fonts (JetBrains Mono, Fira Code, Cascadia)
- Sixel and Kitty graphics protocol (inline images in terminal)
- True color (24-bit) and 256-color
- Configurable in TOML:

```toml
[terminal]
font = "JetBrains Mono"
font_size = 13.0
line_height = 1.2
padding = [8, 8]
opacity = 0.95
cursor_style = "block"    # block | beam | underline
color_scheme = "sigma-dark"
```

---

## Shell Scripting

### Script Compatibility
sigma-sh runs all POSIX shell scripts without modification. Additionally:

```bash
#!/usr/bin/env sigma-sh

# Typed variables
declare -i count=0
declare -a files=()

# Async jobs with structured output
result=$(sigma-job run --json my-heavy-task)
echo $result | sigma-jq '.status'

# AI-assisted error recovery
sigma-ai-recover || { echo "AI could not auto-fix"; exit 1; }
```

### sigma-script
A safer scripting language for SigmaOS-specific features:
- Typed: `str`, `int`, `bool`, `list`, `map`
- No implicit type coercions
- Built-in: `http.get`, `json.parse`, `fs.read`, `proc.run`
- Compiles to native code via Rust backend

---

## Session Management

### sigma-session
- Wayland session manager
- Saves/restores open applications and window positions
- Integrates with compositor for layout persistence
- Per-workspace session profiles

```bash
sigma-session save work-profile
sigma-session restore work-profile
sigma-session list
```

---

## Comparison vs Bash / Zsh / Fish / Nushell

| Feature | Bash | Zsh | Fish | Nushell | **sigma-sh** |
|---------|------|-----|------|---------|-------------|
| POSIX | ✅ | ✅ | ❌ | ❌ | ✅ |
| Autosuggestion | ❌ | Plugin | ✅ | ✅ | ✅ |
| Typed output | ❌ | ❌ | ❌ | ✅ | ✅ |
| Natural language | ❌ | ❌ | ❌ | ❌ | ✅ |
| Memory safe | ❌ | ❌ | ❌ | ✅ | ✅ |
| AI error recovery | ❌ | ❌ | ❌ | ❌ | ✅ |

---

## Source Files

| File | Description |
|------|-------------|
| `src/shell/` | sigma-sh shell engine |
| `src/userspace/` | Core userspace utilities |
| `src/userland/` | User environment management |
| `src/runtime/` | Script runtime |
| `src/lang/` | sigma-script language |

---

## AI Agent Maintenance Instructions

> **For AI agents maintaining this page:**
> - Source: `src/shell/`, `src/userspace/`, `src/lang/`
> - Update utility table when new tools are added to `src/userspace/`
> - Keep natural language example list fresh and accurate
> - Update comparison table when competitors add AI features
