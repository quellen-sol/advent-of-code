package utils

import "iter"

// StringChunks splits s into pieces of chunkSize characters. The last piece is
// shorter when chunkSize doesn't divide s evenly. Characters are runes, so a
// multi-byte character is never split.
func StringChunks(s string, chunkSize int) iter.Seq[string] {
	if chunkSize < 1 {
		panic("StringChunks: chunkSize must be positive")
	}

	return func(yield func(string) bool) {
		start, count := 0, 0
		for i := range s {
			if count == chunkSize {
				if !yield(s[start:i]) {
					return
				}
				start, count = i, 0
			}
			count++
		}

		if start < len(s) {
			yield(s[start:])
		}
	}
}
