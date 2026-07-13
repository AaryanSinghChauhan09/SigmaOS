# Distro Absorption Pipeline

> **Status**: ACTIVE | **Component**: `sigpkg` | **Phase**: Industrial Evolution

The Distro Absorption Pipeline is the strategic mechanism by which SigmaOS achieves ecosystem parity with established Linux distributions. Rather than reinventing the wheel, SigmaOS *absorbs* packages from existing distributions (Arch, Debian, Alpine) and securely transpiles them into native Sovereign Shards or `sigpkg` packages.

---

## The Absorption Strategy

SigmaOS utilizes an automated translation layer that reads upstream package definitions (e.g., `PKGBUILD` for Arch Linux, `APKBUILD` for Alpine) and converts them into `sigma.recipe` files.

### Pipeline Stages

1. **Ingestion**: The `sigma-absorb` daemon continuously monitors upstream repositories for updates.
2. **Translation**: Upstream build scripts are parsed and translated into `sigma.recipe` format.
   - Example: `pacman -S` commands are translated to `sigpkg install`
   - File system paths are remapped to comply with Sovereign VFS standards.
3. **Sandboxing**: The build process runs inside a Firecracker-backed Sovereign Sandbox to prevent malicious build scripts from compromising the host.
4. **Verification & Signing**: The resulting binaries are scanned, hashed (BLAKE3), and signed using post-quantum cryptography (Dilithium5).
5. **Publishing**: The signed package is published to the `sigma-recipes` repository.

## Supported Upstreams

| Distro | Format | Status | Priority |
|---|---|---|---|
| Arch Linux | PKGBUILD | ✅ Active | High |
| Alpine Linux | APKBUILD | 🔄 In Progress | High (for minimal containers) |
| Debian | .deb / apt | 📋 Planned | Medium |
| Nix | .nix | 📋 Planned | Low (complex evaluation) |

## Example Translation

**Arch PKGBUILD:**
```bash
pkgname=hello
pkgver=2.10
source=("https://ftp.gnu.org/gnu/hello/hello-${pkgver}.tar.gz")
build() {
  cd "$pkgname-$pkgver"
  ./configure --prefix=/usr
  make
}
```

**SigmaOS Recipe:**
```toml
[package]
name = "hello"
version = "2.10"
source = "https://ftp.gnu.org/gnu/hello/hello-${version}.tar.gz"
curation = "absorbed-arch"

[build]
sandbox = true
steps = [
    "cd hello-${version}",
    "./configure --prefix=/sigma/usr",
    "make"
]
```

## Security Guarantees

Absorbed packages are always treated as *third-party* and run with restricted capabilities by default. They are subject to the same `sigma-shield` and Sovereign Sandbox isolation rules as any community-submitted code.
