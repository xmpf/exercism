package letter

import "sync"

// FreqMap records the frequency of each rune in a given text.
type FreqMap map[rune]int

// Frequency counts the frequency of each rune in a given text and returns this
// data as a FreqMap.
func Frequency(s string) FreqMap {
	m := FreqMap{}
	for _, r := range s {
		m[r]++
	}
	return m
}

// ConcurrentFrequency counts the frequency of each rune in the given strings,
// by making use of concurrency.
func ConcurrentFrequency(texts []string) FreqMap {
	res := make(FreqMap)
	ch := make(chan FreqMap, 10)

	var wg sync.WaitGroup
	wg.Add(len(texts))

	for i := 0; i < len(texts); i++ {
		go func(text string) {
			ch <- Frequency(text)
			wg.Done()
		}(texts[i])
	}

	go func() {
		wg.Wait()
		close(ch)
	}()

	for mm := range ch {
		for k, v := range mm {
			res[k] += v
		}
	}

	return res
}
