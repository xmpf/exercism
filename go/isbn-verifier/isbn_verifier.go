package isbn

import (
	"strings"
)

func IsValidChar(x byte) bool {
	if x != 'X' && !('0' <= x && x <= '9') {
		return false
	}
	return true
}

func IsValidLength(isbn string) bool {
	return len(isbn) == 10
}

func CharToInt(x byte) int {
	if x == 'X' {
		return 10
	}

	return int(x - '0')
}

func IsValidISBN(isbn string) bool {
	isbn = strings.Replace(isbn, "-", "", -1)

	// check length
	if !IsValidLength(isbn) {
		return false
	}

	sum := 0
	for ix, x := range isbn {

		if !IsValidChar(byte(x)) {
			return false
		}

		_x := CharToInt(byte(x))
		sum += _x * (10 - ix)

		if _x == 10 && ix != 9 {
			return false
		}
	}

	return sum%11 == 0
}
