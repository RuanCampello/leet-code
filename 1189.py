def maxNumberOfBalloons(text: str) -> int:
    from collections import Counter

    frequencies = Counter(text)

    l = (frequencies.get("l") or 0) // 2
    a = frequencies.get("a") or 0
    o = (frequencies.get("o") or 0) // 2
    n = frequencies.get("n") or 0
    b = frequencies.get("b") or 0

    return min(l, a, o, n, b)


print(maxNumberOfBalloons("nlaebolko"))
print(maxNumberOfBalloons("loonbalxballpoon"))
print(maxNumberOfBalloons("leetcode"))
print(maxNumberOfBalloons("looalxallpooalloobbnnn"))
