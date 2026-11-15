-- SPDX-License-Identifier: MIT
-- Telescope fuzzy finder & live grep configuration

return {
  "nvim-telescope/telescope.nvim",
  dependencies = { "nvim-lua/plenary.nvim" },
  config = function()
    local telescope = require("telescope")
    telescope.setup({
      defaults = {
        prompt_prefix = " 🔍 ",
        selection_caret = " ➜ ",
        sorting_strategy = "ascending",
        layout_config = {
          horizontal = { prompt_position = "top", preview_width = 0.55 },
        },
      },
    })
  end,
}
