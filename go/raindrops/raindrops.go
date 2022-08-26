package raindrops

import "fmt"

type Pair struct {
	factor  int
	message string
}

func Convert(number int) string {
	res := ""
	conditions := []Pair{Pair{3, "Pling"}, Pair{5, "Plang"}, Pair{7, "Plong"}}
	found := false

	for _, x := range conditions {
		if number%x.factor == 0 {
			res += x.message
			found = true
		}
	}

	if !found {
		res = fmt.Sprintf("%d", number)
	}

	return res
}
