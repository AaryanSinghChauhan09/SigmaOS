# SigmaOS Package Build System Specification (`sigbuild`)

## 1. Overview

`sigbuild` is the declarative, reproducible build system for building native `.sigpkg` packages from source definitions (`Sigbuildfile` recipes). It provides isolated sandbox container builds, automatic dependency tracking, security hardening checks, and linting.

## 2. Build Pipeline Architecture

```
+-----------------------------------------------------------+
| 1. Read Sigbuildfile Blueprint & Source Verification      |
+-----------------------------+-----------------------------+
                              |
               sigbuild --clean / Bubblewrap Chroot
                              |
+-----------------------------v-----------------------------+
| 2. Isolated Clean Chroot Build Environment                |
|    (Installs build-time dependencies, executes build steps)|
+-----------------------------+-----------------------------+
                              |
                  strip & Security Hardening Scan
                              |
+-----------------------------v-----------------------------+
| 3. ELF Binary Security & Dynamic Dependency Audit        |
+-----------------------------+-----------------------------+
                              |
                 sigkeyring sign / Ed25519
                              |
+-----------------------------v-----------------------------+
| 4. Final Signed .sigpkg Package Output                    |
+-----------------------------------------------------------+
```

## 3. Recipe Format Specification (`Sigbuildfile`)

```bash
# Sigbuildfile - Recipe for Zenith Terminal
pkgname="zenith-terminal"
pkgver="1.2.0"
pkgrel=1
pkgdesc="GPU-accelerated terminal emulator"
arch=('x86_64' 'aarch64' 'riscv64')
url="https://github.com/sigmaos-org/zenith-terminal"
license=('GPL-3.0-or-later')
depends=('zenith-compositor' 'fontconfig')
makedepends=('cargo' 'rustc' 'pkg-config')
source=("https://github.com/sigmaos-org/${pkgname}/archive/v${pkgver}.tar.gz")
sha256sums=('a1b2c3d4e5f67890123456789abcdef0123456789abcdef0123456789abcdef0')

build() {
    cd "${pkgname}-${pkgver}"
    cargo build --release --locked
}

check() {
    cd "${pkgname}-${pkgver}"
    cargo test --release
}

package() {
    cd "${pkgname}-${pkgver}"
    install -Dm755 "target/release/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
    install -Dm644 "LICENSE" "${pkgdir}/usr/share/licenses/${pkgname}/LICENSE"
}
```

## 4. Reproducible Builds & Security Hardening Checks

`sigbuild` enforces strict build environment invariants:
- **Hermetic Build Sandboxing**: Prevents network access during the `build()` and `package()` stages.
- **Reproducible Timestamps**: Overrides `BUILD_DATE` and sets `SOURCE_DATE_EPOCH` to git commit timestamps.
- **Compiler Hardening Enforcement**: Checks compiled ELF binaries for `STACK-PROTECTOR`, `FORTIFY_SOURCE=3`, `PIE` (Position Independent Executable), `RELRO` (Full Read-Only Relocations), and `NX` (No-Execute stack).
