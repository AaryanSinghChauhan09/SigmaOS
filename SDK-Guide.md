# SigmaOS Developer SDK Guide

> Build apps for SigmaOS in any language. Distribute in any format.

---

## Overview

The SigmaOS SDK (`sigma-sdk`) gives developers everything needed to build, test,
and distribute applications for SigmaOS. It supports all major languages through
different binding layers.

---

## Quick Start

### Install the SDK

```bash
# On SigmaOS
sigma-pkg install sigma-sdk

# On Linux (bootstrap mode)
curl -fsSL https://get.sigmaos.app/sdk | sh
```

### Create a New App

```bash
sigma-sdk new my-app --lang rust --format sigpkg
cd my-app
sigma-sdk build
sigma-sdk run
```

---

## Language Bindings

### Rust (Native / Preferred)

```rust
// Cargo.toml
[dependencies]
sigma-sdk = { path = "/usr/share/sigma-sdk/rust" }

// src/main.rs
use sigma_sdk::{App, Window, SigmaResult};

fn main() -> SigmaResult<()> {
    let app = App::new("my-app", "1.0.0");
    let win = Window::new("My SigmaOS App")
        .size(800, 600)
        .show()?;
    app.run()
}
```

### JavaScript / TypeScript (Electron / Browser)

```typescript
// navigator.sigmaos.* API — available in Zenith Desktop and sigma-browser
import type { SigmaOS } from '@sigmaos/types';

declare const sigmaos: SigmaOS;

async function main() {
    const info = await navigator.sigmaos.system.info();
    console.log(`SigmaOS ${info.version} on ${info.arch}`);

    // Install a package
    await navigator.sigmaos.pkg.install('sigma-edit');

    // Read a file via sovereign VFS
    const data = await navigator.sigmaos.fs.readFile('/home/user/notes.md');
}
```

```bash
# Install TypeScript types
npm install @sigmaos/types
# Or for Electron
npm install @sigmaos/electron-sdk
```

### Python

```python
# pip install sigmaos
import sigmaos

info = sigmaos.system.info()
print(f"SigmaOS {info.version}")

# Install a package
sigmaos.pkg.install("sigma-edit")

# Read a file
data = sigmaos.fs.read("/home/user/notes.md")
```

### Java

```java
// Maven: io.sigmaos:sigma-sdk:1.0.0
import io.sigmaos.SigmaOS;
import io.sigmaos.SystemInfo;

public class Main {
    public static void main(String[] args) {
        SystemInfo info = SigmaOS.system().info();
        System.out.println("SigmaOS " + info.getVersion());
    }
}
```

### .NET / C#

```csharp
using SigmaOS.SDK;

var info = await SigmaOS.System.InfoAsync();
Console.WriteLine($"SigmaOS {info.Version}");
await SigmaOS.Pkg.InstallAsync("sigma-edit");
```

---

## navigator.sigmaos.* API Reference

The browser bridge API — available in all Zenith Desktop web apps:

```typescript
navigator.sigmaos.system.info()            // OS version, arch, memory
navigator.sigmaos.system.resources()       // CPU, RAM, disk usage

navigator.sigmaos.pkg.install(name)        // install a package
navigator.sigmaos.pkg.remove(name)         // remove a package
navigator.sigmaos.pkg.list()               // list installed packages
navigator.sigmaos.pkg.search(query)        // search registry

navigator.sigmaos.fs.readFile(path)        // read a file
navigator.sigmaos.fs.writeFile(path, data) // write a file
navigator.sigmaos.fs.listDir(path)         // list directory

navigator.sigmaos.vault.get(key)           // read a secret from sigma-vault
navigator.sigmaos.vault.set(key, value)    // store a secret

navigator.sigmaos.shard.load(name)         // load a shard
navigator.sigmaos.shard.list()             // list loaded shards

navigator.sigmaos.security.pledge(caps)    // restrict capabilities
navigator.sigmaos.security.attestation()   // get PQC attestation token
```

---

## Multi-Format Build

From one `PKGBUILD`, `sigma-sdk` can produce any format:

```bash
sigma-sdk build --target sigpkg    # native .sigpkg
sigma-sdk build --target appimage  # Linux AppImage
sigma-sdk build --target flatpak   # Flatpak bundle
sigma-sdk build --target apk       # Android APK
sigma-sdk build --target ipa       # iOS IPA (requires macOS + Xcode)
sigma-sdk build --target wasm      # WebAssembly bundle
sigma-sdk build --target jar       # Java JAR
sigma-sdk build --target exe       # Windows Portable EXE
sigma-sdk build --target nupkg     # NuGet package
sigma-sdk build --target electron  # Electron installer
sigma-sdk build --target all       # Build all formats
```

---

## App Templates

```bash
sigma-sdk new my-app --template cli          # CLI tool
sigma-sdk new my-app --template gui-rust     # Rust native GUI
sigma-sdk new my-app --template gui-electron # Electron app
sigma-sdk new my-app --template browser-ext  # Browser extension
sigma-sdk new my-app --template shard        # Kernel shard
sigma-sdk new my-app --template driver       # SDF hardware driver
sigma-sdk new my-app --template service      # Background daemon
```

---

## Shard Development

A shard is the SigmaOS unit of capability — independently loadable, testable, replaceable:

```rust
// shards/my-shard/src/lib.rs
use sigma_shard::{Shard, ShardResult, ShardContext};

pub struct MyShard {
    state: u64,
}

impl Shard for MyShard {
    fn name() -> &'static str { "my-shard" }
    fn version() -> &'static str { "1.0.0" }

    fn init(ctx: &ShardContext) -> ShardResult<Self> {
        Ok(Self { state: 0 })
    }

    fn tick(&mut self, ctx: &ShardContext) -> ShardResult<()> {
        self.state += 1;
        Ok(())
    }

    fn shutdown(self) { /* cleanup */ }
}

sigma_shard::register!(MyShard);
```

Build and load:
```bash
sigma-sdk build --target shard
sigma-pkg load my-shard-1.0.0-x86_64.sigpkg
```

---

## CI/CD Integration

```yaml
# .github/workflows/sigma-build.yml
name: Build SigmaOS App
on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: sigmaos/setup-sdk@v1
        with:
          sdk-version: '1.0.0'
      - run: sigma-sdk build --target all
      - run: sigma-pkg verify *.sigpkg
      - uses: actions/upload-artifact@v4
        with:
          name: sigma-packages
          path: dist/*.sigpkg
```

---

## Documentation Hub

Full API reference, tutorials, and examples:

- Online: `https://docs.sigmaos.app` (planned v1.0)
- Local: `sigma-pkg install sigma-docs && sigma-docs serve`
- Wiki: [API-Documentation](API-Documentation.md) · [Developer-Roadmap](Developer-Roadmap.md)

---

*See also: [Component-Integration](Component-Integration.md) · [sigpkg-Spec](sigpkg-Spec.md) · [Your-First-App](Your-First-App.md)*
