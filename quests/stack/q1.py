from typing import List


def buildArray(target: List[int], n: int) -> List[str]:
    operations = []
    pointer = 1

    for num in target:
        # this means that we have a mismatch, so for [1, 3], we receive the stream [1, 2, 3]:
        # - when we got to 1, it will just push because 1 < 1 == False (Push)
        # - but when we got to 2 on the stream, we have 3 on the target, then 2 < 3 == True (Push, Push, Pop)
        # so we do a push + pop so that we can discard the 2 as it isn't from the target
        while pointer < num:
            operations.append("Push")
            operations.append("Pop")
            pointer += 1

        operations.append("Push")
        pointer += 1

    return operations


print(buildArray([1, 3], 3))
