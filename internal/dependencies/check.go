package dependencies

import (
	"fmt"
	"os/exec"

	"github.com/RAprogramm/neviraide-install/internal/utils"
	"github.com/alexeyco/simpletable"
)

func Check() []Dep {
	utils.RequestSudo()

	missingDeps := checkCommandsAvailability()

	missingCount := 0
	for _, dep := range missingDeps {
		if !dep.Exist {
			missingCount++
		}
	}

	table := simpletable.New()

	table.Header = &simpletable.Header{
		Cells: []*simpletable.Cell{
			{Align: simpletable.AlignCenter, Text: "Status"},
			{Align: simpletable.AlignCenter, Text: "Name"},
			{Align: simpletable.AlignCenter, Text: "Command"},
			{Align: simpletable.AlignCenter, Text: "RequiredBy"},
		},
	}

	for _, dep := range missingDeps {
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

	if missingCount > 0 {
		table.Footer = &simpletable.Footer{
			Cells: []*simpletable.Cell{
				{},
				{},
				{},
				{Align: simpletable.AlignRight, Text: utils.Color("red", "", "%d out of %d is missing", missingCount, len(Dependencies))},
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

	table.SetStyle(simpletable.StyleCompactLite)
	fmt.Printf("%s\n\n", table.String())

	return missingDeps
}

func checkCommandsAvailability() []Dep {
	for i := range Dependencies {
		cmd := exec.Command("which", Dependencies[i].Command)
		if err := cmd.Run(); err != nil {
			Dependencies[i].Exist = false
		} else {
			Dependencies[i].Exist = true
		}
	}
	return Dependencies
}
