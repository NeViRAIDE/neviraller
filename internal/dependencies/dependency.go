package dependencies

type Dep struct {
	Exist      bool
	Name       string
	Command    string
	RequiredBy string
}

var Dependencies = []Dep{
	{false, "git", "git", "Work with git"},
	{false, "ripgrep", "rg", "Telescope.nvim"},
	{false, "fd", "fd", "Telescope.nvim"},
	{false, "unzip", "unzip", "Mason.nvim"},
	{false, "tar", "tar", "Mason.nvim"},
	{false, "wget", "wget", "Mason.nvim"},
	{false, "curl", "curl", "Mason.nvim"},
	{false, "npm", "npm", "Mason.nvim"},
}
