package main

func merge(nums1 []int, m int, nums2 []int, n int) []int {
	var mergedArrays []int

	if m == 0 {
		return nums2
	}
	if n == 0 {
		return nums1
	}

	idx, nextIdx := 0, 0

	for idx < m && nextIdx < n {
		if nums1[idx] <= nums2[nextIdx] {
			mergedArrays = append(mergedArrays, nums1[idx])
			idx++
		} else {
			mergedArrays = append(mergedArrays, nums2[nextIdx])
			nextIdx++
		}
	}

	for idx < m {
		mergedArrays = append(mergedArrays, nums1[idx])
		idx++
	}
	for nextIdx < n {
		mergedArrays = append(mergedArrays, nums2[nextIdx])
		nextIdx++
	}

	return mergedArrays
}

func main() {
	merged := merge([]int{1, 2, 3, 0, 0, 0}, 3, []int{2, 5, 6}, 3)

	for _, num := range merged {
		println(num)
	}
}
