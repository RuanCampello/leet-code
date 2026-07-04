def firstUniqChar(s: str) -> int:
    from collections import Counter

    counter = Counter(s)
    for index, char in enumerate(s):
        if counter[char] == 1:
            return index

    return -1


print(firstUniqChar("leetcode"))
print(firstUniqChar("loveleetcode"))
print(firstUniqChar("aabb"))

