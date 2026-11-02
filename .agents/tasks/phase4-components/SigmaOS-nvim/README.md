# SigmaOS-nvim

SigmaOS-nvim delivers a curated Neovim configuration and plugin suite for SigmaOS, providing a polished developer experience out of the box with LSP integration, treesitter-based syntax highlighting, fuzzy finding, and SigmaOS-themed colour schemes. It targets developers who want a powerful terminal editor that feels native to the SigmaOS desktop without manual configuration overhead.

**Upstream reference projects:** [aether.nvim](https://github.com/ThemerCorp/aether.nvim), [lumon.nvim](https://github.com/lumon-nvim/lumon.nvim)

## Build Instructions

```bash
# Clone the repository
git clone https://github.com/SigmaOS/SigmaOS-nvim.git
cd SigmaOS-nvim

# Run the Neovim setup script
bash src/main.sh
```

Dependencies: `neovim >= 0.9`, `git`, `nodejs` (for LSP servers)
