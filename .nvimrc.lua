local tmux_term = require("tmux-awesome-manager.src.term")

vim.keymap.set(
	"n",
	"<leader>rr",
	tmux_term.run({
		cmd = "cargo run tui",
		name = "Cargo tui",
		open_as = "window",
	}),
	{}
) -- Send text to a open terminal?
