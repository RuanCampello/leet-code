def mergeAlternately(word1: str, word2: str) -> str:
    merged = []
    counter = 0

    while counter < len(word1) and counter < len(word2):
        merged.append(word1[counter])
        merged.append(word2[counter])
        counter += 1

    merged.append(word1[counter:])
    merged.append(word2[counter:])

    return "".join(merged)


print(mergeAlternately("abc", "pqr"))
print(mergeAlternately("ab", "pqrs"))
print(mergeAlternately("abcd", "pq"))
