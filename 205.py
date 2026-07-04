from typing import List


def isIsomorphic(s: str, t: str) -> bool:
    if len(s) != len(t):
        return False

    def indexString(s: str) -> List[int]:
        seen = {}
        index = []
        for i, char in enumerate(s):
            if char not in seen:
                seen[char] = i
            index.append(seen[char])

        return index

    return indexString(s) == indexString(t)


print(isIsomorphic("egg", "add"))
print(isIsomorphic("f11", "b23"))
print(isIsomorphic("paper", "title"))
