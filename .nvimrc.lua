local harpoon_tmux = require("harpoon.tmux")
local function run_tui()
	harpoon_tmux.sendCommand(1, "cargo run tui")
	harpoon_tmux.gotoTerminal(1)
end

vim.keymap.set("n", "<leader>r", run_tui, { noremap = true, silent = true })
