// Package dependencies manages the dependencies required by the NEVIRALLER application.
// It defines the structure for representing a single dependency and a list of all necessary dependencies.
package dependencies

// Dep represents a single system dependency required by the NEVIRALLER application.
// It includes information about the existence of the dependency, its name, the command
// to check or use it, and the part of NEVIRALLER that requires it.
type Dep struct {
	Exist      bool   // Exist indicates whether the dependency exists on the system.
	Name       string // Name is the human-readable name of the dependency.
	Command    string // Command is the system command associated with the dependency.
	RequiredBy string // RequiredBy indicates which part of NEVIRALLER requires this dependency.
}

// Dependencies is a slice of Dep that lists all the dependencies required for NEVIRALLER.
// Each dependency is initialized with its existence set to false, which will be checked
// at runtime. The list includes essential tools like curl, git, npm, and others,
// each associated with a specific functionality of NEVIRALLER.
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
