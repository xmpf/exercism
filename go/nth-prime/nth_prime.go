package prime

import (
	"errors"
)

func IsPrime(n int) bool {
	switch {
	case n%2 == 0 || n%3 == 0:
		return false
	default:
		for i := 5; i*i <= n; i += 6 {
			if n%i == 0 || n%(i+2) == 0 {
				return false
			}
		}
	}
	return true
}

// Nth returns the nth prime number.
// An error must be returned if the nth prime number can't be calculated ('n' is equal or less than zero)
func Nth(n int) (int, error) {
	if n <= 0 {
		return 0, errors.New("Error")
	} else if n == 1 {
		return 2, nil
	} else if n == 2 {
		return 3, nil
	}

	cnt := 2
	for i := 5; ; i += 2 {
		if IsPrime(i) {
			cnt += 1
			if cnt >= n {
				return i, nil
			}
		}
	}
}
