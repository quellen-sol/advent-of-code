package utils

import "advent/types"

func GCD[T types.Number](a, b T) T {
	a, b = abs(a), abs(b)
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

func LCM[T types.Number](a, b T) T {
	if a == 0 || b == 0 {
		return 0
	}
	// Dividing first keeps the intermediate value as small as the result.
	return abs(a / GCD(a, b) * b)
}

func abs[T types.Number](n T) T {
	if n < 0 {
		return -n
	}
	return n
}
