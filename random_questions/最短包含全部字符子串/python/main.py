from collections import defaultdict
import sys

def calc(s):
    if not s:
        return [0, 0]
    all_chars = set(s)
    all_chars_len = len(all_chars)
    char_map = defaultdict(int)
    left = right = 0
    length = len(s)
    while left < length and right < length and len(char_map) < all_chars_len:
        c = s[right]
        char_map[c] += 1
        right += 1
    while char_map[s[left]] > 1:
        char_map[s[left]] -= 1
        left += 1

    return [left, right-left]

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Need a string")
        exit(1)

    result = calc(sys.argv[1])
    print("Result: %s" % result)
    left, right = result
    print("Actual substring: %s" % sys.argv[1][left:left+right])
