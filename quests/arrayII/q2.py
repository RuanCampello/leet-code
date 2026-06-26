from typing import List


def smallerNumbersThanCurrent(nums: List[int]) -> List[int]:
    from collections import Counter

    smaller = [0] * len(nums)
    count = Counter(nums)

    for idx, num in enumerate(nums):
        for value, quantity in count.items():
            if value < num:
                smaller[idx] += quantity

    return smaller


print(smallerNumbersThanCurrent([8, 1, 2, 2, 3]))
print(smallerNumbersThanCurrent([6, 5, 4, 8]))
print(smallerNumbersThanCurrent([7, 7, 7, 7]))
