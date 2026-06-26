def addBinary(a: str, b: str) -> str:
    return bin(int(a, 2) + int(b, 2))[2:]


print(addBinary("11", "1"))
print(addBinary("1010", "1011"))
