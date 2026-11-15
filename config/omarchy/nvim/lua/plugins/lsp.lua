-- SPDX-License-Identifier: MIT
-- LSP client configuration for SigmaOS / Omarchy
-- Configures rust-analyzer, pyright, lua-language-server, and clangd

return {
  "neovim/nvim-lspconfig",
  config = function()
    local lspconfig = require("lspconfig")

    -- Server configurations with default capabilities
    local servers = { "rust_analyzer", "pyright", "lua_ls", "clangd" }
    for _, lsp in ipairs(servers) do
      if lspconfig[lsp] then
        lspconfig[lsp].setup({
          flags = { debounce_text_changes = 150 },
        })
      end
    end

    -- Diagnostic display format
    vim.diagnostic.config({
      virtual_text = { prefix = "●" },
      signs = true,
      underline = true,
      update_in_insert = false,
    })
  end,
}
