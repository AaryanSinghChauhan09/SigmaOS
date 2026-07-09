# SigmaOS Roadmap: Embedded Code Autocomplete
Provide offline code completion in the Zenith text editor using CodeLlama.
## Goals
- CodeLlama-7B-Q4 inference inside the editor process
- Language Server Protocol (LSP) stub for Rust, Python, C
## Key Milestones
- [ ] LSP server stub in Rust
- [ ] GGUF model hot-reload on editor focus
- [ ] Inline ghost-text completion rendering