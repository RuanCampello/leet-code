from typing import List
from itertools import chain


def shuffle(nums: List[int], n: int) -> List[int]:
    return list(chain.from_iterable(zip(nums, nums[n:])))


print(shuffle([1, 2, 3, 4, 4, 3, 2, 1], 4))
print(shuffle([2, 5, 1, 3, 4, 7], 3))
