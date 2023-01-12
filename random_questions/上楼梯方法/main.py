def stairs(n, possible_steps):
    memo = [1] + [0] * n
    for i in range(1, n + 1):
        approaches = 0
        for s in possible_steps:
            prev_stairs = i - s
            if prev_stairs < 0:
                break
            approaches += memo[prev_stairs]
        memo[i] = approaches
    return memo[-1]

