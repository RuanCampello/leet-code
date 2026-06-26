from typing import List


def removeElement(nums: List[int], val: int) -> int:
    k = 0

    for i in range(len(nums)):
        if nums[i] != val:
            nums[k] = nums[i]
            k += 1
    return k


print(removeElement([3, 2, 2, 3], 3))
