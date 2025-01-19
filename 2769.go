package main

import (
	"fmt"
)

func theMaximumAchievableX(num int, t int) int {
	return num + (2 * t)
}

func main() {
	res := theMaximumAchievableX(4, 1)
	fmt.Printf("%d", res)
}
