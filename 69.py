def mySqrt(x: int) -> int:
    if x < 2:
        return x

    n = x // 2

    while True:
        next = (n + x // n) // 2
        if next >= n:
            return n
        n = next


print(mySqrt(int(input())))
