def isValid(s: str) -> bool:
    open = []
    paren = {"(": ")", "[": "]", "{": "}"}
    end = [")", "]", "}"]

    for c in s:
        if c in paren.keys():
            open.append(c)
        elif open and c in end and paren.get(open[-1]) != c:
            return False
        else:
            open.pop()

    return len(open) == 0


print(isValid("()"))
print(isValid("()[]{}"))
print(isValid("(]"))
