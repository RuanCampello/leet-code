package main

import (
	"fmt"
	"sort"
)

func rangeSum(nums []int, n int, left int, right int) int {
	var sum_sub_array []int
	mod := int(1e9 + 7)

	// edge cases
	if len(nums) > 1000 {
		return 0
	}

	// index in n
	for i := 0; i < n; i++ {
		sum := 0
		// for every index of n[i], create an array from n[i] to n
		for j := i; j < n; j++ {
			sum += nums[j]
			sum_sub_array = append(sum_sub_array, sum)
		}
	}
	sort.Ints(sum_sub_array)

	sum := 0
	// minus 1 cause the problems uses the L take (starts array from 1)
	for _, num := range sum_sub_array[left-1 : right] {
		sum = (sum + num) % mod
	}
	return sum
}

func main() {
	s := rangeSum([]int{1, 2, 3, 4}, 4, 1, 5)
	fmt.Println(s)
}
