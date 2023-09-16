package dependencies

type Dep struct {
	Exist      bool
	Name       string
	Command    string
	RequiredBy string
}

var Dependencies = []Dep{
	{false, "curl", "curl", "Mason.nvim"},
	{false, "fd", "fd", "Telescope.nvim"},
	{false, "git", "git", "Work with git"},
	{false, "npm", "npm", "Mason.nvim"},
	{false, "ripgrep", "rg", "Telescope.nvim"},
	{false, "tar", "tar", "Mason.nvim"},
	{false, "unzip", "unzip", "Mason.nvim"},
	{false, "wget", "wget", "Neovim Nightly updater"},
}
