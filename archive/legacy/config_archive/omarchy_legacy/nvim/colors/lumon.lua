-- SPDX-License-Identifier: MIT
-- Lumon Industries Neovim theme (inspired by omacom/lumon.nvim)
-- Clean, sterile, eerie cyberpunk corporate terminal aesthetic

local lumon = {}

function lumon.setup()
  vim.cmd("hi clear")
  if vim.fn.exists("syntax_on") == 1 then
    vim.cmd("syntax reset")
  end

  vim.g.colors_name = "lumon"
  vim.o.termguicolors = true

  local hl = vim.api.nvim_set_hl
  hl(0, "Normal", { fg = "#d8dee9", bg = "#0f141c" })
  hl(0, "Comment", { fg = "#4c566a", italic = true })
  hl(0, "Constant", { fg = "#88c0d0" })
  hl(0, "String", { fg = "#a3be8c" })
  hl(0, "Identifier", { fg = "#81a1c1" })
  hl(0, "Function", { fg = "#5e81ac", bold = true })
  hl(0, "Statement", { fg = "#b48ead" })
  hl(0, "Type", { fg = "#ebcb8b" })
  hl(0, "LineNr", { fg = "#2e3440" })
  hl(0, "CursorLine", { bg = "#1b222d" })
end

lumon.setup()
return lumon
