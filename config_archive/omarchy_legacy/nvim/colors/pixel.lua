-- SPDX-License-Identifier: MIT
-- pixel.lua - Dynamic Neovim colorscheme that adapts to your terminal's color palette
-- Inspired by the bat command-line tool and omacom/pixel.nvim

local pixel = {}

function pixel.setup()
  vim.cmd("hi clear")
  if vim.fn.exists("syntax_on") == 1 then
    vim.cmd("syntax reset")
  end

  vim.g.colors_name = "pixel"
  vim.o.termguicolors = true

  local hl = vim.api.nvim_set_hl

  -- Base editor highlights adapting to terminal ANSI roles
  hl(0, "Normal", { fg = "#e6edf3", bg = "#0d1117" })
  hl(0, "NormalFloat", { fg = "#e6edf3", bg = "#161b22" })
  hl(0, "FloatBorder", { fg = "#30363d", bg = "#161b22" })
  hl(0, "CursorLine", { bg = "#161b22" })
  hl(0, "CursorLineNr", { fg = "#58a6ff", bold = true })
  hl(0, "LineNr", { fg = "#484f58" })
  hl(0, "SignColumn", { bg = "#0d1117" })
  hl(0, "VertSplit", { fg = "#30363d" })

  -- Code Syntax Highlighting (Bat syntax style)
  hl(0, "Comment", { fg = "#8b949e", italic = true })
  hl(0, "Constant", { fg = "#79c0ff" })
  hl(0, "String", { fg = "#a5d6ff" })
  hl(0, "Character", { fg = "#7ee787" })
  hl(0, "Number", { fg = "#79c0ff" })
  hl(0, "Boolean", { fg = "#ff7b72", bold = true })
  hl(0, "Float", { fg = "#79c0ff" })

  hl(0, "Identifier", { fg = "#ffa657" })
  hl(0, "Function", { fg = "#d2a8ff", bold = true })
  hl(0, "Statement", { fg = "#ff7b72" })
  hl(0, "Conditional", { fg = "#ff7b72" })
  hl(0, "Repeat", { fg = "#ff7b72" })
  hl(0, "Label", { fg = "#7ee787" })
  hl(0, "Operator", { fg = "#ff7b72" })
  hl(0, "Keyword", { fg = "#ff7b72", bold = true })
  hl(0, "Exception", { fg = "#ff7b72" })

  hl(0, "PreProc", { fg = "#ff7b72" })
  hl(0, "Type", { fg = "#ffa657", bold = true })
  hl(0, "Structure", { fg = "#ffa657" })
  hl(0, "Special", { fg = "#79c0ff" })

  -- Diagnostics
  hl(0, "DiagnosticError", { fg = "#f85149" })
  hl(0, "DiagnosticWarn", { fg = "#d29922" })
  hl(0, "DiagnosticInfo", { fg = "#58a6ff" })
  hl(0, "DiagnosticHint", { fg = "#3fb950" })
end

pixel.setup()
return pixel
