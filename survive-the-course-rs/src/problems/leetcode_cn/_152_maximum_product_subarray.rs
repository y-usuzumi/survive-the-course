// https://leetcode-cn.com/problems/maximum-product-subarray/submissions/

use std::cmp::{max, min};

pub struct Solution;

/// 注：本解法与Leetcode上的标准解法略有出入。
/// 
/// 本解法的基础是Kadane算法。若直接套用Kadane算法，则无法处理带有负数的情况，如
/// [5, -3, -4],直接套用Kadane算法得出的局部最大值为[5, -3, 12]，而实际最大值
/// 应为全部三数之积。
/// 
/// 问题的关键在于当后面还有负数的情况下，如果我们当前有一个负数的最小值，则乘以后面
/// 的负数有可能得到更大的正数。因此，我们除了要维护一下局部的正数最大值之外，还要
/// 维护一个负数的最小值。设positive_max[i]为包含nums[i]时所得出的最大正数积，
/// negative_min[i]为包含nums[i]时所得出的最小负数积，我们按如下方式转移状态：
/// 
/// 1. 每当乘以一个正数时：
/// positive_max[i] = max(positive_max[i-1] * nums[i], nums[i])
/// negative_max[i] = negative_max[i-1] * nums[i]
/// 2. 每当乘以一个负数时：
/// positive_max[i] = negative_max[i-1] * nums[i]
/// negative_max[i] = min(positive_max[i-1] * nums[i], nums[i])
/// 
/// 当然，为了优化，可以仅使用两个变量维护当前positive_max和negative_max，
/// 而舍弃之前的结果。
/// 在更新positive_max和negative_max时，顺便更新当前的全局最大值，最后返回
/// 全局最大值即可。
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        // 当仅有一个数时，直接返回该数
        if nums.len() == 1 {
            return nums[0];
        }
        // 当长度大于1时，最终结果一定不会小于0
        let (mut curr_neg_min, mut curr_pos_max) = (0, 0);

        if nums[0] > 0 {
            curr_pos_max = nums[0];
        } else {
            curr_neg_min = nums[0];
        }
        let mut max_product = nums[0];

        for idx in 1..nums.len() {
            let num = nums[idx];
            if num > 0 {
                curr_pos_max = max(num, curr_pos_max * num);
                curr_neg_min = curr_neg_min * num;
                
            } else {
                let (pm, nm) = (curr_pos_max, curr_neg_min);
                curr_neg_min = min(num, pm * num);
                curr_pos_max = nm * num;
                
            }
            max_product = max(max_product, curr_pos_max);
        }
        max_product
    }
}

pub struct Solution2;

/// 此解法源自评论区某大神。
/// 此解法的精髓之处在于要想到，对于不包含0的数组，当负数的个数为奇数时
/// （除去数组长度为1的情况），最大值一定为以下两者之一：
/// 
/// * 由左边开始，一直乘到最右一个负数之前（即不包括该负数）
/// * 由右边开始，一直乘到最左一个负数之前（即不包括该负数）
/// 
/// 之所以成立，是因为数组元素为整数类型。
/// 
/// 比如，或给定数组为[2, -3, -5, 8, -2, 3, -7, -1, 9]
/// 由左边开始，取[2, -3, -5, 8, -2, 3, -7]，求其积，得到10080。
/// 由右边开始，取[-5, 8, -2, 3, -7, -1, 9]，求其积，得到15120。
/// 则最终结果为15120。
/// 
/// 当数组中包含0时，以0为界拆成若干子数组，分别按上方做法求解，最后取
/// 全局最大值。
/// 
/// 实际实现时不必拆分数组，也不必判断何时遇到最后一个负数而停止，继续运算
/// 即可，因为当随时更新最大值时，超过最后一个负数时的所有乘积均为负数，
/// 不会更新当前最大值。
impl Solution2 {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max_product = nums[0];
        let mut curr_product = 1;
        for num in nums.iter() {
            curr_product *= num;
            if curr_product > max_product {
                max_product = curr_product;
            }
            if *num == 0 {
                curr_product = 1;
            }
        }

        curr_product = 1;

        for num in nums.iter().rev() {
            curr_product *= num;
            if curr_product > max_product {
                max_product = curr_product;
            }
            if *num == 0 {
                curr_product = 1;
            }
        }

        max_product
    }
}

#[cfg(test)]
mod tests {
    // TODO: Implement the procedural macro for multiple solutions
    use super::*;

    mod tests_solution {
        use super::*;

        #[test]
        fn test_1() {
            assert_eq!(Solution::max_product(vec![2,3,-2,4]), 6);
        }
    
        #[test]
        fn test_2() {
            assert_eq!(Solution::max_product(vec![-2,0,-1]), 0);
        }
    
        #[test]
        fn test_3() {
            assert_eq!(Solution::max_product(vec![7,-2,-4]), 56);
        }
    }

    mod tests_solution2 {
        use super::*;

        #[test]
        fn test_1() {
            assert_eq!(Solution2::max_product(vec![2,3,-2,4]), 6);
        }
    
        #[test]
        fn test_2() {
            assert_eq!(Solution2::max_product(vec![-2,0,-1]), 0);
        }
    
        #[test]
        fn test_3() {
            assert_eq!(Solution2::max_product(vec![7,-2,-4]), 56);
        }
    }
 
}