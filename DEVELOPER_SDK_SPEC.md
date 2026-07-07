# SigmaOS Developer SDK & Ecosystem

## 1. Zero-Friction Developer Sandboxes
SigmaOS replaces traditional "install everything on the host" workflows with **Ephemeral Dev Sandboxes**. 
- Built on top of the `sigma_sandbox.rs` MicroVM infrastructure.
- Developers type `sigma-dev shell rust` and are instantly dropped into an isolated environment containing the Rust toolchain, Cargo, and a predefined set of dependencies.
- Upon exit, the sandbox diff layer is discarded. The host system remains pristine, entirely avoiding "dependency rot."

## 2. LSP and IDE Integration
- The SDK includes native language server protocols (LSPs) that run transparently inside the dev sandboxes but communicate with the host IDE (VS Code, Zed, Neovim) over a secure local socket.

## 3. Package Management SDK
The `sigpkg` toolkit allows developers to easily package their C/C++, Rust, Python, or Node applications into atomic `.sigpkg` archives for distribution. Standardized templates abstract away the complexity of dependency resolution.
