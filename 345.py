def reverseVowels(s: str) -> str:
  vowels = ["a", "e", "i", "o", "u"]
  positions = {}
  res = list(" "*len(s))

  for idx, char in enumerate(list(s)):
    if char.lower() in vowels:
      new_position = abs(idx - len(s)) - 2
      positions.update({new_position: char})
    else:
      print("char", char)
      positions.update({idx: char})

  for position in sorted(positions):
    res[position] = positions[position]

  print("positions", positions)
  return "".join(res)

print(reverseVowels("IceCreAm"))
