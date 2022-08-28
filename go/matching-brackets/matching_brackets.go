package brackets

var open_brackets = []byte{'(', '{', '[', '<'}
var close_brackets = []byte{')', '}', ']', '>'}

func Contains(haystack []byte, needle byte) bool {
	for _, elem := range haystack {
		if elem == needle {
			return true
		}
	}
	return false
}

func IsBracket(c byte) bool {
	return Contains(open_brackets, byte(c)) ||
		Contains(close_brackets, byte(c))
}

func MatchingBracket(c byte) (byte, bool) {
	brackets := make(map[byte]byte)
	brackets[']'] = '['
	brackets['}'] = '{'
	brackets['>'] = '<'
	brackets[')'] = '('

	if _, exists := brackets[c]; !exists {
		return c, false
	}
	return brackets[c], true
}

func Bracket(input string) bool {
	var stack []byte
	var bracket byte

	for _, c := range input {
		if !IsBracket(byte(c)) {
			continue
		}

		if Contains(open_brackets, byte(c)) {
			stack = append(stack, byte(c))
		}

		if Contains(close_brackets, byte(c)) {
			if len(stack) == 0 {
				return false
			}

			bracket, stack = stack[len(stack)-1], stack[:len(stack)-1]
			if match, _ := MatchingBracket(byte(c)); bracket != match {
				return false
			}
		}
	}
	return len(stack) == 0
}
