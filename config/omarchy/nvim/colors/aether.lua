-- SPDX-License-Identifier: MIT
-- Aether dynamic colorscheme for Neovim (inspired by omacom/aether.nvim)
-- Adapts highlights from active Wallust / terminal ANSI color definitions

local aether = {}

function aether.setup()
  vim.cmd("hi clear")
  if vim.fn.exists("syntax_on") == 1 then
    vim.cmd("syntax reset")
  end

  vim.g.colors_name = "aether"
  vim.o.termguicolors = true

  -- Highlight groups mapped to standard ANSI / dynamic palettes
  local hl = vim.api.nvim_set_hl
  hl(0, "Normal", { fg = "#cdd6f4", bg = "#1e1e2e" })
  hl(0, "Comment", { fg = "#6c7086", italic = true })
  hl(0, "Constant", { fg = "#fab387" })
  hl(0, "String", { fg = "#a6e3a1" })
  hl(0, "Identifier", { fg = "#89b4fa" })
  hl(0, "Function", { fg = "#89dceb", bold = true })
  hl(0, "Statement", { fg = "#cba6f7" })
  hl(0, "PreProc", { fg = "#f5c2e7" })
  hl(0, "Type", { fg = "#f9e2af" })
  hl(0, "Special", { fg = "#f38ba8" })
  hl(0, "LineNr", { fg = "#45475a" })
  hl(0, "CursorLine", { bg = "#313244" })
  hl(0, "StatusLine", { fg = "#cdd6f4", bg = "#181825" })
end

aether.setup()
return aether
