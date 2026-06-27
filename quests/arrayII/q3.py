from typing import List


def findDisappearedNumbers(nums: List[int]) -> List[int]:
    correct = list(range(1, len(nums) + 1))
    diff = list(set(correct) ^ set(nums))
    return diff


print(findDisappearedNumbers([4, 3, 2, 7, 8, 2, 3, 1]))
print(findDisappearedNumbers([1, 1]))
