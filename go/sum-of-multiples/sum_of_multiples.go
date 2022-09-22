package summultiples

// func Unique(divisors []int) []int {
// 	keys := make(map[int]bool)
// 	list := []int{}

// 	for _, value := range divisors {
// 		if _, exists := keys[value]; !exists {
// 			keys[value] = true
// 			list = append(list, value)
// 		}
// 	}

// 	return list
// }

func SumMultiples(limit int, divisors ...int) int {

	sum := 0
	multiples := make(map[int]bool)
	for _, divisor := range divisors {

		if divisor == 0 {
			// errors.New("Division by zero!")
			continue
		}

		for multiple := divisor; multiple < limit; multiple += divisor {
			if _, exists := multiples[multiple]; !exists {
				multiples[multiple] = true
				sum += multiple
			}
		}
	}
	return sum
}
