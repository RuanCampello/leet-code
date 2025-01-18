package main

import (
	"slices"
	"strings"
)

func stringMatching(words []string) []string {
	matching := []string{}

	for idx := 0; idx < len(words); idx++ {
		for nextIdx := 0; nextIdx < len(words); nextIdx++ {
			nextWord := words[nextIdx]

			if idx != nextIdx && strings.Contains(words[idx], nextWord) && !slices.Contains(matching, nextWord) {
				matching = append(matching, nextWord)
			}
		}
	}
	return matching
}

func main() {
	words := []string{"ga", "ugao", "dbh", "a"}

	stringMatching(words)
}
