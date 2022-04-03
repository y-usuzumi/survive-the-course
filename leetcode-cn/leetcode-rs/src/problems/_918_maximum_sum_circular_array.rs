// https://leetcode-cn.com/problems/maximum-sum-circular-subarray/

/// 本题的多种解法中一定需要用到Kadane算法。该算法用于找到数组中的和为最大的连续子数组。
/// 算法的基本思想为动态规划。
///
/// 设数组为A,其长度为n。
/// 用Max(A, i)代表数组中以i结尾（必须包含A[i]）的最大连续子数组的和，则递推
/// Max(A, i+1)分为以下两种情况：
/// Max(A, i) < 0 -- Max(A, i+1) = A[i+1]
/// Max(A, i) >= 0 -- Max(A, i+1) = Max(A, i) + A[i+1]
/// （其中=0的情况归为任一情况都可以）
/// 直观看来，如果前面若干个数的和为负数，则只选我自己肯定比加上一个负数更大。
/// 
/// 当Max(A, 0)到Max(A, n-1)都算完之后，取其中最大的为最终结果。当然为了优化算法，可以
/// 一边算Max(A, i)，一边更新当前的最大结果。

pub struct Solution;

/// 本解法在Leetcode中被称为Kadane's (Min Variant)。其思路是：
/// 对于循环数组，其子数组分为两种，一种是中间一段，一种是两端两段。
/// 1. 对于中间一段的情况，直接运用Kadane算法即可。
/// 2. 对于两端两段的情况，由于数组和为定值，两端两段和最大也就相当于剩下的中间一段和最小。
/// 由此可将整个数组转换为负数，则求和最小的中间一段的问题即可转换为和最大的中间一段的问题。
/// 于是仍然可以直接套用Kadane算法，最后用整个数组之和减去即可。但此时还有一个问题需要注意，
/// 若中间一段恰好选中了整个数组，则相当于两端没选中任何东西，所以结果不成立。补救办法是运用
/// Kadane算法是分别丢弃最左一位和最右一位，然后取较大者。比如若原数组为[-3, -2, -3]，
/// 中间段Kadane的结果为-2，而若不砍去两端值而直接使用Kadane算法，结果为8,此时数组之与其相减
/// 结果为0,显然错误。
impl Solution {
    pub fn max_subarray_sum_circular(mut nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        // 中间一段的Kadane结果
        let positive_max = Self::kadane(&nums);
        let sum: i32 = nums.iter().sum();
        // 取反
        for num in nums.iter_mut() {
            *num = -*num;
        }
        // 将最左一数舍去
        let negative_max_1 = Self::kadane(&nums[1..]);
        // 将最右一数舍去
        let negative_max_2 = Self::kadane(&nums[..nums.len()-1]);
        // 两端两段转换为中间一段时取反，所以sum减去取反之后的Kadane结果，也就是sum + negative_max
        std::cmp::max(positive_max, 
            std::cmp::max(sum + negative_max_1, sum + negative_max_2))
    }

    fn kadane(slice: &[i32]) -> i32 {
        // Max(A, 0)到Max(A, i)之中的最大值
        let mut curr_max = i32::MIN;
        // Max(A, i)
        let mut curr_sum = 0;
        for num in slice.iter() {
            // 若Max(A, i-1) > 0，则Max(A, i) = Max(A, i-1) + A[i],
            // 否则Max(A, i) = A[i]
            curr_sum = std::cmp::max(curr_sum + num, *num);
            // Max(A, 0)到Max(A, i)中的最大值 =
            // max(Max(A, 0)到Max(A, i-1)中的最大值, Max(A, i))
            curr_max = std::cmp::max(curr_max, curr_sum);
        }
        curr_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_subarray_sum_circular(vec![1,-2,3,-2]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_subarray_sum_circular(vec![5,-3,5]), 10);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::max_subarray_sum_circular(vec![-3,-2,-3]), -2);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::max_subarray_sum_circular(vec![-2]), -2);
    }
}