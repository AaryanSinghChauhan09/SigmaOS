# 📦 Package Management & Arch Parity

`sigpkg` is the primary, zero-dependency package manager for SigmaOS. Designed with **Arch Linux (`pacman`/ALPM) and AUR clean-room parity**, `sigpkg` also incorporates translation adapters for Debian (`.deb`), Fedora (`.rpm`), Gentoo (`ebuild`), and Alpine (`.apk`).

***

## 🏗️ Architecture

*   **Content-Addressed Store (CAS):** Packages are stored by content hash in `/sigma/store/`, guaranteeing $O(1)$ deduplication and immutability.
*   **SAT-Solver Dependency Engine:** Uses boolean satisfiability (SAT) solving for exact dependency graph resolution without circular locks.
*   **Atomic Transactional Rollbacks:** All package transactions log pre- and post-snapshots (`SnapperTransactionGuard`), enabling instant single-command rollbacks.
*   **Dual-Layer Cryptographic Attestation:** All binary packages and build recipes are verified using classical GPG keys and **Dilithium-5 post-quantum signatures**.

***

## 💻 Command Reference (`pacman` Parity)

| Arch (`pacman`) Command | `sigpkg` Equivalent | Description |
| :--- | :--- | :--- |
| `pacman -S <pkg>` | `sigpkg install <pkg>` | Install package from repository |
| `pacman -Syu` | `sigpkg update` | Refresh repositories and upgrade all packages |
| `pacman -R <pkg>` | `sigpkg remove <pkg>` | Remove installed package |
| `pacman -Ss <query>` | `sigpkg search <query>` | Search packages across repositories |
| `pacman -Qi <pkg>` | `sigpkg info <pkg>` | Query installed package metadata |
| `pacman -U <file>` | `sigpkg install-file <file>` | Install local binary package file |

***

## 🔨 Arch PKGBUILD & AUR Parity (`ArchRecipeSandboxCompiler`)

SigmaOS parses standard Arch Linux `PKGBUILD` scripts and compiles them inside an isolated, unprivileged chroot sandbox (`ArchRecipeSandboxCompiler`).

### Example Sovereign Recipe (`spkg.recipe`)

```sh
# Sovereign Recipe (Arch PKGBUILD Parity)
pkgname=ripgrep
pkgver=13.0.0
pkgrel=1
arch=('x86_64' 'aarch64' 'riscv64')
depends=('glibc')
makedepends=('cargo')
source=("https://github.com/BurntSushi/ripgrep/archive/${pkgver}.tar.gz")
sha256sums=('SKIP')

build() {
    cargo build --release --locked
}

package() {
    install -Dm755 "target/release/rg" "${pkgdir}/usr/bin/rg"
}
```

***

## ⚡ Advanced Capabilities

### 1. Atomic Transaction Rollbacks

Rollback the last package operation or switch to a specific transaction marker:

```bash
sigpkg rollback --last
```

### 2. Gentoo Portage USE Flags

Toggle compile-time feature flags on supported recipes:

```bash
# Enable SSL and disable Wayland for a build recipe
sigpkg build ripgrep --use="+ssl -wayland"
```

### 3. Post-Quantum Signature Verification

Verify package authenticity prior to installation:

```bash
sigpkg verify-signature ripgrep-13.0.0-x86_64.spkg
```
