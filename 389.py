def findTheDifference(s: str, t: str) -> str:
    ascii = 0

    for char in s + t:
        ascii ^= ord(char)
    return chr(ascii)


print(findTheDifference("a", "aa"))
