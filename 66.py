from typing import List


def plusOne(digits: List[int]) -> List[int]:
    for i in range(len(digits) - 1, -1, -1):
        if digits[i] < 9:
            digits[i] += 1
            return digits

        # if it's a 9 it becomes 0 and the loop continues
        # to the next digit
        digits[i] = 0

    # when it finishes it means every 9 -> 0
    # so we just add the carryover
    return [1] + digits


print(plusOne([1, 2, 3]))
print(plusOne([4, 3, 2, 1]))
print(plusOne([9]))
