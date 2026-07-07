# Developer SDK Roadmap

## Packaging Templates
To reduce friction, SigmaOS provides official `sigpkg` templates for C/C++, Rust, Python, Node.js, and containerized apps.

## CI/CD Skeleton
A comprehensive GitHub Actions pipeline that builders can fork to instantly enforce reproducible builds and artifact signing for their own software.

## Ephemeral Dev Sandboxes
- Spin up isolated MicroVMs with pre-installed language toolchains.
- Throw away the environment when done to prevent host OS bloat.
- Native LSP integrations bridge the sandbox environment to the host IDE (e.g., VSCode, Neovim).
