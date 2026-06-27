from typing import List


def evalRPN(tokens: List[str]) -> int:
    import operator

    stack = []
    operators = {
        "+": operator.add,
        "-": operator.sub,
        "*": operator.mul,
        "/": lambda left, right: int(left / right),
    }

    for token in tokens:
        if token in operators:
            right = stack.pop()
            left = stack.pop()
            stack.append(operators[token](left, right))

            # match token:
            #     case "+":
            #         stack.append(left + right)
            #     case "-":
            #         stack.append(left - right)
            #     case "*":
            #         stack.append(left * right)
            #     case "/":
            #         stack.append(int(left / right))
            #     case _:
            #         return -1
        else:
            stack.append(int(token))

    return stack.pop()


print(evalRPN(["2", "1", "+", "3", "*"]))
print(evalRPN(["4", "13", "5", "/", "+"]))
print(evalRPN(["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]))
