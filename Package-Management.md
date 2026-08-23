# SigmaOS Package Management

## Universal Package Manager (sigma-pkg)

```
sigma-pkg (Universal Frontend)
  ↓
Native sigma | .deb bridge | .rpm bridge | AUR | Flatpak | AppImage
  ↓
Content-Addressed Package Store (hash-indexed)
```

## sigma-pkg Commands

```bash
sigma-pkg search firefox
sigma-pkg install firefox
sigma-pkg remove firefox
sigma-pkg upgrade         # update all packages
sigma-pkg info firefox
sigma-pkg list --installed
sigma-pkg verify firefox  # integrity check
sigma-pkg rollback        # undo last transaction
sigma-pkg history
sigma-pkg audit           # CVE scan
```

## Package Security

All packages signed with **Dilithium-5** (post-quantum signature).

Verification:
1. Verify Dilithium-5 signature against keyring
2. Verify SHA-512 archive hash
3. Verify individual file hashes
4. Check dependency satisfiability

## AUR Compatibility

```bash
sigma-aur install visual-studio-code-bin
sigma-aur search yay
sigma-aur upgrade
```

Build process: PKGBUILD clone → source download → checksum verify → build → package → install

## Flatpak Integration

```bash
sigma-flatpak install flathub org.mozilla.firefox
sigma-flatpak run org.mozilla.firefox
sigma-flatpak update
```

## AppImage Support

```bash
./application.AppImage           # run directly
sigma-appimage install app.AppImage  # integrate into system
```

## Content-Addressed Store

```
/var/sigma/store/
├── sha512-a1b2c3.../  firefox 128.0
├── sha512-d4e5f6.../  libgtk3 3.24
└── sha512-g7h8i9.../  libglib 2.78
```

Benefits: Multi-version install, atomic rollback, deduplication.

## Declarative Packages (NixOS-style)

```nix
{
  packages = ["firefox" "vim" "git"];
  sigma.enableAUR = true;
  flatpak.packages = ["org.videolan.VLC"];
}
```

```bash
sigma-nix apply /etc/sigma/packages.nix
```
