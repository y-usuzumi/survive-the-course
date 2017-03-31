from collections import deque


def read_eval_seq(stack_capacity):
    nums = deque(int(x) for x in input().split(' '))
    stack = deque()
    num_seq = deque(range(1, len(nums) + 1))
    while (num_seq or stack) and nums:
        while num_seq and num_seq[0] <= nums[0]:
            stack.append(num_seq.popleft())
            if len(stack) > stack_capacity:
                return False
        try:
            if stack.pop() != nums.popleft():
                return False
        except IndexError:
            return False
    if not num_seq and not nums:
        return True
    return False


if __name__ == '__main__':
    stack_capacity, seq_len, pop_seq_len = (int(x) for x in input().split(' '))
    for i in range(pop_seq_len):
        print("YES" if read_eval_seq(stack_capacity) else "NO")
