def gcd(a: int, b: int) -> int:
  while b != 0:
    [a, b] = [b, a % b]
  return a

def gcdOfStrings(str1: str, str2: str) -> str:
  if (str1 + str2 != str2 + str1):
    return ""

  return str1[0:gcd(len(str1), len(str2))]

print(gcdOfStrings("ABCABC", "ABC"))
