package anagram

import (
	"fmt"
	"strings"
)

func Counter(str string) map[byte]int {
	counter := make(map[byte]int)
	for _, x := range str {
		counter[byte(x)] += 1
	}
	return counter
}

func IsMapsEqual(x map[byte]int, y map[byte]int) bool {
	return fmt.Sprint(x) == fmt.Sprint(y)
}

func IsAnagram(source string, candidate string) bool {
	source = strings.ToLower(source)
	candidate = strings.ToLower(candidate)

	return (source != candidate) && IsMapsEqual(Counter(source), Counter(candidate))
}

func Detect(subject string, candidates []string) []string {
	list := make([]string, 0)

	for _, candidate := range candidates {
		if IsAnagram(subject, candidate) {
			list = append(list, candidate)
		}
	}

	return list
}
