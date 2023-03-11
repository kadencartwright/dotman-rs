-- examples for your init.lua
-- set termguicolors to enable highlight groups
--[[vim.opt.termguicolors = true


-- OR setup with some options
require("nvim-tree").setup({
  sort_by = "case_sensitive",
  view = {
    width = 30,
    mappings = {
      list = {
        { key = "u", action = "dir_up" },
      },
    },
  },
  renderer = {
    group_empty = true,
  },
  filters = {
    dotfiles = false,
  },
  diagnostics = {
      enable = true,
      show_on_dirs = true,
      show_on_open_dirs = true,
      debounce_delay = 50,
      severity = {
          min = vim.diagnostic.severity.HINT,
          max = vim.diagnostic.severity.ERROR,
      }
  },
  git = {
      enable = true,
      ignore = false
  },
  log = {
      enable=true,
      truncate=true,
      types={
          diagnostics=true,
      },
  },
})

]]
