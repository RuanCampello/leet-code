from typing import List


def findErrorNums(nums: List[int]) -> List[int]:
    from collections import Counter

    correct = list(range(1, len(nums) + 1))
    repeated = [num for num, count in Counter(nums).items() if count > 1]

    diff = list(set(correct) ^ set(nums))
    return repeated + diff


print(findErrorNums([1, 2, 2, 4]))
print(findErrorNums([1, 1]))
print(findErrorNums([2, 2]))
