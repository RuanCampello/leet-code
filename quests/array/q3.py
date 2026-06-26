from typing import List


def findMaxConsecutiveOnes(nums: List[int]) -> int:
    maximum = 0
    current = 0

    for num in nums:
        current = current + 1 if num == 1 else 0
        maximum = max(maximum, current)

    return maximum


print(findMaxConsecutiveOnes([1, 1, 0, 1, 1, 1]))
print(findMaxConsecutiveOnes([1, 0, 1, 1, 0, 1]))
