def solve(values):
    if not values:
        return 0
    positives = [values[0]]
    negatives = [0]
    for idx in range(1, len(values)):
        positives.append(negatives[-1] + values[idx])
        negatives.append(max(positives[-2], negatives[-1]))
    return max(positives[-1], negatives[-1])


if __name__ == '__main__':
    print(solve([1, 2, 3, 1]))
    print(solve([2, 7, 9, 3, 1]))
    print(solve([7, 2, 3, 9, 1]))
