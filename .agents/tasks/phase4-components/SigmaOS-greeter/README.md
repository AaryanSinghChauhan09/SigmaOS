# SigmaOS-greeter

Display manager greeter UI providing the login screen with user selection, password entry, session picker, and branding. Designed to work with sigma-display-manager.

**Upstream inspiration:** https://github.com/Litarvan/lightdm-webkit-theme-litarvan

**Dependencies:** `lightdm`, `webkit2gtk`, `SigmaOS-display-manager`, `SigmaOS-common`

**Build/Install:**
```
cargo build --release
sudo cp target/release/sigma-greeter /usr/lib/sigma/greeter
```
