package binarysearch

func SearchInts(list []int, key int) int {
	for low, high := 0, len(list) - 1; low <= high; {
		mid := (low + high) >> 1
		switch {
		case key < list[mid]:
			high = mid - 1
		case key > list[mid]:
			low =  mid + 1
		default:
			return mid
		}
	}
	return -1
}