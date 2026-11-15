-- -----------------------------------------------------
-- Omarchy Linux / SigmaOS Neovim Configuration (init.lua)
-- Fast, clean, modern Lua config tailored for Omarchy
-- -----------------------------------------------------

-- Leader key
vim.g.mapleader = " "
vim.g.maplocalleader = " "

-- Core options
local opt = vim.opt
opt.number = true
opt.relativenumber = true
opt.mouse = "a"
opt.termguicolors = true
opt.wrap = false
opt.scrolloff = 8
opt.sidescrolloff = 8
opt.signcolumn = "yes"
opt.cursorline = true

-- Tabs & Indentation (standard 4 spaces or 2 spaces)
opt.tabstop = 4
opt.softtabstop = 4
opt.shiftwidth = 4
opt.expandtab = true
opt.smartindent = true

-- Search settings
opt.ignorecase = true
opt.smartcase = true
opt.hlsearch = true
opt.incsearch = true

-- Clipboard integration (Wayland wl-copy & X11)
if vim.fn.has("wsl") == 0 then
    opt.clipboard = "unnamedplus"
end

-- Splits open in natural directions
opt.splitbelow = true
opt.splitright = true

-- Performance & swap
opt.swapfile = false
opt.backup = false
opt.undofile = true
opt.updatetime = 250
opt.timeoutlen = 300

-- Keymaps
local map = vim.keymap.set

-- Clear search highlights
map("n", "<Esc>", "<cmd>nohlsearch<CR>", { silent = true })

-- Better window navigation
map("n", "<C-h>", "<C-w>h", { desc = "Navigate window left" })
map("n", "<C-j>", "<C-w>j", { desc = "Navigate window down" })
map("n", "<C-k>", "<C-w>k", { desc = "Navigate window up" })
map("n", "<C-l>", "<C-w>l", { desc = "Navigate window right" })

-- Buffer navigation
map("n", "<Tab>", "<cmd>bnext<CR>", { desc = "Next buffer" })
map("n", "<S-Tab>", "<cmd>bprev<CR>", { desc = "Previous buffer" })
map("n", "<leader>bd", "<cmd>bdelete<CR>", { desc = "Close buffer" })

-- Quick save & quit
map("n", "<leader>w", "<cmd>write<CR>", { desc = "Write buffer" })
map("n", "<leader>q", "<cmd>quit<CR>", { desc = "Quit window" })

-- Diagnostic keymaps
map("n", "[d", vim.diagnostic.goto_prev, { desc = "Go to previous diagnostic" })
map("n", "]d", vim.diagnostic.goto_next, { desc = "Go to next diagnostic" })
map("n", "<leader>e", vim.diagnostic.open_float, { desc = "Show line diagnostic" })
map("n", "<leader>qf", vim.diagnostic.setqflist, { desc = "Diagnostics in quickfix" })

-- Built-in terminal map to exit insert mode
map("t", "<Esc><Esc>", "<C-\\><C-n>", { desc = "Exit terminal insert" })

-- Status message
vim.api.nvim_create_autocmd("VimEnter", {
    callback = function()
        vim.notify("Welcome to SigmaOS Omarchy Neovim", vim.log.levels.INFO)
    end,
})
