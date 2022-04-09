// https://leetcode-cn.com/problems/maximum-length-of-subarray-with-positive-product/

use std::cmp::max;
use std::mem::swap;

pub struct Solution;

/// 我的解法，很显然和别人相比弱爆了
/// 受152. Maximum Product Subarray影响，最先想到的思路是正向算一遍，反向算一遍。
/// 本题可以分情况讨论:
/// 1. 当数组不含0的情况下：
///   1.1. 若数组中的负数为偶数个（包括0），则必然可选取整个数组。
///   1.2. 若数组中的负数为奇数个，则统计到最后一个负数之前为止。正向统计一遍，反向统计一遍，
///        取两者较大值。
/// 2. 当数组含0的情况下，按0拆为若干子数组，分别按上述方法统计，最后取整体最大值。
/// 实际实现时有若干优化方法，比如无论是否含0,两个方向均仅需统计一次即可，若遇0时刷新临时变量
/// 即可。
impl Solution {
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        // 正向统计
        let max_length_forward = Self::max_length_unidirectional(nums.iter());
        // 反向统计
        let max_length_backward = Self::max_length_unidirectional(nums.iter().rev());
        // 取较大
        max(max_length_backward, max_length_forward)
    }

    fn max_length_unidirectional<'a, T: Iterator<Item = &'a i32>>(nums: T) -> i32 {
        // 当前全局最优值
        let mut max_length = 0;
        // 当前值
        let mut curr_length = 0;
        // 当当前负数为奇数个时，此部分长度可能最终被舍去，故单独存放
        let mut pending_length = 0;
        // 当前负数个数是否为奇数
        let mut is_negs_odd = false;
        for num in nums {
            if *num == 0 {
                // 若遇到0,则刷新临时变量重新统计
                curr_length = 0;
                pending_length = 0;
                is_negs_odd = false;
            } else if *num < 0 {
                if is_negs_odd {
                    // 若当前负数个数为奇数而又遇到了一个负数，则可能被舍去的部分不会舍去，
                    // 而是加到总长度里，同时这个新的负数也要算到长度里
                    curr_length += pending_length + 1;
                    // 可能舍去的部分清零
                    pending_length = 0;
                } else {
                    // 若当前负数个数为偶数，则新的负数加到可能舍去的部分里
                    pending_length += 1;
                }
                // 算上新负数，奇偶性调换
                is_negs_odd = !is_negs_odd;
            } else {
                if is_negs_odd {
                    // 若当前负数个数为奇数而遇到一个正数，则加到可能舍去的部分里
                    pending_length += 1;
                } else {
                    // 若当前负数个数为偶数而遇到一个正数，直接加到局部最大长度里
                    curr_length += 1;
                }
            }
            // 由局部结果更新全局结果
            if curr_length > max_length {
                max_length = curr_length;
            }
        }
        max_length
    }
}

pub struct Solution2;

/// 某评论区大神的解法。其思路是仅遍历一次，同时维护到目前为止乘积分别为正数和负数的最长长度。
/// 每遇到一个正数，则正数最长长度和负数最长长度分别加一，每遇到一个负数，则正数最长长度与
/// 负数最长长度互换并同时加一。每遇到一个零，则正数最长长度和负数最长长度归零重新开始统计。
/// 遍历过程中要随时用正数最长长度更新全局最优值。
impl Solution2 {
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        // 全局最优值
        let mut res = 0;
        // 到目前为止乘积分别为正数和负数的最长长度
        let (mut pos_max, mut neg_max) = (0, 0);

        for num in nums {
            if num == 0 {
                // 遇到零时重新开始统计
                pos_max = 0;
                neg_max = 0;
            } else {
                // 正数和负数最长长度同时加一，除非还没遇到过负数
                pos_max += 1;
                if neg_max > 0 {
                    neg_max += 1;
                }
                if num < 0 {
                    // 遇到负数，除了加一之外还需要互换
                    swap(&mut pos_max, &mut neg_max);
                }
                // 随时更新全局最优值
                res = max(res, pos_max);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tests_solution {
        use super::*;

        #[test]
        fn test_1() {
            assert_eq!(Solution::get_max_len(vec![1, -2, -3, 4]), 4);
        }

        #[test]
        fn test_2() {
            assert_eq!(Solution::get_max_len(vec![0, 1, -2, -3, -4]), 3);
        }

        #[test]
        fn test_3() {
            assert_eq!(Solution::get_max_len(vec![-1, -2, -3, 0, 1]), 2);
        }
    }

    mod tests_solution2 {
        use super::*;

        #[test]
        fn test_1() {
            assert_eq!(Solution2::get_max_len(vec![1, -2, -3, 4]), 4);
        }

        #[test]
        fn test_2() {
            assert_eq!(Solution2::get_max_len(vec![0, 1, -2, -3, -4]), 3);
        }

        #[test]
        fn test_3() {
            assert_eq!(Solution2::get_max_len(vec![-1, -2, -3, 0, 1]), 2);
        }
    }
}
