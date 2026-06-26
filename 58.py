def lengthOfLastWord(s: str) -> int:
    words = s.strip().split()
    return len(words[-1])


print(lengthOfLastWord("Hello World"))
print(lengthOfLastWord("   fly me   to   the moon  "))
print(lengthOfLastWord("luffy is still joyboy"))
