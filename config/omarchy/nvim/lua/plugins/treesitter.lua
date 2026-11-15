-- SPDX-License-Identifier: MIT
-- Treesitter high-performance syntax parsing for SigmaOS / Omarchy

return {
  "nvim-treesitter/nvim-treesitter",
  build = ":TSUpdate",
  config = function()
    require("nvim-treesitter.configs").setup({
      ensure_installed = { "rust", "python", "lua", "c", "bash", "toml", "json", "html", "markdown" },
      highlight = { enable = true },
      indent = { enable = true },
      incremental_selection = {
        enable = true,
        keymaps = {
          init_selection = "<C-space>",
          node_incremental = "<C-space>",
          scope_incremental = "<nop>",
          node_decremental = "<bs>",
        },
      },
    })
  end,
}
