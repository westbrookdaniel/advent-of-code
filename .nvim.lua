-- NOTE: This is some workspace specific neovim config

require("conform").formatters_by_ft.python = {
	"ruff_fix",
	"ruff_organize_imports",
	"ruff_format",
}
