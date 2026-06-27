from functools import cache


@cache
def climbStairs(n: int) -> int:
    if n == 1 or n == 2:
        return n

    return climbStairs(n - 1) + climbStairs(n - 2)


print(climbStairs(int(input())))

# 4:
# 1 - 1 - 1 - 1
# 1 - 1 - 2
# 1 - 2 - 1
# 2 - 1 - 1
# 2 - 2
