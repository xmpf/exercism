package sieve

func Sieve(limit int) []int {
	var primes map[int]bool
	var ret []int
	for i := 2; i < limit; i += 1 {
		if _, exists := primes[i]; !exists {
			continue
		}

		primes[i] = true
		ret = append(ret, i)

		next := i
		for {
			next += i
			if next >= limit {
				break
			}
			primes[next] = false
		}
	}

	return ret
}
