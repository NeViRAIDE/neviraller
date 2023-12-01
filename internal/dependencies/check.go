// Package dependencies provides functionality to check the availability of system dependencies.
// It primarily includes functions to verify if certain commands are installed on the system,
// and utilizes the simpletable package to display the results in a tabular format.
// This package is intended to be used in the context of the neviraide-install project,
// offering a user-friendly way to ensure that all necessary dependencies are present before installation.
package dependencies

import (
	"fmt"
	"os/exec"

	"github.com/RAprogramm/neviraide-install/internal/utils"
	"github.com/alexeyco/simpletable"
)

// Dep is a struct type (not shown in your code snippet) presumably representing a single dependency.
// It likely includes fields like Name, Command, Exist, and RequiredBy.

// Check verifies the availability of specified dependencies on the system.
// It requests superuser privileges, checks each dependency, and displays the results in a table.
func Check() []Dep {
	// Request superuser privileges for checking dependencies.
	utils.RequestSudo()

	// Get a list of dependencies and their availability status.
	missingDeps := checkCommandsAvailability()

	// Count the number of missing dependencies.
	missingCount := 0
	for _, dep := range missingDeps {
		if !dep.Exist {
			missingCount++
		}
	}

	// Initialize a new table for displaying the results.
	table := simpletable.New()

	// Define the header for the table with column names.
	table.Header = &simpletable.Header{
		Cells: []*simpletable.Cell{
			{Align: simpletable.AlignCenter, Text: "Status"},
			{Align: simpletable.AlignCenter, Text: "Name"},
			{Align: simpletable.AlignCenter, Text: "Command"},
			{Align: simpletable.AlignCenter, Text: "RequiredBy"},
		},
	}

	// Iterate over the dependencies and add rows to the table.
	for _, dep := range missingDeps {
		// Set the status icon depending on the availability of the dependency.
		icon := utils.Color("red", "", "✗")
		if dep.Exist {
			icon = utils.Color("green", "", "✓")
		}
		r := []*simpletable.Cell{
			{Text: icon},
			{Text: dep.Name},
			{Text: dep.Command},
			{Text: dep.RequiredBy},
		}

		table.Body.Cells = append(table.Body.Cells, r)
	}

	// Add a footer to the table indicating the total number of missing dependencies.
	if missingCount > 0 {
		table.Footer = &simpletable.Footer{
			Cells: []*simpletable.Cell{
				{},
				{},
				{},
				{
					Align: simpletable.AlignRight,
					Text: utils.Color(
						"red",
						"",
						"%d out of %d is missing",
						missingCount,
						len(Dependencies),
					),
				},
			},
		}
	} else {
		table.Footer = &simpletable.Footer{
			Cells: []*simpletable.Cell{
				{},
				{},
				{},
				{Align: simpletable.AlignRight, Text: utils.Color("green", "bold", "All dependencies are present")},
			},
		}
	}

	// Set the style of the table and print it.
	table.SetStyle(simpletable.StyleCompactLite)
	fmt.Printf("%s\n\n", table.String())

	// Return the list of dependencies and their status.
	return missingDeps
}

// checkCommandsAvailability checks if each dependency in the Dependencies list is available on the system.
// It uses the 'which' command to verify the existence of each dependency command.
func checkCommandsAvailability() []Dep {
	for i := range Dependencies {
		// Execute the 'which' command for each dependency.
		cmd := exec.Command("which", Dependencies[i].Command)
		if err := cmd.Run(); err != nil {
			// Mark the dependency as missing if the command fails.
			Dependencies[i].Exist = false
		} else {
			// Mark the dependency as existing if the command succeeds.
			Dependencies[i].Exist = true
		}
	}
	// Return the updated list of dependencies with their existence status.
	return Dependencies
}
