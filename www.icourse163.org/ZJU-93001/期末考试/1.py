# import time
#


def read_input():
    a = input()
    return [int(i) for i in a.split(' ')]


def pre_order_from_post_and_in(post_order, in_order):
    if not len(post_order):
        return

    n = post_order[-1]
    yield n
    split_idx = in_order.index(n)
    # print("SPLITINDEX: %s" % split_idx)
    in_left, in_right = in_order[:split_idx], in_order[split_idx+1:]
    # print("INLEFT: %s" % in_left)
    # print("INRIGHT: %s" % in_right)
    post_left, post_right = post_order[:split_idx], post_order[split_idx:-1]
    # print("POSTLEFT: %s" % post_left)
    # print("POSTRIGHT: %s" % post_right)
    # time.sleep(1)
    yield from pre_order_from_post_and_in(post_left, in_left)
    yield from pre_order_from_post_and_in(post_right, in_right)


if __name__ == '__main__':
    cnt = int(input())
    post_order = read_input()
    in_order = read_input()
    print("Preorder: %s" % ' '.join(str(s) for s in list(pre_order_from_post_and_in(post_order, in_order))))
