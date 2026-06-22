# Package Manager

SigmaOS has two complementary package systems: the native kernel-level `sigma-pkg` and the runtime `pkg.ensure` API.

---

## sigma-pkg (Native .spkg)

The native package manager for OS-level components — drivers, system libraries, and kernel modules.

### Key Properties

- **Reproducible builds**: Content-addressed storage using BLAKE3 hashes.
- **Post-quantum verification**: Every `.spkg` is signed with **Dilithium-5** (NIST PQC Level 5).
- **Zero-dependency resolution**: Written in C++; does not require Python, Ruby, or Node.js.
- **Atomic installation**: All-or-nothing; a failed install never leaves a partial state.

### Package Format (.spkg)

```
package.spkg
├── MANIFEST.sig   — Dilithium-5 signature over the manifest
├── MANIFEST.json  — { name, version, deps, files[] }
└── payload.tar.zst — Zstandard-compressed file tree
```

### Installation Flow

```
sigma-pkg install ffmpeg.spkg
  1. Verify MANIFEST.sig (Dilithium-5, public key pinned at /etc/sigma/pkg.pub)
  2. Check deps against installed package DB
  3. Decompress payload to staging area
  4. Atomic rename staging → /usr/sigma/
  5. Update package DB
```

### Commands

```bash
sigma-pkg install <package.spkg>   # Install a local package
sigma-pkg remove <name>            # Remove a package
sigma-pkg list                     # List installed packages
sigma-pkg verify <name>            # Re-verify signatures
sigma-pkg build <recipe_dir>       # Build a .spkg from a recipe
```

---

## pkg.ensure (Runtime Alpine Packages)

The web-facing package API. Installs any Alpine Linux package into `~/.sigmaos/pkg` using `apk` inside a bubblewrap user namespace. **No root required.**

### How It Works

```
navigator.sigmaos.pkg.ensure(["ffmpeg"])
  ↓
inject.js → native host → sigmad-process /process
  ↓
bwrap --unshare-user --unshare-net ...
  /sbin/apk add --root ~/.sigmaos/pkg --initdb --no-scripts ffmpeg
  ↓
Binaries symlinked to ~/.sigmaos/bin/ffmpeg
  ↓
shell.exec can now use caps: ["bin:~/.sigmaos/bin/ffmpeg"]
```

### Packages You Never Need to Build Into SigmaOS Core

Because of `pkg.ensure`, the SigmaOS core is permanently frozen. These are just `apk add` away:

| Category | Packages |
|----------|---------|
| Media | `ffmpeg`, `imagemagick`, `yt-dlp`, `mpv` |
| Dev | `python3`, `nodejs`, `git`, `gcc`, `rust` |
| Documents | `poppler-utils`, `tesseract-ocr`, `pandoc` |
| Network | `curl`, `wget`, `rsync`, `openssh` |
| Data | `sqlite`, `jq`, `csvkit` |

### Permission

The first call to `pkg.ensure` prompts: **"Allow this site to install software packages?"**

Grant once → the site can install any number of packages silently thereafter.

---

## Package Sources

| Source | Format | Verification |
|--------|--------|-------------|
| Official SigmaOS repo | `.spkg` | Dilithium-5 |
| Alpine Linux repos | `.apk` (via `pkg.ensure`) | Alpine RSA (upstream) |
| Community recipes | `.spkg` build recipes | PR-reviewed |

---

## Writing a sigma-pkg Recipe

Create a directory with:
```
my-package/
├── RECIPE.toml     — metadata (name, version, deps)
├── build.sh        — build script (runs in sandbox)
└── install.sh      — install script
```

`RECIPE.toml`:
```toml
[package]
name    = "mysoftware"
version = "1.0.0"
arch    = ["x86_64", "aarch64"]
deps    = ["libsigma"]

[build]
script  = "build.sh"
```

Then: `sigma-pkg build my-package/` → outputs `mysoftware-1.0.0.spkg`
