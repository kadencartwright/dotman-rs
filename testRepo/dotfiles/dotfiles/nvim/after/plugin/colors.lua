-- Lua
require('onedark').setup {
    style = 'dark'
}
require('onedark').load()

function ApplyColors(color)
	color = color or "onedark"
	vim.cmd.colorscheme(color)
end

ApplyColors()
